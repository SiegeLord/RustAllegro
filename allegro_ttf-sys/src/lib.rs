// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name="allegro_ttf_sys"]
#![crate_type = "lib"]

extern crate allegro_font_sys;
#[macro_use]
extern crate allegro_util;
extern crate libc;

pub use self::allegro_ttf::*;

pub mod allegro_ttf
{
	use libc::*;
	use allegro_util::c_bool;
	use allegro_font_sys::ALLEGRO_FONT;

	pub const ALLEGRO_TTF_NO_KERNING: u32  = 1;
	pub const ALLEGRO_TTF_MONOCHROME: u32  = 2;
	pub const ALLEGRO_TTF_NO_AUTOHINT: u32 = 4;

	extern "C"
	{
		pub fn al_load_ttf_font(filename: *const c_char, size: c_int, flags: c_int) -> *mut ALLEGRO_FONT;
		//~ pub fn al_load_ttf_font_f(file: *mut ALLEGRO_FILE, filename: *const c_char, size: c_int, flags: c_int) -> *mut ALLEGRO_FONT;
		pub fn al_load_ttf_font_stretch(filename: *const c_char, w: c_int, h: c_int, flags: c_int) -> *mut ALLEGRO_FONT;
		//~ pub fn al_load_ttf_font_stretch_f(file: *mut ALLEGRO_FILE, filename: *const c_char, w: c_int, h: c_int, flags: c_int) -> *mut ALLEGRO_FONT;
		pub fn al_init_ttf_addon() -> c_bool;
		pub fn al_shutdown_ttf_addon();
		pub fn al_get_allegro_ttf_version() -> uint32_t;
	}
}
