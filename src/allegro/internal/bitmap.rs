use std::libc::*;
use std::rc::Rc;

use internal::bitmap_like::*;
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

pub trait HasBitmapHolder
{
	fn ref_holder(&self) -> Rc<BitmapHolder>;
}

pub struct BitmapHolder
{
	priv allegro_bitmap: *mut ALLEGRO_BITMAP,
	priv is_ref: bool
}

impl BitmapHolder
{
	pub fn new(bmp: *mut ALLEGRO_BITMAP, is_ref: bool) -> BitmapHolder
	{
		BitmapHolder{ allegro_bitmap: bmp, is_ref: is_ref }
	}

	pub fn get_bitmap(&self) -> *mut ALLEGRO_BITMAP
	{
		self.allegro_bitmap
	}
}

impl Drop for BitmapHolder
{
	fn drop(&mut self)
	{
		if !self.is_ref
		{
			unsafe
			{
				use internal::core::dummy_target;

				/* If the current target is a sub-bitmap of this bitmap (or in some unforeseen case this bitmap itself),
				 * then the target becomes invalid. Set it to the dummy target then. */
				let target = al_get_target_bitmap();
				if target == self.allegro_bitmap || target == al_get_parent_bitmap(self.allegro_bitmap)
				{
					al_set_target_bitmap(dummy_target);
				}
				al_destroy_bitmap(self.allegro_bitmap);
			}
		}
	}
}

pub struct Bitmap
{
	priv holder: Rc<BitmapHolder>
}

impl Bitmap
{
	fn new(w: i32, h: i32) -> Option<Bitmap>
	{
		Bitmap::new_with_options(w, h, &BitmapOptions::new())
	}

	fn new_with_options(w: i32, h: i32, opt: &BitmapOptions) -> Option<Bitmap>
	{
		let b =	unsafe
		{
			al_set_new_bitmap_flags(opt.flags.get() as c_int);
			al_set_new_bitmap_format(opt.format as c_int);
			al_create_bitmap(w as c_int, h as c_int)
		};
		if b.is_null()
		{
			None
		}
		else
		{
			Some(Bitmap{ holder: Rc::new(BitmapHolder::new(b, false)) })
		}
	}

	fn load(filename: &str) -> Option<Bitmap>
	{
		Bitmap::load_with_options(filename, &BitmapOptions::new())
	}

	fn load_with_options(filename: &str, opt: &BitmapOptions) -> Option<Bitmap>
	{
		let b = unsafe
		{
			al_set_new_bitmap_flags(opt.flags.get() as c_int);
			al_set_new_bitmap_format(opt.format as c_int);
			filename.with_c_str(|s|
			{
				al_load_bitmap(s)
			})
		};
		if b.is_null()
		{
			None
		}
		else
		{
			Some(Bitmap{ holder: Rc::new(BitmapHolder::new(b, false)) })
		}
	}

	pub fn create_sub_bitmap<'l>(&'l self, x: i32, y: i32, w: i32, h: i32) -> Option<SubBitmap<'l>>
	{
		unsafe
		{
			let b = al_create_sub_bitmap(self.holder.allegro_bitmap, x as c_int, y as c_int, w as c_int, h as c_int);
			if b.is_null()
			{
				None
			}
			else
			{
				Some(SubBitmap{ holder: Rc::new(BitmapHolder::new(b, false)), parent: self })
			}
		}
	}

	pub fn maybe_clone(&self) -> Option<Bitmap>
	{
		clone_bitmap(self.holder.allegro_bitmap)
	}
}

impl BitmapLike for Bitmap
{
	fn get_bitmap(&self) -> *mut ALLEGRO_BITMAP
	{
		self.holder.allegro_bitmap
	}
}

impl HasBitmapHolder for Bitmap
{
	fn ref_holder(&self) -> Rc<BitmapHolder>
	{
		self.holder.clone()
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
	priv holder: Rc<BitmapHolder>,
	priv parent: &'m Bitmap
}

impl<'m> SubBitmap<'m>
{
	pub fn create_sub_bitmap<'l>(&'l self, x: i32, y: i32, w: i32, h: i32) -> Option<SubBitmap<'l>>
	{
		unsafe
		{
			let b = al_create_sub_bitmap(self.holder.allegro_bitmap, x as c_int, y as c_int, w as c_int, h as c_int);
			if b.is_null()
			{
				None
			}
			else
			{
				Some(SubBitmap{ holder: Rc::new(BitmapHolder::new(b, false)), parent: self.parent })
			}
		}
	}

	pub fn get_parent<'l>(&'l self) -> &'l Bitmap
	{
		self.parent
	}

	pub fn to_bitmap(&self) -> Option<Bitmap>
	{
		clone_bitmap(self.holder.allegro_bitmap)
	}
}

impl<'m> BitmapLike for SubBitmap<'m>
{
	fn get_bitmap(&self) -> *mut ALLEGRO_BITMAP
	{
		self.holder.allegro_bitmap
	}
}

impl<'l> HasBitmapHolder for SubBitmap<'l>
{
	fn ref_holder(&self) -> Rc<BitmapHolder>
	{
		self.holder.clone()
	}
}

pub fn new_bitmap_ref(bmp: *mut ALLEGRO_BITMAP) -> Bitmap
{
	Bitmap{ holder: Rc::new(BitmapHolder::new(bmp, true)) }
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
			Some(Bitmap{ holder: Rc::new(BitmapHolder::new(b, false)) })
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
