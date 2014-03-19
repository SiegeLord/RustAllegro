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
