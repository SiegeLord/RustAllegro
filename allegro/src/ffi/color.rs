// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;

#[repr(C)]
#[deriving(Clone)]
pub struct ALLEGRO_COLOR
{
	pub r: f32,
	pub g: f32,
	pub b: f32,
	pub a: f32
}

pub static ALLEGRO_PIXEL_FORMAT_ANY: u32 = 0;
pub static ALLEGRO_PIXEL_FORMAT_ANY_NO_ALPHA: u32 = 1;
pub static ALLEGRO_PIXEL_FORMAT_ANY_WITH_ALPHA: u32 = 2;
pub static ALLEGRO_PIXEL_FORMAT_ANY_15_NO_ALPHA: u32 = 3;
pub static ALLEGRO_PIXEL_FORMAT_ANY_16_NO_ALPHA: u32 = 4;
pub static ALLEGRO_PIXEL_FORMAT_ANY_16_WITH_ALPHA: u32 = 5;
pub static ALLEGRO_PIXEL_FORMAT_ANY_24_NO_ALPHA: u32 = 6;
pub static ALLEGRO_PIXEL_FORMAT_ANY_32_NO_ALPHA: u32 = 7;
pub static ALLEGRO_PIXEL_FORMAT_ANY_32_WITH_ALPHA: u32 = 8;
pub static ALLEGRO_PIXEL_FORMAT_ARGB_8888: u32 = 9;
pub static ALLEGRO_PIXEL_FORMAT_RGBA_8888: u32 = 10;
pub static ALLEGRO_PIXEL_FORMAT_ARGB_4444: u32 = 11;
pub static ALLEGRO_PIXEL_FORMAT_RGB_888: u32 = 12;
pub static ALLEGRO_PIXEL_FORMAT_RGB_565: u32 = 13;
pub static ALLEGRO_PIXEL_FORMAT_RGB_555: u32 = 14;
pub static ALLEGRO_PIXEL_FORMAT_RGBA_5551: u32 = 15;
pub static ALLEGRO_PIXEL_FORMAT_ARGB_1555: u32 = 16;
pub static ALLEGRO_PIXEL_FORMAT_ABGR_8888: u32 = 17;
pub static ALLEGRO_PIXEL_FORMAT_XBGR_8888: u32 = 18;
pub static ALLEGRO_PIXEL_FORMAT_BGR_888: u32 = 19;
pub static ALLEGRO_PIXEL_FORMAT_BGR_565: u32 = 20;
pub static ALLEGRO_PIXEL_FORMAT_BGR_555: u32 = 21;
pub static ALLEGRO_PIXEL_FORMAT_RGBX_8888: u32 = 22;
pub static ALLEGRO_PIXEL_FORMAT_XRGB_8888: u32 = 23;
pub static ALLEGRO_PIXEL_FORMAT_ABGR_F32: u32 = 24;
pub static ALLEGRO_PIXEL_FORMAT_ABGR_8888_LE: u32 = 25;
pub static ALLEGRO_PIXEL_FORMAT_RGBA_4444: u32 = 26;
pub static ALLEGRO_NUM_PIXEL_FORMATS: u32 = 27;

extern "C"
{
	pub fn al_map_rgb(r: c_uchar, g: c_uchar, b: c_uchar) -> ALLEGRO_COLOR;
	pub fn al_map_rgba(r: c_uchar, g: c_uchar, b: c_uchar, a: c_uchar) -> ALLEGRO_COLOR;
	pub fn al_map_rgb_f(r: c_float, g: c_float, b: c_float) -> ALLEGRO_COLOR;
	pub fn al_map_rgba_f(r: c_float, g: c_float, b: c_float, a: c_float) -> ALLEGRO_COLOR;

	pub fn al_unmap_rgb(color: ALLEGRO_COLOR, r: *mut c_uchar, g: *mut c_uchar, b: *mut c_uchar);
	pub fn al_unmap_rgba(color: ALLEGRO_COLOR, r: *mut c_uchar, g: *mut c_uchar, b: *mut c_uchar, a: *mut c_uchar);
	pub fn al_unmap_rgb_f(color: ALLEGRO_COLOR, r: *mut c_float, g: *mut c_float, b: *mut c_float);
	pub fn al_unmap_rgba_f(color: ALLEGRO_COLOR, r: *mut c_float, g: *mut c_float, b: *mut c_float, a: *mut c_float);

	pub fn al_get_pixel_size(format: c_int) -> c_int;
	pub fn al_get_pixel_format_bits(format: c_int) -> c_int;
}
