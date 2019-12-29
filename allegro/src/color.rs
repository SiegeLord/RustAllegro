// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use ffi::*;
use libc::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
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

	pub fn from_rgb_f(r: f32, g: f32, b: f32) -> Color
	{
		Color::from_rgba_f(r, g, b, 1.0)
	}

	pub fn from_rgba_f(r: f32, g: f32, b: f32, a: f32) -> Color
	{
		Color(ALLEGRO_COLOR {
			r: r,
			g: g,
			b: b,
			a: a,
		})
	}

	pub fn from_allegro_color(c: ALLEGRO_COLOR) -> Color
	{
		Color(c)
	}

	pub fn get_allegro_color(&self) -> ALLEGRO_COLOR
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

	pub fn to_rgb_f(&self) -> (f32, f32, f32)
	{
		(self.0.r, self.0.g, self.0.b)
	}

	pub fn to_rgba_f(&self) -> (f32, f32, f32, f32)
	{
		(self.0.r, self.0.g, self.0.b, self.0.a)
	}
}

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum PixelFormat
{
	PixelFormatAny = ALLEGRO_PIXEL_FORMAT_ANY,
	PixelFormatAnyNoAlpha = ALLEGRO_PIXEL_FORMAT_ANY_NO_ALPHA,
	PixelFormatAnyWithAlpha = ALLEGRO_PIXEL_FORMAT_ANY_WITH_ALPHA,
	PixelFormatAny15NoAlpha = ALLEGRO_PIXEL_FORMAT_ANY_15_NO_ALPHA,
	PixelFormatAny16NoAlpha = ALLEGRO_PIXEL_FORMAT_ANY_16_NO_ALPHA,
	PixelFormatAny16WithAlpha = ALLEGRO_PIXEL_FORMAT_ANY_16_WITH_ALPHA,
	PixelFormatAny24NoAlpha = ALLEGRO_PIXEL_FORMAT_ANY_24_NO_ALPHA,
	PixelFormatAny32NoAlpha = ALLEGRO_PIXEL_FORMAT_ANY_32_NO_ALPHA,
	PixelFormatAny32WithAlpha = ALLEGRO_PIXEL_FORMAT_ANY_32_WITH_ALPHA,
	PixelFormatArgb8888 = ALLEGRO_PIXEL_FORMAT_ARGB_8888,
	PixelFormatRgba8888 = ALLEGRO_PIXEL_FORMAT_RGBA_8888,
	PixelFormatArgb4444 = ALLEGRO_PIXEL_FORMAT_ARGB_4444,
	PixelFormatRgb888 = ALLEGRO_PIXEL_FORMAT_RGB_888,
	PixelFormatRgb565 = ALLEGRO_PIXEL_FORMAT_RGB_565,
	PixelFormatRgb555 = ALLEGRO_PIXEL_FORMAT_RGB_555,
	PixelFormatRgba5551 = ALLEGRO_PIXEL_FORMAT_RGBA_5551,
	PixelFormatArgb1555 = ALLEGRO_PIXEL_FORMAT_ARGB_1555,
	PixelFormatAbgr8888 = ALLEGRO_PIXEL_FORMAT_ABGR_8888,
	PixelFormatXbgr8888 = ALLEGRO_PIXEL_FORMAT_XBGR_8888,
	PixelFormatBgr888 = ALLEGRO_PIXEL_FORMAT_BGR_888,
	PixelFormatBgr565 = ALLEGRO_PIXEL_FORMAT_BGR_565,
	PixelFormatBgr555 = ALLEGRO_PIXEL_FORMAT_BGR_555,
	PixelFormatRgbx8888 = ALLEGRO_PIXEL_FORMAT_RGBX_8888,
	PixelFormatXrgb888 = ALLEGRO_PIXEL_FORMAT_XRGB_8888,
	PixelFormatAbgrF32 = ALLEGRO_PIXEL_FORMAT_ABGR_F32,
	PixelFormatAbgr8888Le = ALLEGRO_PIXEL_FORMAT_ABGR_8888_LE,
	PixelFormatRgba4444 = ALLEGRO_PIXEL_FORMAT_RGBA_4444,
}

impl PixelFormat
{
	pub fn get_size(&self) -> i32
	{
		unsafe { al_get_pixel_size(*self as c_int) as i32 }
	}

	pub fn get_bits(&self) -> i32
	{
		unsafe { al_get_pixel_format_bits(*self as c_int) as i32 }
	}
}
