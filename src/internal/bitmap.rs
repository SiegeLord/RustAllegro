use std::libc::*;
use std::ptr;
use std::num::Zero;

use internal::bitmap_like::*;
use internal::core_drawing::*;
use internal::color::*;

use ffi::*;

pub mod external
{
	pub use super::BitmapOptions;
	pub use super::Bitmap;
	pub use super::SubBitmap;
}

pub struct BitmapOptions
{
	format: PixelFormat,
	flags: BitmapFlags
}

impl BitmapOptions
{
	pub fn new() -> BitmapOptions
	{
		BitmapOptions{ format: PixelFormatAny, flags: Zero::zero() }
	}
}

pub struct Bitmap
{
	priv allegro_bitmap: *mut ALLEGRO_BITMAP,
	priv is_ref: bool
}

impl Bitmap
{
	pub fn create_sub_bitmap<'l>(&'l self, x: i32, y: i32, w: i32, h: i32) -> Option<SubBitmap<'l>>
	{
		unsafe
		{
			let b = al_create_sub_bitmap(self.allegro_bitmap, x as c_int, y as c_int, w as c_int, h as c_int);
			if ptr::is_null(b)
			{
				None
			}
			else
			{
				Some(SubBitmap{allegro_bitmap: b, parent: self})
			}
		}
	}

	pub fn maybe_clone(&self) -> Option<Bitmap>
	{
		clone_bitmap(self.allegro_bitmap)
	}
}

impl Drop for Bitmap
{
	fn drop(&mut self)
	{
		unsafe
		{
			if !self.is_ref
			{
				al_destroy_bitmap(self.allegro_bitmap);
			}
		}
	}
}

impl DrawTarget for Bitmap
{
	fn get_target_bitmap(&self) -> *mut ALLEGRO_BITMAP
	{
		self.allegro_bitmap
	}
}

impl BitmapLike for Bitmap
{
	fn get_bitmap(&self) -> *mut ALLEGRO_BITMAP
	{
		self.allegro_bitmap
	}
}

impl CoreDrawing for Bitmap {}

impl Clone for Bitmap
{
	fn clone(&self) -> Bitmap
	{
		self.maybe_clone().unwrap()
	}
}

pub struct SubBitmap<'self>
{
	priv allegro_bitmap: *mut ALLEGRO_BITMAP,
	priv parent: &'self Bitmap
}

impl<'self> SubBitmap<'self>
{
	pub fn create_sub_bitmap<'l>(&'l self, x: i32, y: i32, w: i32, h: i32) -> Option<SubBitmap<'l>>
	{
		unsafe
		{
			let b = al_create_sub_bitmap(self.allegro_bitmap, x as c_int, y as c_int, w as c_int, h as c_int);
			if ptr::is_null(b)
			{
				None
			}
			else
			{
				Some(SubBitmap{allegro_bitmap: b, parent: self.parent})
			}
		}
	}

	pub fn get_parent<'l>(&'l self) -> &'l Bitmap
	{
		self.parent
	}

	pub fn to_bitmap(&self) -> Option<Bitmap>
	{
		clone_bitmap(self.allegro_bitmap)
	}
}

impl<'self> DrawTarget for SubBitmap<'self>
{
	fn get_target_bitmap(&self) -> *mut ALLEGRO_BITMAP
	{
		self.allegro_bitmap
	}
}

impl<'self> BitmapLike for SubBitmap<'self>
{
	fn get_bitmap(&self) -> *mut ALLEGRO_BITMAP
	{
		self.allegro_bitmap
	}
}

impl<'self> CoreDrawing for SubBitmap<'self> {}

pub fn new_bitmap(w: i32, h: i32) -> Option<Bitmap>
{
	unsafe
	{
		let b = al_create_bitmap(w as c_int, h as c_int);
		if ptr::is_null(b)
		{
			None
		}
		else
		{
			Some(Bitmap{allegro_bitmap: b, is_ref: false})
		}
	}
}

pub fn new_bitmap_with_options(w: i32, h: i32, opt: &BitmapOptions) -> Option<Bitmap>
{
	unsafe
	{
		al_set_new_bitmap_flags(opt.flags.get() as c_int);
		al_set_new_bitmap_format(opt.format as c_int);
	}
	new_bitmap(w, h)
}


pub fn new_bitmap_ref(bmp: *mut ALLEGRO_BITMAP) -> Bitmap
{
	Bitmap{ allegro_bitmap: bmp, is_ref: true }
}

pub fn clone_bitmap(bmp: *mut ALLEGRO_BITMAP) -> Option<Bitmap>
{
	unsafe
	{
		if al_get_target_bitmap() != bmp
		{
			al_set_target_bitmap(bmp)
		}
		let b = al_clone_bitmap(bmp);
		if ptr::is_null(b)
		{
			None
		}
		else
		{
			Some(Bitmap{ allegro_bitmap: b, is_ref: false })
		}
	}
}
