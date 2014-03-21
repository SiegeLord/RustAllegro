use std::libc::*;
use std::ptr;

use internal::bitmap_like::*;
use internal::core_drawing::*;
use internal::color::*;
use rust_util::Flag;

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
		BitmapOptions{ format: PixelFormatAny, flags: Flag::zero() }
	}
}

pub struct Bitmap
{
	priv allegro_bitmap: *mut ALLEGRO_BITMAP,
	priv is_ref: bool
}

impl Bitmap
{
	fn new(w: i32, h: i32) -> Option<Bitmap>
	{
		Bitmap::new_with_options(w, h, &BitmapOptions::new())
	}

	fn new_with_options(w: i32, h: i32, opt: &BitmapOptions) -> Option<Bitmap>
	{
		let b;
		unsafe
		{
			al_set_new_bitmap_flags(opt.flags.get() as c_int);
			al_set_new_bitmap_format(opt.format as c_int);
			b = al_create_bitmap(w as c_int, h as c_int);
		}
		if b.is_null()
		{
			None
		}
		else
		{
			Some(Bitmap{allegro_bitmap: b, is_ref: false})
		}
	}

	fn load(filename: &str) -> Option<Bitmap>
	{
		Bitmap::load_with_options(filename, &BitmapOptions::new())
	}

	fn load_with_options(filename: &str, opt: &BitmapOptions) -> Option<Bitmap>
	{
		let mut b = ptr::mut_null();
		unsafe
		{
			al_set_new_bitmap_flags(opt.flags.get() as c_int);
			al_set_new_bitmap_format(opt.format as c_int);
			filename.with_c_str(|s|
			{
				b = al_load_bitmap(s);
			});
		}
		if b.is_null()
		{
			None
		}
		else
		{
			Some(Bitmap{allegro_bitmap: b, is_ref: false})
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

impl CoreDrawing for Bitmap {}

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

impl<'m> DrawTarget for SubBitmap<'m>
{
	fn get_target_bitmap(&self) -> *mut ALLEGRO_BITMAP
	{
	self.allegro_bitmap
	}
}

impl<'m> BitmapLike for SubBitmap<'m>
{
	fn get_bitmap(&self) -> *mut ALLEGRO_BITMAP
	{
		self.allegro_bitmap
	}
}

impl<'m> CoreDrawing for SubBitmap<'m> {}

/* Should be safe as Bitmap does not reference a SubBitmap*/
#[unsafe_destructor]
impl<'m> Drop for SubBitmap<'m>
{
	fn drop(&mut self)
	{
		unsafe
		{
			al_destroy_bitmap(self.allegro_bitmap);
		}
	}
}

pub fn new_bitmap_ref(bmp: *mut ALLEGRO_BITMAP) -> Bitmap
{
	Bitmap{ allegro_bitmap: bmp, is_ref: true }
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
			Some(Bitmap{ allegro_bitmap: b, is_ref: false })
		}
	}
}

impl ::internal::core::Core
{
	pub fn create_bitmap(&self, w: i32, h: i32) -> Option<Bitmap>
	{
		Bitmap::new(w, h)
	}

	pub fn create_bitmap_with_options(&self, w: i32, h: i32, opt: &BitmapOptions) -> Option<Bitmap>
	{
		Bitmap::new_with_options(w, h, opt)
	}

	pub fn load_bitmap(&self, filename: &str) -> Option<Bitmap>
	{
		Bitmap::load(filename)
	}

	pub fn load_bitmap_with_options(&self, filename: &str, opt: &BitmapOptions) -> Option<Bitmap>
	{
		Bitmap::load_with_options(filename, opt)
	}
}
