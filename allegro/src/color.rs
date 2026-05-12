// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use allegro_sys::*;
use libc::*;

#[repr(C)]
#[derive(PartialEq, Copy, Clone, Debug, PartialOrd)]
pub struct Color(ALLEGRO_COLOR);

impl Color
{
	pub fn from_rgb(r: u8, g: u8, b: u8) -> Color
	{
		Color::from_rgba(r, g, b, 255)
	}

	pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Color
	{
		Color::from_rgba_f(
			r as f32 / 255.0,
			g as f32 / 255.0,
			b as f32 / 255.0,
			a as f32 / 255.0,
		)
	}

	pub const fn from_rgb_f(r: f32, g: f32, b: f32) -> Color
	{
		Color::from_rgba_f(r, g, b, 1.0)
	}

	pub const fn from_rgba_f(r: f32, g: f32, b: f32, a: f32) -> Color
	{
		Color(ALLEGRO_COLOR {
			r: r,
			g: g,
			b: b,
			a: a,
		})
	}

	pub const fn from_allegro_color(c: ALLEGRO_COLOR) -> Color
	{
		Color(c)
	}

	pub const fn get_allegro_color(&self) -> ALLEGRO_COLOR
	{
		self.0
	}

	pub fn to_rgb(&self) -> (u8, u8, u8)
	{
		(
			(self.0.r * 255.0) as u8,
			(self.0.g * 255.0) as u8,
			(self.0.b * 255.0) as u8,
		)
	}

	pub fn to_rgba(&self) -> (u8, u8, u8, u8)
	{
		(
			(self.0.r * 255.0) as u8,
			(self.0.g * 255.0) as u8,
			(self.0.b * 255.0) as u8,
			(self.0.a * 255.0) as u8,
		)
	}

	pub const fn to_rgb_f(&self) -> (f32, f32, f32)
	{
		(self.0.r, self.0.g, self.0.b)
	}

	pub const fn to_rgba_f(&self) -> (f32, f32, f32, f32)
	{
		(self.0.r, self.0.g, self.0.b, self.0.a)
	}

	pub const fn to_rgba_array_f(&self) -> [f32; 4]
	{
		[self.0.r, self.0.g, self.0.b, self.0.a]
	}
}

#[repr(u32)]
#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug, PartialOrd, Ord)]
pub enum PixelFormat
{
	Any = ALLEGRO_PIXEL_FORMAT_ANY,
	AnyNoAlpha = ALLEGRO_PIXEL_FORMAT_ANY_NO_ALPHA,
	AnyWithAlpha = ALLEGRO_PIXEL_FORMAT_ANY_WITH_ALPHA,
	Any15NoAlpha = ALLEGRO_PIXEL_FORMAT_ANY_15_NO_ALPHA,
	Any16NoAlpha = ALLEGRO_PIXEL_FORMAT_ANY_16_NO_ALPHA,
	Any16WithAlpha = ALLEGRO_PIXEL_FORMAT_ANY_16_WITH_ALPHA,
	Any24NoAlpha = ALLEGRO_PIXEL_FORMAT_ANY_24_NO_ALPHA,
	Any32NoAlpha = ALLEGRO_PIXEL_FORMAT_ANY_32_NO_ALPHA,
	Any32WithAlpha = ALLEGRO_PIXEL_FORMAT_ANY_32_WITH_ALPHA,
	Argb8888 = ALLEGRO_PIXEL_FORMAT_ARGB_8888,
	Rgba8888 = ALLEGRO_PIXEL_FORMAT_RGBA_8888,
	Argb4444 = ALLEGRO_PIXEL_FORMAT_ARGB_4444,
	Rgb888 = ALLEGRO_PIXEL_FORMAT_RGB_888,
	Rgb565 = ALLEGRO_PIXEL_FORMAT_RGB_565,
	Rgb555 = ALLEGRO_PIXEL_FORMAT_RGB_555,
	Rgba5551 = ALLEGRO_PIXEL_FORMAT_RGBA_5551,
	Argb1555 = ALLEGRO_PIXEL_FORMAT_ARGB_1555,
	Abgr8888 = ALLEGRO_PIXEL_FORMAT_ABGR_8888,
	Xbgr8888 = ALLEGRO_PIXEL_FORMAT_XBGR_8888,
	Bgr888 = ALLEGRO_PIXEL_FORMAT_BGR_888,
	Bgr565 = ALLEGRO_PIXEL_FORMAT_BGR_565,
	Bgr555 = ALLEGRO_PIXEL_FORMAT_BGR_555,
	Rgbx8888 = ALLEGRO_PIXEL_FORMAT_RGBX_8888,
	Xrgb888 = ALLEGRO_PIXEL_FORMAT_XRGB_8888,
	AbgrF32 = ALLEGRO_PIXEL_FORMAT_ABGR_F32,
	Abgr8888Le = ALLEGRO_PIXEL_FORMAT_ABGR_8888_LE,
	Rgba4444 = ALLEGRO_PIXEL_FORMAT_RGBA_4444,
}

impl PixelFormat
{
	pub fn get_size(&self) -> usize
	{
		unsafe { al_get_pixel_size(*self as c_int) as usize }
	}

	pub fn get_bits(&self) -> i32
	{
		unsafe { al_get_pixel_format_bits(*self as c_int) as i32 }
	}
}
