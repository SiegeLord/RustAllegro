// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;
use std::ops::Deref;

use ffi::*;

#[deriving(Clone)]
pub struct Color(pub ALLEGRO_COLOR);

impl Color
{
	pub fn unmap_rgb(&self) -> (u8, u8, u8)
	{
		unsafe
		{
			let mut r = 0u8;
			let mut g = 0u8;
			let mut b = 0u8;
			al_unmap_rgb(**self, &mut r, &mut g, &mut b);
			(r, g, b)
		}
	}

	pub fn unmap_rgba(&self) -> (u8, u8, u8, u8)
	{
		unsafe
		{
			let mut r = 0u8;
			let mut g = 0u8;
			let mut b = 0u8;
			let mut a = 0u8;
			al_unmap_rgba(**self, &mut r, &mut g, &mut b, &mut a);
			(r, g, b, a)
		}
	}

	pub fn unmap_rgb_f(&self) -> (f32, f32, f32)
	{
		(self.r, self.g, self.b)
	}

	pub fn unmap_rgba_f(&self) -> (f32, f32, f32, f32)
	{
		(self.r, self.g, self.b, self.a)
	}

	pub fn get_color(&self) -> ALLEGRO_COLOR
	{
		**self
	}
}

impl Deref<ALLEGRO_COLOR> for Color
{
	fn deref<'l>(&'l self) -> &'l ALLEGRO_COLOR
	{
		let Color(ref c) = *self;
		c
	}
}

#[repr(u32)]
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
		unsafe
		{
			al_get_pixel_size(*self as c_int) as i32
		}
	}

	pub fn get_bits(&self) -> i32
	{
		unsafe
		{
			al_get_pixel_format_bits(*self as c_int) as i32
		}
	}
}
