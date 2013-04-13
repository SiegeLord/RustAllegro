use core::libc::*;

pub mod C
{
	use core::libc::*;

	use super::ALLEGRO_COLOR;

	pub extern "C"
	{
		fn al_map_rgb(r : c_uchar, g : c_uchar, b : c_uchar) -> ALLEGRO_COLOR;
		fn al_map_rgba(r : c_uchar, g : c_uchar, b : c_uchar, a : c_uchar) -> ALLEGRO_COLOR;
		fn al_map_rgb_f(r : c_float, g : c_float, b : c_float) -> ALLEGRO_COLOR;
		fn al_map_rgba_f(r : c_float, g : c_float, b : c_float, a : c_float) -> ALLEGRO_COLOR;

		fn al_unmap_rgb(color : ALLEGRO_COLOR, r : *mut c_uchar, g : *mut c_uchar, b : *mut c_uchar);
		fn al_unmap_rgba(color : ALLEGRO_COLOR, r : *mut c_uchar, g : *mut c_uchar, b : *mut c_uchar, a : *mut c_uchar);
		fn al_unmap_rgb_f(color : ALLEGRO_COLOR, r : *mut c_float, g : *mut c_float, b : *mut c_float);
		fn al_unmap_rgba_f(color : ALLEGRO_COLOR, r : *mut c_float, g : *mut c_float, b : *mut c_float, a : *mut c_float);

		fn al_get_pixel_size(format : c_int) -> c_int;
		fn al_get_pixel_format_bits(format : c_int) -> c_int;
	}
}

pub struct ALLEGRO_COLOR
{
	// Issue #5874
	pub r : i32,
	pub g : i32,
	pub b : i32,
	pub a : i32
}

pub static ALLEGRO_PIXEL_FORMAT_ANY : i32 = 0;
pub static ALLEGRO_PIXEL_FORMAT_ANY_NO_ALPHA : i32 = 1;
pub static ALLEGRO_PIXEL_FORMAT_ANY_WITH_ALPHA : i32 = 2;
pub static ALLEGRO_PIXEL_FORMAT_ANY_15_NO_ALPHA : i32 = 3;
pub static ALLEGRO_PIXEL_FORMAT_ANY_16_NO_ALPHA : i32 = 4;
pub static ALLEGRO_PIXEL_FORMAT_ANY_16_WITH_ALPHA : i32 = 5;
pub static ALLEGRO_PIXEL_FORMAT_ANY_24_NO_ALPHA : i32 = 6;
pub static ALLEGRO_PIXEL_FORMAT_ANY_32_NO_ALPHA : i32 = 7;
pub static ALLEGRO_PIXEL_FORMAT_ANY_32_WITH_ALPHA : i32 = 8;
pub static ALLEGRO_PIXEL_FORMAT_ARGB_8888 : i32 = 9;
pub static ALLEGRO_PIXEL_FORMAT_RGBA_8888 : i32 = 10;
pub static ALLEGRO_PIXEL_FORMAT_ARGB_4444 : i32 = 11;
pub static ALLEGRO_PIXEL_FORMAT_RGB_888 : i32 = 12;
pub static ALLEGRO_PIXEL_FORMAT_RGB_565 : i32 = 13;
pub static ALLEGRO_PIXEL_FORMAT_RGB_555 : i32 = 14;
pub static ALLEGRO_PIXEL_FORMAT_RGBA_5551 : i32 = 15;
pub static ALLEGRO_PIXEL_FORMAT_ARGB_1555 : i32 = 16;
pub static ALLEGRO_PIXEL_FORMAT_ABGR_8888 : i32 = 17;
pub static ALLEGRO_PIXEL_FORMAT_XBGR_8888 : i32 = 18;
pub static ALLEGRO_PIXEL_FORMAT_BGR_888 : i32 = 19;
pub static ALLEGRO_PIXEL_FORMAT_BGR_565 : i32 = 20;
pub static ALLEGRO_PIXEL_FORMAT_BGR_555 : i32 = 21;
pub static ALLEGRO_PIXEL_FORMAT_RGBX_8888 : i32 = 22;
pub static ALLEGRO_PIXEL_FORMAT_XRGB_8888 : i32 = 23;
pub static ALLEGRO_PIXEL_FORMAT_ABGR_F32 : i32 = 24;
pub static ALLEGRO_PIXEL_FORMAT_ABGR_8888_LE : i32 = 25;
pub static ALLEGRO_PIXEL_FORMAT_RGBA_4444 : i32 = 26;
pub static ALLEGRO_PIXEL_FORMAT_SINGLE_CHANNEL_8 : i32 = 27;
pub static ALLEGRO_NUM_PIXEL_FORMATS : i32 = 28;

pub fn al_map_rgb(r : u8, g : u8, b : u8) -> ALLEGRO_COLOR
{
	unsafe
	{
		return C::al_map_rgb(r as c_uchar, g as c_uchar, b as c_uchar);
	}
}

pub fn al_map_rgba(r : u8, g : u8, b : u8, a : u8) -> ALLEGRO_COLOR
{
	unsafe
	{
		return C::al_map_rgba(r as c_uchar, g as c_uchar, b as c_uchar, a as c_uchar);
	}
}

pub fn al_map_rgb_f(r : f32, g : f32, b : f32) -> ALLEGRO_COLOR
{
	unsafe
	{
		return C::al_map_rgb_f(r as c_float, g as c_float, b as c_float);
	}
}

pub fn al_map_rgba_f(r : f32, g : f32, b : f32, a : f32) -> ALLEGRO_COLOR
{
	unsafe
	{
		return C::al_map_rgba_f(r as c_float, g as c_float, b as c_float, a as c_float);
	}
}

pub fn al_unmap_rgb(color : ALLEGRO_COLOR, r : &mut u8, g : &mut u8, b : &mut u8)
{
	unsafe
	{
		C::al_unmap_rgb(color, r as *mut c_uchar, g as *mut c_uchar, b as *mut c_uchar);
	}
}

pub fn al_unmap_rgba(color : ALLEGRO_COLOR, r : &mut u8, g : &mut u8, b : &mut u8, a : &mut u8)
{
	unsafe
	{
		C::al_unmap_rgba(color, r as *mut c_uchar, g as *mut c_uchar, b as *mut c_uchar, a as *mut c_uchar);
	}
}

pub fn al_unmap_rgb_f(color : ALLEGRO_COLOR, r : &mut f32, g : &mut f32, b : &mut f32)
{
	unsafe
	{
		C::al_unmap_rgb_f(color, r as *mut c_float, g as *mut c_float, b as *mut c_float);
	}
}

pub fn al_unmap_rgba_f(color : ALLEGRO_COLOR, r : &mut f32, g : &mut f32, b : &mut f32, a : &mut f32)
{
	unsafe
	{
		C::al_unmap_rgba_f(color, r as *mut c_float, g as *mut c_float, b as *mut c_float, a as *mut c_float);
	}
}

pub fn al_get_pixel_size(format : i32) -> i32
{
	unsafe
	{
		return C::al_get_pixel_size(format as c_int) as i32;
	}
}

pub fn al_get_pixel_format_bits(format : i32) -> i32
{
	unsafe
	{
		return C::al_get_pixel_format_bits(format as c_int) as i32;
	}
}
