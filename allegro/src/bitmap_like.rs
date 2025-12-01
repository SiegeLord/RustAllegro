// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use crate::bitmap::SubBitmap;
use crate::color::{Color, PixelFormat};

use allegro_sys::*;
use allegro_util::Flag;
use libc::*;
use std::marker::PhantomData;
use std::mem;
use std::mem::MaybeUninit;
use std::rc::Weak;

allegro_util::flag_type! {
	BitmapFlags
	{
		MEMORY_BITMAP = ALLEGRO_MEMORY_BITMAP,
		KEEP_BITMAP_FORMAT = ALLEGRO_KEEP_BITMAP_FORMAT,
		FORCE_LOCKING = ALLEGRO_FORCE_LOCKING,
		NO_PRESERVE_TEXTURE = ALLEGRO_NO_PRESERVE_TEXTURE,
		ALPHA_TEST = ALLEGRO_ALPHA_TEST,
		MIN_LINEAR = ALLEGRO_MIN_LINEAR,
		MAG_LINEAR = ALLEGRO_MAG_LINEAR,
		MIPMAP = ALLEGRO_MIPMAP,
		NO_PREMULTIPLIED_ALPHA = ALLEGRO_NO_PREMULTIPLIED_ALPHA,
		VIDEO_BITMAP = ALLEGRO_VIDEO_BITMAP,
		CONVERT_BITMAP = ALLEGRO_CONVERT_BITMAP
	}
}

/**
A trait implemented by types that behave like bitmaps.
*/
pub trait BitmapLike
{
	fn get_allegro_bitmap(&self) -> *mut ALLEGRO_BITMAP;

	/**
	Creates a sub-bitmap of the current bitmap. Note that the parent bitmap
	will panic upon destruction if any strong references to its sub-bitmaps are
	held at that time.
	*/
	fn create_sub_bitmap(&self, x: i32, y: i32, w: i32, h: i32) -> Result<Weak<SubBitmap>, ()>;

	fn get_width(&self) -> i32
	{
		unsafe { al_get_bitmap_width(self.get_allegro_bitmap()) as i32 }
	}

	fn get_height(&self) -> i32
	{
		unsafe { al_get_bitmap_height(self.get_allegro_bitmap()) as i32 }
	}

	fn get_format(&self) -> PixelFormat
	{
		unsafe { mem::transmute(al_get_bitmap_format(self.get_allegro_bitmap()) as u32) }
	}

	fn get_flags(&self) -> BitmapFlags
	{
		unsafe { mem::transmute(al_get_bitmap_flags(self.get_allegro_bitmap()) as u32) }
	}

	fn get_pixel(&self, x: i32, y: i32) -> Color
	{
		unsafe {
			Color::from_allegro_color(al_get_pixel(
				self.get_allegro_bitmap(),
				x as c_int,
				y as c_int,
			))
		}
	}

	fn convert_mask_to_alpha(&self, mask_color: Color)
	{
		unsafe {
			al_convert_mask_to_alpha(self.get_allegro_bitmap(), mask_color.get_allegro_color());
		}
	}

	/**
	Returns if this bitmap is compatible with the current display. This comes
	into play when you have multiple displays in a single thread, and have
	created bitmaps for different displays. A bitmap created for one display
	may or may not be compatible with the other display. If the bitmap is not
	compatible, drawing it will be slow.
	*/
	fn is_compatible_bitmap(&self) -> bool
	{
		unsafe { al_is_compatible_bitmap(self.get_allegro_bitmap()) != 0 }
	}
}

pub trait RowAccess
{
	type DataT<T>;
}

pub struct DirectAccess;

impl RowAccess for DirectAccess
{
	type DataT<T> = T;
}

pub struct MaybeUninitAccess;

impl RowAccess for MaybeUninitAccess
{
	type DataT<T> = MaybeUninit<T>;
}

/**
Locked region of a bitmap.
 */
pub struct LockedRegion<'l, RowAccessT: RowAccess + ?Sized, B: BitmapLike + ?Sized>
{
	region: *mut ALLEGRO_LOCKED_REGION,
	bitmap: &'l mut B,
	width: i32,
	height: i32,
	phantom: PhantomData<RowAccessT>,
}

impl<'l, RowAccessT: RowAccess + ?Sized, B: BitmapLike + ?Sized> LockedRegion<'l, RowAccessT, B>
{
	fn check_lock<D>(&self, y: i32) -> bool
	{
		if y < 0 || y >= self.height()
		{
			return false;
		}
		if self.format().get_size() != std::mem::size_of::<D>()
		{
			return false;
		}
		if self.format().get_size() != std::mem::align_of::<D>()
		{
			return false;
		}
		true
	}

	pub fn row<D>(&self, y: i32) -> Option<&'l [RowAccessT::DataT<D>]>
	{
		if !self.check_lock::<D>(y)
		{
			return None;
		}
		let region = unsafe { &*self.region };
		Some(unsafe {
			std::slice::from_raw_parts(
				region.data.offset((y * region.pitch) as isize) as *mut RowAccessT::DataT<D>,
				self.width() as usize,
			)
		})
	}

	pub fn row_mut<D>(&mut self, y: i32) -> Option<&'l mut [RowAccessT::DataT<D>]>
	{
		if !self.check_lock::<D>(y)
		{
			return None;
		}
		let region = unsafe { &*self.region };
		Some(unsafe {
			std::slice::from_raw_parts_mut(
				region.data.offset((y * region.pitch) as isize) as *mut RowAccessT::DataT<D>,
				self.width() as usize,
			)
		})
	}

	pub fn format(&self) -> PixelFormat
	{
		unsafe { std::mem::transmute((*self.region).format) }
	}

	pub fn width(&self) -> i32
	{
		self.width
	}

	pub fn height(&self) -> i32
	{
		self.height
	}
}

impl<'l, RowAccessT: RowAccess + ?Sized, B: BitmapLike + ?Sized> Drop
	for LockedRegion<'l, RowAccessT, B>
{
	fn drop(&mut self)
	{
		unsafe {
			al_unlock_bitmap(self.bitmap.get_allegro_bitmap());
		}
	}
}

pub trait BitmapLock: BitmapLike
{
	/// Locks the bitmap for reading and optionally writing.
	fn lock(
		&mut self, x: i32, y: i32, width: i32, height: i32, format: PixelFormat, write: bool,
	) -> Result<LockedRegion<DirectAccess, Self>, ()>
	{
		let region = unsafe {
			al_lock_bitmap_region(
				self.get_allegro_bitmap(),
				x as c_int,
				y as c_int,
				width as c_int,
				height as c_int,
				format as c_int,
				if write
				{
					ALLEGRO_LOCK_READWRITE as c_int
				}
				else
				{
					ALLEGRO_LOCK_READONLY as c_int
				},
			)
		};
		if region.is_null()
		{
			Err(())
		}
		else
		{
			Ok(LockedRegion::<DirectAccess, Self> {
				bitmap: self,
				region: region,
				width: width,
				height: height,
				phantom: PhantomData,
			})
		}
	}

	/// Locks the bitmap for writing only.
	fn lock_write_only(
		&mut self, x: i32, y: i32, width: i32, height: i32, format: PixelFormat,
	) -> Result<LockedRegion<MaybeUninitAccess, Self>, ()>
	{
		let region = unsafe {
			al_lock_bitmap_region(
				self.get_allegro_bitmap(),
				x as c_int,
				y as c_int,
				width as c_int,
				height as c_int,
				format as c_int,
				ALLEGRO_LOCK_WRITEONLY as c_int,
			)
		};
		if region.is_null()
		{
			Err(())
		}
		else
		{
			Ok(LockedRegion::<MaybeUninitAccess, Self> {
				bitmap: self,
				region: region,
				width: width,
				height: height,
				phantom: PhantomData,
			})
		}
	}
}
