// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use allegro_font_sys::*;
use allegro_sys::*;
use allegro::{Core, Color, Bitmap, BitmapLike};
use addon::FontAddon;

use libc::*;
use std::ffi::CString;
use std::mem;

#[derive(Copy, Clone)]
pub enum FontAlign
{
	Left,
	Centre,
	Right
}

impl FontAlign
{
	fn get_allegro_flags(&self) -> c_int
	{
		match *self
		{
			FontAlign::Left => ALLEGRO_ALIGN_LEFT,
			FontAlign::Right => ALLEGRO_ALIGN_RIGHT,
			FontAlign::Centre => ALLEGRO_ALIGN_CENTRE,
		}
	}
}

pub trait FontDrawing
{
	fn draw_text(&self, font: &Font, color: Color, x: f32, y: f32, align: FontAlign, text: &str);
	fn draw_justified_text(&self, font: &Font, color: Color, x1: f32, x2: f32, y: f32, diff: f32, align: FontAlign, text: &str);
}

impl FontDrawing for Core
{
	fn draw_justified_text(&self, font: &Font, color: Color, x1: f32, x2: f32, y: f32, diff: f32, align: FontAlign, text: &str)
	{
		if text.len() == 0
		{
			return;
		}
		unsafe
		{
			let mut info: ALLEGRO_USTR_INFO = mem::uninitialized();
			let ustr = al_ref_buffer(&mut info, text.as_ptr() as *const i8, text.len() as size_t);

			al_draw_justified_ustr(mem::transmute(font.get_font()), color.0, x1 as c_float,
			                       x2 as c_float, y as c_float, diff as c_float, align.get_allegro_flags(), ustr);
		}
	}

	fn draw_text(&self, font: &Font, color: Color, x: f32, y: f32, align: FontAlign, text: &str)
	{
		if text.len() == 0
		{
			return;
		}
		unsafe
		{
			let mut info: ALLEGRO_USTR_INFO = mem::uninitialized();
			let ustr = al_ref_buffer(&mut info, text.as_ptr() as *const i8, text.len() as size_t);

			al_draw_ustr(mem::transmute(font.get_font()), color.0, x as c_float, y as c_float, align.get_allegro_flags(), ustr);
		}
	}
}

pub struct Font
{
	allegro_font: *mut ALLEGRO_FONT,
}

impl Font
{
	pub fn new_builtin(_: &FontAddon) -> Result<Font, ()>
	{
		unsafe
		{
			Font::wrap_allegro_font(al_create_builtin_font())
		}
	}

	pub fn load_bitmap_font(_: &FontAddon, filename: &str) -> Result<Font, ()>
	{
		unsafe
		{
			let filename = CString::new(filename.as_bytes()).unwrap();
			let font = al_load_bitmap_font(filename.as_ptr());
			Font::wrap_allegro_font(font)
		}
	}

	pub fn grab_from_bitmap(_: &FontAddon, bmp: &Bitmap, ranges: &[(c_int, c_int)]) -> Result<Font, ()>
	{
		unsafe
		{
			let font = al_grab_font_from_bitmap(bmp.get_allegro_bitmap(), (ranges.len() * 2) as c_int, ranges.as_ptr() as *const c_int);
			Font::wrap_allegro_font(font)
		}
	}

	pub unsafe fn wrap_allegro_font(font: *mut ALLEGRO_FONT) -> Result<Font, ()>
	{
		if font.is_null()
		{
			Err(())
		}
		else
		{
			Ok(Font{ allegro_font: font })
		}
	}

	pub fn get_font(&self) -> *mut ALLEGRO_FONT
	{
		self.allegro_font
	}

	pub fn get_text_width(&self, text: &str) -> i32
	{
		unsafe
		{
			let mut info: ALLEGRO_USTR_INFO = mem::uninitialized();
			let ustr = al_ref_buffer(&mut info, text.as_ptr() as *const i8, text.len() as size_t);
			al_get_ustr_width(self.get_font() as *const _, ustr) as i32
		}
	}

	pub fn get_line_height(&self) -> i32
	{
		unsafe
		{
			al_get_font_line_height(self.get_font() as *const _) as i32
		}
	}

	pub fn get_ascent(&self) -> i32
	{
		unsafe
		{
			al_get_font_ascent(self.get_font() as *const _) as i32
		}
	}

	pub fn get_descent(&self) -> i32
	{
		unsafe
		{
			al_get_font_descent(self.get_font() as *const _) as i32
		}
	}

	pub fn get_text_dimensions(&self, text: &str) -> (i32, i32, i32, i32)
	{
		unsafe
		{
			let (mut x, mut y, mut w, mut h): (c_int, c_int, c_int, c_int) = mem::uninitialized();
			let mut info: ALLEGRO_USTR_INFO = mem::uninitialized();
			let ustr = al_ref_buffer(&mut info, text.as_ptr() as *const i8, text.len() as size_t);
			al_get_ustr_dimensions(self.get_font() as *const _, ustr, &mut x, &mut y, &mut w, &mut h);
			(x as i32, y as i32, w as i32, h as i32)
		}
	}
}

// Not Send just because of the marker
impl Drop for Font
{
	fn drop(&mut self)
	{
		unsafe
		{
			al_destroy_font(self.allegro_font);
		}
	}
}
