use std::libc::*;
use ptr = std::ptr;

use core_drawing::*;

use ffi::*;

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
}

impl Drop for Bitmap
{
	fn drop(&self)
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

impl CoreDrawing for Bitmap;
