use ffi::*;
use allegro::ffi::*;
use allegro::{Color, DrawTarget};

use std::cast;
use std::libc::*;
use std::mem;

pub enum FontAlignment
{
	AlignLeft,
	AlignCentre,
	AlignRight
}

pub trait FontDrawing
{
	fn draw_text(&self, font: &Font, color: Color, x: f32, y: f32, align: FontAlignment, text: &str);
}

impl<T: DrawTarget> FontDrawing for T
{
	fn draw_text(&self, font: &Font, color: Color, x: f32, y: f32, align: FontAlignment, text: &str)
	{
		unsafe
		{
			if al_get_target_bitmap() != self.get_target_bitmap()
			{
				al_set_target_bitmap(self.get_target_bitmap())
			}

			let mut info: ALLEGRO_USTR_INFO = mem::uninit();
			let ustr = al_ref_buffer(&mut info, text.as_ptr() as *i8, text.len() as c_int);

			let flags = match align
			{
				AlignLeft => ALLEGRO_ALIGN_LEFT,
				AlignRight => ALLEGRO_ALIGN_RIGHT,
				AlignCentre => ALLEGRO_ALIGN_CENTRE,
			};

			al_draw_ustr(cast::transmute(font.get_font()), *color, x as c_float, y as c_float, flags, ustr);
		}
	}
}

pub struct Font
{
	priv allegro_font: *mut ALLEGRO_FONT
}

impl Font
{
	pub fn get_font(&self) -> *mut ALLEGRO_FONT
	{
		self.allegro_font
	}

	pub fn get_text_width(&self, text: &str) -> i32
	{
		unsafe
		{
			let mut info: ALLEGRO_USTR_INFO = mem::uninit();
			let ustr = al_ref_buffer(&mut info, text.as_ptr() as *i8, text.len() as c_int);
			al_get_ustr_width(self.get_font() as *_, ustr) as i32
		}
	}

	pub fn get_line_height(&self) -> i32
	{
		unsafe
		{
			al_get_font_line_height(self.get_font() as *_) as i32
		}
	}

	pub fn get_ascent(&self) -> i32
	{
		unsafe
		{
			al_get_font_ascent(self.get_font() as *_) as i32
		}
	}

	pub fn get_descent(&self) -> i32
	{
		unsafe
		{
			al_get_font_descent(self.get_font() as *_) as i32
		}
	}

	pub fn get_text_dimensions(&self, text: &str) -> (i32, i32, i32, i32)
	{
		unsafe
		{
			let (mut x, mut y, mut w, mut h) = mem::uninit::<(c_int, c_int, c_int, c_int)>();
			let mut info: ALLEGRO_USTR_INFO = mem::uninit();
			let ustr = al_ref_buffer(&mut info, text.as_ptr() as *i8, text.len() as c_int);
			al_get_ustr_dimensions(self.get_font() as *_, ustr, &mut x, &mut y, &mut w, &mut h);
			(x as i32, y as i32, w as i32, h as i32)
		}
	}
}

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

impl ::addon::FontAddon
{
	pub fn create_builtin_font(&self) -> Option<Font>
	{
		let ret = unsafe
		{
			al_create_builtin_font()
		};
		if ret.is_null()
		{
			None
		}
		else
		{
			Some(Font{ allegro_font: ret })
		}
	}
}
