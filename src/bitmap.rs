use std::libc::*;
use std::ptr;
use std::num::Zero;

use bitmap_like::*;
use core_drawing::*;
use color::*;

use ffi::*;

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
	pub fn new(w: int, h: int) -> Option<Bitmap>
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

	pub fn new_with_options(w: int, h: int, opt: &BitmapOptions) -> Option<Bitmap>
	{
		unsafe
		{
			al_set_new_bitmap_flags(opt.flags.get() as c_int);
			al_set_new_bitmap_format(opt.format as c_int);
		}
		Bitmap::new(w, h)
	}

	pub fn create_sub_bitmap<'l>(&'l self, x: int, y: int, w: int, h: int) -> Option<SubBitmap<'l>>
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
		private::clone_bitmap(self.allegro_bitmap)
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
	pub fn create_sub_bitmap<'l>(&'l self, x: int, y: int, w: int, h: int) -> Option<SubBitmap<'l>>
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
		private::clone_bitmap(self.allegro_bitmap)
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

mod private
{
	use std::ptr;
	use ffi::*;

	pub fn bitmap_ref(bmp: *mut ALLEGRO_BITMAP) -> super::Bitmap
	{
		super::Bitmap{ allegro_bitmap: bmp, is_ref: true }
	}

	pub fn clone_bitmap(bmp: *mut ALLEGRO_BITMAP) -> Option<super::Bitmap>
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
				Some(super::Bitmap{ allegro_bitmap: b, is_ref: false })
			}
		}
	}
}
