
#![allow(non_camel_case_types)]

pub use self::allegro_font::*;

pub mod allegro_font
{
	use libc::*;
	use allegro::c_bool;
    use allegro::ffi::{ALLEGRO_USTR, ALLEGRO_COLOR, ALLEGRO_BITMAP};

	pub struct ALLEGRO_FONT
	{
		data: *mut c_void,
		height: c_int,
		vtable: *mut ALLEGRO_FONT_VTABLE,
	}

	pub struct ALLEGRO_FONT_VTABLE
	{
		font_height: Option<extern "C" fn(arg1: *ALLEGRO_FONT) -> c_int>,
		font_ascent: Option<extern "C" fn(arg1: *ALLEGRO_FONT) -> c_int>,
		font_descent: Option<extern "C" fn(arg1: *ALLEGRO_FONT) -> c_int>,
		char_length: Option<extern "C" fn(arg1: *ALLEGRO_FONT, arg2: c_int) -> c_int>,
		text_length: Option<extern "C" fn(arg1: *ALLEGRO_FONT, arg2: *ALLEGRO_USTR) -> c_int>,
		render_char: Option<extern "C" fn(arg1: *ALLEGRO_FONT, arg2: ALLEGRO_COLOR, arg3: c_int, arg4: c_float, arg5: c_float) -> c_int>,
		render: Option<extern "C" fn(arg1: *ALLEGRO_FONT, arg2: ALLEGRO_COLOR, arg3: *ALLEGRO_USTR, arg4: c_float, arg5: c_float) -> c_int>,
		destroy: Option<extern "C" fn(arg1: *mut ALLEGRO_FONT)>,
		get_text_dimensions: Option<extern "C" fn(arg1: *ALLEGRO_FONT, arg2: *ALLEGRO_USTR, arg3: *mut c_int, arg4: *mut c_int, arg5: *mut c_int, arg6: *mut c_int)>,
	}

	pub static ALLEGRO_ALIGN_LEFT: c_int = 0;
	pub static ALLEGRO_ALIGN_CENTRE: c_int = 1;
	pub static ALLEGRO_ALIGN_CENTER: c_int = 1;
	pub static ALLEGRO_ALIGN_RIGHT: c_int = 2;
	pub static ALLEGRO_ALIGN_INTEGER: c_int = 4;

	extern "C"
	{
		pub fn al_register_font_loader(ext: *c_schar, load: Option<extern "C" fn (arg1: *c_schar, arg2: c_int, arg3: c_int) -> *mut ALLEGRO_FONT>) -> c_bool;
		pub fn al_load_bitmap_font(filename: *c_schar) -> *mut ALLEGRO_FONT;
		pub fn al_load_font(filename: *c_schar, size: c_int, flags: c_int) -> *mut ALLEGRO_FONT;
		pub fn al_grab_font_from_bitmap(bmp: *mut ALLEGRO_BITMAP, n: c_int, ranges: *c_int) -> *mut ALLEGRO_FONT;
		pub fn al_create_builtin_font() -> *mut ALLEGRO_FONT;
		pub fn al_draw_ustr(font: *ALLEGRO_FONT, color: ALLEGRO_COLOR, x: c_float, y: c_float, flags: c_int, ustr: *ALLEGRO_USTR);
		pub fn al_draw_text(font: *ALLEGRO_FONT, color: ALLEGRO_COLOR, x: c_float, y: c_float, flags: c_int, text: *c_schar);
		pub fn al_draw_justified_text(font: *ALLEGRO_FONT, color: ALLEGRO_COLOR, x1: c_float, x2: c_float, y: c_float, diff: c_float, flags: c_int, text: *c_schar);
		pub fn al_draw_justified_ustr(font: *ALLEGRO_FONT, color: ALLEGRO_COLOR, x1: c_float, x2: c_float, y: c_float, diff: c_float, flags: c_int, text: *ALLEGRO_USTR);
		pub fn al_get_text_width(f: *ALLEGRO_FONT, str: *c_schar) -> c_int;
		pub fn al_get_ustr_width(f: *ALLEGRO_FONT, ustr: *ALLEGRO_USTR) -> c_int;
		pub fn al_get_font_line_height(f: *ALLEGRO_FONT) -> c_int;
		pub fn al_get_font_ascent(f: *ALLEGRO_FONT) -> c_int;
		pub fn al_get_font_descent(f: *ALLEGRO_FONT) -> c_int;
		pub fn al_destroy_font(f: *mut ALLEGRO_FONT);
		pub fn al_get_ustr_dimensions(f: *ALLEGRO_FONT, text: *ALLEGRO_USTR, bbx: *mut c_int, bby: *mut c_int, bbw: *mut c_int, bbh: *mut c_int);
		pub fn al_get_text_dimensions(f: *ALLEGRO_FONT, text: *c_schar, bbx: *mut c_int, bby: *mut c_int, bbw: *mut c_int, bbh: *mut c_int);
		pub fn al_init_font_addon();
		pub fn al_shutdown_font_addon();
		pub fn al_get_allegro_font_version() -> uint32_t;
	}
}
