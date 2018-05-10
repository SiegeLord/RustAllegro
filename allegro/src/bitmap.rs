// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use bitmap_like::BitmapLike;
use core::Core;

use core::{check_bitmap_targeted_elsewhere, update_thread_state};
use ffi::*;
use libc::*;
use std::cell::RefCell;
use std::ffi::CString;
use std::rc::{Rc, Weak};

pub struct Bitmap
{
	allegro_bitmap: *mut ALLEGRO_BITMAP,
	owned: bool,
	sub_bitmaps: Rc<RefCell<Vec<Rc<SubBitmap>>>>,
}

impl Bitmap
{
	pub fn new(_: &Core, w: i32, h: i32) -> Result<Bitmap, ()>
	{
		unsafe {
			let b = al_create_bitmap(w as c_int, h as c_int);
			if b.is_null()
			{
				Err(())
			}
			else
			{
				Ok(Bitmap::wrap(b, true))
			}
		}
	}

	pub fn load(_: &Core, filename: &str) -> Result<Bitmap, ()>
	{
		unsafe {
			let filename = CString::new(filename.as_bytes()).unwrap();
			let b = al_load_bitmap(filename.as_ptr());
			if b.is_null()
			{
				Err(())
			}
			else
			{
				Ok(Bitmap::wrap(b, true))
			}
		}
	}

	/// Wraps an Allegro bitmap.
	pub unsafe fn wrap(bmp: *mut ALLEGRO_BITMAP, own: bool) -> Bitmap
	{
		Bitmap {
			allegro_bitmap: bmp,
			owned: own,
			sub_bitmaps: Rc::new(RefCell::new(vec![])),
		}
	}

	pub unsafe fn clone_and_wrap(bmp: *mut ALLEGRO_BITMAP) -> Result<Bitmap, ()>
	{
		let b = al_clone_bitmap(bmp);
		if b.is_null()
		{
			Err(())
		}
		else
		{
			Ok(Bitmap::wrap(b, true))
		}
	}

	fn has_outstanding_sub_bitmaps(&self) -> bool
	{
		for bmp in &*self.sub_bitmaps.borrow()
		{
			if Rc::strong_count(&bmp) != 1
			{
				return true;
			}
		}
		false
	}

	pub fn maybe_clone(&self) -> Result<Bitmap, ()>
	{
		unsafe { Bitmap::clone_and_wrap(self.allegro_bitmap) }
	}
}

unsafe impl Send for Bitmap {}
unsafe impl Sync for Bitmap {}

impl BitmapLike for Bitmap
{
	fn get_allegro_bitmap(&self) -> *mut ALLEGRO_BITMAP
	{
		self.allegro_bitmap
	}

	fn create_sub_bitmap(&self, x: i32, y: i32, w: i32, h: i32) -> Result<Weak<SubBitmap>, ()>
	{
		SubBitmap::new(self.allegro_bitmap, Rc::downgrade(&self.sub_bitmaps), x, y, w, h)
	}
}

impl Clone for Bitmap
{
	fn clone(&self) -> Bitmap
	{
		self.maybe_clone().unwrap()
	}
}

impl Drop for Bitmap
{
	fn drop(&mut self)
	{
		if self.has_outstanding_sub_bitmaps()
		{
			panic!("Bitmap has outstanding sub-bitmaps.");
		}
		if self.owned
		{
			unsafe {
				check_bitmap_targeted_elsewhere(self.allegro_bitmap, "destroy");
				al_destroy_bitmap(self.allegro_bitmap);
				update_thread_state();
			}
		}
	}
}

pub struct SubBitmap
{
	allegro_bitmap: *mut ALLEGRO_BITMAP,
	siblings: Weak<RefCell<Vec<Rc<SubBitmap>>>>,
}

impl SubBitmap
{
	fn new(
		parent: *mut ALLEGRO_BITMAP, siblings: Weak<RefCell<Vec<Rc<SubBitmap>>>>, x: i32, y: i32, w: i32, h: i32,
	) -> Result<Weak<SubBitmap>, ()>
	{
		let b = unsafe { al_create_sub_bitmap(parent, x as c_int, y as c_int, w as c_int, h as c_int) };
		if b.is_null()
		{
			Err(())
		}
		else
		{
			if let Some(siblings) = siblings.upgrade()
			{
				let sub_bitmap = Rc::new(SubBitmap {
					allegro_bitmap: b,
					siblings: Rc::downgrade(&siblings),
				});
				let ret = Rc::downgrade(&sub_bitmap);
				siblings.borrow_mut().push(sub_bitmap);
				Ok(ret)
			}
			else
			{
				Err(())
			}
		}
	}

	pub fn to_bitmap(&self) -> Result<Bitmap, ()>
	{
		unsafe { Bitmap::clone_and_wrap(self.allegro_bitmap) }
	}
}

unsafe impl Send for SubBitmap {}
unsafe impl Sync for SubBitmap {}

impl BitmapLike for SubBitmap
{
	fn get_allegro_bitmap(&self) -> *mut ALLEGRO_BITMAP
	{
		self.allegro_bitmap
	}

	fn create_sub_bitmap(&self, x: i32, y: i32, w: i32, h: i32) -> Result<Weak<SubBitmap>, ()>
	{
		SubBitmap::new(self.allegro_bitmap, self.siblings.clone(), x, y, w, h)
	}
}

impl Drop for SubBitmap
{
	fn drop(&mut self)
	{
		unsafe {
			check_bitmap_targeted_elsewhere(self.allegro_bitmap, "destroy");
			al_destroy_bitmap(self.allegro_bitmap);
			update_thread_state();
		}
	}
}
