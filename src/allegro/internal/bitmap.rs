use std::libc::*;
use std::kinds::marker::NoSend;

use internal::bitmap_like::*;
use internal::color::*;

use ffi::*;

pub mod external
{
	pub use super::Bitmap;
	pub use super::SubBitmap;
}

pub struct Bitmap
{
	priv allegro_bitmap: *mut ALLEGRO_BITMAP,
	priv is_ref: bool,
	priv no_send_marker: NoSend
}

impl Bitmap
{
	fn new(w: i32, h: i32) -> Option<Bitmap>
	{
		unsafe
		{
			let b =	al_create_bitmap(w as c_int, h as c_int);
			if b.is_null()
			{
				None
			}
			else
			{
				Some(Bitmap{ allegro_bitmap: b, is_ref: false, no_send_marker: NoSend })
			}
		}
	}

	fn load(filename: &str) -> Option<Bitmap>
	{
		let b = filename.with_c_str(|s|
		{
			unsafe
			{
				al_load_bitmap(s)
			}
		});
		if b.is_null()
		{
			None
		}
		else
		{
			Some(Bitmap{ allegro_bitmap: b, is_ref: false, no_send_marker: NoSend })
		}
	}

	pub fn create_sub_bitmap<'l>(&'l self, x: i32, y: i32, w: i32, h: i32) -> Option<SubBitmap<'l>>
	{
		unsafe
		{
			let b = al_create_sub_bitmap(self.allegro_bitmap, x as c_int, y as c_int, w as c_int, h as c_int);
			if b.is_null()
			{
				None
			}
			else
			{
				Some(SubBitmap{ allegro_bitmap: b, parent: self })
			}
		}
	}

	pub fn maybe_clone(&self) -> Option<Bitmap>
	{
		clone_bitmap(self.allegro_bitmap)
	}
}

impl BitmapLike for Bitmap
{
	fn get_bitmap(&self) -> *mut ALLEGRO_BITMAP
	{
		self.allegro_bitmap
	}
}

impl Clone for Bitmap
{
	fn clone(&self) -> Bitmap
	{
		self.maybe_clone().unwrap()
	}
}

// Not Send just because of the marker
#[unsafe_destructor]
impl Drop for Bitmap
{
	fn drop(&mut self)
	{
		if !self.is_ref
		{
			unsafe
			{
				use internal::core::dummy_target;

				/* If this bitmap is the target or the parent of the target
				 * then the target becomes invalid. Set it to the dummy target. */
				let target = al_get_target_bitmap();
				if target.is_null()
				{
					fail!("Null target bitmap!");
				}
				if target == self.allegro_bitmap || al_get_parent_bitmap(target) == self.allegro_bitmap
				{
					al_set_target_bitmap(dummy_target);
				}
				al_destroy_bitmap(self.allegro_bitmap);
			}
		}
	}
}

pub struct SubBitmap<'m>
{
	priv allegro_bitmap: *mut ALLEGRO_BITMAP,
	priv parent: &'m Bitmap
}

impl<'m> SubBitmap<'m>
{
	pub fn create_sub_bitmap<'l>(&'l self, x: i32, y: i32, w: i32, h: i32) -> Option<SubBitmap<'l>>
	{
		unsafe
		{
			let b = al_create_sub_bitmap(self.allegro_bitmap, x as c_int, y as c_int, w as c_int, h as c_int);
			if b.is_null()
			{
				None
			}
			else
			{
				Some(SubBitmap{ allegro_bitmap: b, parent: self.parent })
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

impl<'m> BitmapLike for SubBitmap<'m>
{
	fn get_bitmap(&self) -> *mut ALLEGRO_BITMAP
	{
		self.allegro_bitmap
	}
}

/* Should be safe, as the parent doesn't reference the child */
#[unsafe_destructor]
impl<'l> Drop for SubBitmap<'l>
{
	fn drop(&mut self)
	{
		unsafe
		{
			use internal::core::dummy_target;

			/* If this bitmap is the target then the target becomes invalid.
			 * Set it to the dummy target. */
			let target = al_get_target_bitmap();
			if target.is_null()
			{
				fail!("Null target bitmap!");
			}
			if target == self.allegro_bitmap
			{
				al_set_target_bitmap(dummy_target);
			}
			al_destroy_bitmap(self.allegro_bitmap);
		}
	}
}

pub fn new_bitmap_ref(bmp: *mut ALLEGRO_BITMAP) -> Bitmap
{
	Bitmap{ allegro_bitmap: bmp, is_ref: true, no_send_marker: NoSend }
}

pub fn clone_bitmap(bmp: *mut ALLEGRO_BITMAP) -> Option<Bitmap>
{
	unsafe
	{
		let b = al_clone_bitmap(bmp);
		if b.is_null()
		{
			None
		}
		else
		{
			Some(Bitmap{ allegro_bitmap: b, is_ref: false, no_send_marker: NoSend })
		}
	}
}

impl ::internal::core::Core
{
	pub fn set_new_bitmap_flags(&self, flags: BitmapFlags)
	{
		unsafe
		{
			al_set_new_bitmap_flags(flags.get() as c_int);
		}
	}

	pub fn set_new_bitmap_format(&self, format: PixelFormat)
	{
		unsafe
		{
			al_set_new_bitmap_format(format as c_int);
		}
	}

	pub fn create_bitmap(&self, w: i32, h: i32) -> Option<Bitmap>
	{
		Bitmap::new(w, h)
	}

	pub fn load_bitmap(&self, filename: &str) -> Option<Bitmap>
	{
		Bitmap::load(filename)
	}
}
