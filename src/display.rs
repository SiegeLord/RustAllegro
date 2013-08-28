use std::libc::*;
use ptr = std::ptr;

use core_drawing::*;

use ffi::*;

struct Display
{
	priv allegro_display: *mut ALLEGRO_DISPLAY
}

impl Display
{
	pub fn new(w: int, h: int) -> Option<Display>
	{
		unsafe
		{
			let d = al_create_display(w as c_int, h as c_int);
			if ptr::is_null(d)
			{
				None
			}
			else
			{
				Some(Display{allegro_display: d})
			}
		}
	}

	pub fn flip(&self)
	{
		unsafe
		{
			al_flip_display();
		}
	}

	pub fn get_width(&self) -> float
	{
		unsafe
		{
			al_get_display_width(self.allegro_display) as float
		}
	}

	pub fn get_height(&self) -> float
	{
		unsafe
		{
			al_get_display_height(self.allegro_display) as float
		}
	}
}

impl Drop for Display
{
	fn drop(&self)
	{
		unsafe
		{
			al_destroy_display(self.allegro_display);
		}
	}
}

impl DrawTarget for Display
{
	fn get_target_bitmap(&self) -> *mut ALLEGRO_BITMAP
	{
		unsafe
		{
			al_get_backbuffer(self.allegro_display)
		}
	}
}

impl CoreDrawing for Display;
