// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name="allegro_color_sys"]
#![crate_type = "lib"]

extern crate allegro_sys;
extern crate allegro_util;
extern crate libc;

pub use allegro_color::*;

pub mod allegro_color
{
	#![allow(non_camel_case_types)]

	use libc::*;
	use allegro_util::c_bool;
	use allegro_sys::*;

	extern "C"
	{
		pub fn al_get_allegro_color_version() -> uint32_t;
		pub fn al_color_hsv_to_rgb(hue: c_float, saturation: c_float, value: c_float, red: *mut c_float, green: *mut c_float, blue: *mut c_float) -> ();
		pub fn al_color_rgb_to_hsl(red: c_float, green: c_float, blue: c_float, hue: *mut c_float, saturation: *mut c_float, lightness: *mut c_float) -> ();
		pub fn al_color_rgb_to_hsv(red: c_float, green: c_float, blue: c_float, hue: *mut c_float, saturation: *mut c_float, value: *mut c_float) -> ();
		pub fn al_color_hsl_to_rgb(hue: c_float, saturation: c_float, lightness: c_float, red: *mut c_float, green: *mut c_float, blue: *mut c_float) -> ();
		pub fn al_color_name_to_rgb(name: *const c_char, r: *mut c_float, g: *mut c_float, b: *mut c_float) -> c_bool;
		pub fn al_color_rgb_to_name(r: c_float, g: c_float, b: c_float) -> *const c_char;
		pub fn al_color_cmyk_to_rgb(cyan: c_float, magenta: c_float, yellow: c_float, key: c_float, red: *mut c_float, green: *mut c_float, blue: *mut c_float) -> ();
		pub fn al_color_rgb_to_cmyk(red: c_float, green: c_float, blue: c_float, cyan: *mut c_float, magenta: *mut c_float, yellow: *mut c_float, key: *mut c_float) -> ();
		pub fn al_color_yuv_to_rgb(y: c_float, u: c_float, v: c_float, red: *mut c_float, green: *mut c_float, blue: *mut c_float) -> ();
		pub fn al_color_rgb_to_yuv(red: c_float, green: c_float, blue: c_float, y: *mut c_float, u: *mut c_float, v: *mut c_float) -> ();
		pub fn al_color_rgb_to_html(red: c_float, green: c_float, blue: c_float, string: *mut c_char) -> ();
		pub fn al_color_html_to_rgb(string: *const c_char, red: *mut c_float, green: *mut c_float, blue: *mut c_float) -> c_bool;

		pub fn al_color_yuv(y: c_float, u: c_float, v: c_float) -> ALLEGRO_COLOR;
		pub fn al_color_cmyk(c: c_float, m: c_float, y: c_float, k: c_float) -> ALLEGRO_COLOR;
		pub fn al_color_hsl(h: c_float, s: c_float, l: c_float) -> ALLEGRO_COLOR;
		pub fn al_color_hsv(h: c_float, s: c_float, v: c_float) -> ALLEGRO_COLOR;
		pub fn al_color_name(name: *const c_char) -> ALLEGRO_COLOR;
		pub fn al_color_html(string: *const c_char) -> ALLEGRO_COLOR;
	}
}
