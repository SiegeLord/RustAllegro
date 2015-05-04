// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name="allegro_font_sys"]
#![crate_type = "lib"]

#![allow(non_camel_case_types)]
#![allow(non_camel_case_types, raw_pointer_derive)]

extern crate libc;
extern crate allegro_sys as allegro;
#[macro_use]
extern crate allegro_util;

pub use self::allegro_font::*;

pub mod allegro_font
{
	use libc::*;
	use allegro_util::c_bool;
    use allegro::{ALLEGRO_USTR, ALLEGRO_COLOR, ALLEGRO_BITMAP};

	#[repr(C)]
	#[derive(Copy, Clone)]
	pub struct ALLEGRO_FONT
	{
		pub data: *mut c_void,
		pub height: c_int,
		pub vtable: *mut ALLEGRO_FONT_VTABLE,
	}

	#[repr(C)]
	pub struct ALLEGRO_FONT_VTABLE
	{
		pub font_height: Option<extern "C" fn(arg1: *const ALLEGRO_FONT) -> c_int>,
		pub font_ascent: Option<extern "C" fn(arg1: *const ALLEGRO_FONT) -> c_int>,
		pub font_descent: Option<extern "C" fn(arg1: *const ALLEGRO_FONT) -> c_int>,
		pub char_length: Option<extern "C" fn(arg1: *const ALLEGRO_FONT, arg2: c_int) -> c_int>,
		pub text_length: Option<extern "C" fn(arg1: *const ALLEGRO_FONT, arg2: *const ALLEGRO_USTR) -> c_int>,
		pub render_char: Option<extern "C" fn(arg1: *const ALLEGRO_FONT, arg2: ALLEGRO_COLOR, arg3: c_int, arg4: c_float, arg5: c_float) -> c_int>,
		pub render: Option<extern "C" fn(arg1: *const ALLEGRO_FONT, arg2: ALLEGRO_COLOR, arg3: *const ALLEGRO_USTR, arg4: c_float, arg5: c_float) -> c_int>,
		pub destroy: Option<extern "C" fn(arg1: *mut ALLEGRO_FONT)>,
		pub get_text_dimensions: Option<extern "C" fn(arg1: *const ALLEGRO_FONT, arg2: *const ALLEGRO_USTR, arg3: *mut c_int, arg4: *mut c_int, arg5: *mut c_int, arg6: *mut c_int)>,
	}
	derive_copy_clone!(ALLEGRO_FONT_VTABLE);

	pub const ALLEGRO_ALIGN_LEFT: c_int = 0;
	pub const ALLEGRO_ALIGN_CENTRE: c_int = 1;
	pub const ALLEGRO_ALIGN_CENTER: c_int = 1;
	pub const ALLEGRO_ALIGN_RIGHT: c_int = 2;
	pub const ALLEGRO_ALIGN_INTEGER: c_int = 4;

	extern "C"
	{
		pub fn al_register_font_loader(ext: *const c_char, load: Option<extern "C" fn (arg1: *const c_char, arg2: c_int, arg3: c_int) -> *mut ALLEGRO_FONT>) -> c_bool;
		pub fn al_load_bitmap_font(filename: *const c_char) -> *mut ALLEGRO_FONT;
		pub fn al_load_font(filename: *const c_char, size: c_int, flags: c_int) -> *mut ALLEGRO_FONT;
		pub fn al_grab_font_from_bitmap(bmp: *mut ALLEGRO_BITMAP, n: c_int, ranges: *const c_int) -> *mut ALLEGRO_FONT;
		pub fn al_create_builtin_font() -> *mut ALLEGRO_FONT;
		pub fn al_draw_ustr(font: *const ALLEGRO_FONT, color: ALLEGRO_COLOR, x: c_float, y: c_float, flags: c_int, ustr: *const ALLEGRO_USTR);
		pub fn al_draw_text(font: *const ALLEGRO_FONT, color: ALLEGRO_COLOR, x: c_float, y: c_float, flags: c_int, text: *const c_char);
		pub fn al_draw_justified_text(font: *const ALLEGRO_FONT, color: ALLEGRO_COLOR, x1: c_float, x2: c_float, y: c_float, diff: c_float, flags: c_int, text: *const c_char);
		pub fn al_draw_justified_ustr(font: *const ALLEGRO_FONT, color: ALLEGRO_COLOR, x1: c_float, x2: c_float, y: c_float, diff: c_float, flags: c_int, text: *const ALLEGRO_USTR);
		pub fn al_get_text_width(f: *const ALLEGRO_FONT, str: *const c_char) -> c_int;
		pub fn al_get_ustr_width(f: *const ALLEGRO_FONT, ustr: *const ALLEGRO_USTR) -> c_int;
		pub fn al_get_font_line_height(f: *const ALLEGRO_FONT) -> c_int;
		pub fn al_get_font_ascent(f: *const ALLEGRO_FONT) -> c_int;
		pub fn al_get_font_descent(f: *const ALLEGRO_FONT) -> c_int;
		pub fn al_destroy_font(f: *mut ALLEGRO_FONT);
		pub fn al_get_ustr_dimensions(f: *const ALLEGRO_FONT, text: *const ALLEGRO_USTR, bbx: *mut c_int, bby: *mut c_int, bbw: *mut c_int, bbh: *mut c_int);
		pub fn al_get_text_dimensions(f: *const ALLEGRO_FONT, text: *const c_char, bbx: *mut c_int, bby: *mut c_int, bbw: *mut c_int, bbh: *mut c_int);
		pub fn al_init_font_addon();
		pub fn al_shutdown_font_addon();
		pub fn al_get_allegro_font_version() -> uint32_t;
	}
}
