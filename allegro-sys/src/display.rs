// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;

use events::ALLEGRO_EVENT_SOURCE;
use bitmap::ALLEGRO_BITMAP;
use allegro_util::c_bool;

pub const ALLEGRO_WINDOWED: u32                  = 1 << 0;
pub const ALLEGRO_FULLSCREEN: u32                = 1 << 1;
pub const ALLEGRO_OPENGL: u32                    = 1 << 2;
pub const ALLEGRO_DIRECT3D_INTERNAL: u32         = 1 << 3;
pub const ALLEGRO_RESIZABLE: u32                 = 1 << 4;
pub const ALLEGRO_FRAMELESS: u32                 = 1 << 5;
pub const ALLEGRO_NOFRAME: u32                   = ALLEGRO_FRAMELESS;
pub const ALLEGRO_GENERATE_EXPOSE_EVENTS: u32    = 1 << 6;
pub const ALLEGRO_OPENGL_3_0: u32                = 1 << 7;
pub const ALLEGRO_OPENGL_FORWARD_COMPATIBLE: u32 = 1 << 8;
pub const ALLEGRO_FULLSCREEN_WINDOW: u32         = 1 << 9;
pub const ALLEGRO_MINIMIZED: u32                 = 1 << 10;
pub const ALLEGRO_PROGRAMMABLE_PIPELINE: u32     = 1 << 11;
pub const ALLEGRO_MAXIMIZED: u32                 = 1 << 13;
pub const ALLEGRO_OPENGL_ES_PROFILE: u32         = 1 << 14;

pub const ALLEGRO_RED_SIZE: u32 = 0;
pub const ALLEGRO_GREEN_SIZE: u32 = 1;
pub const ALLEGRO_BLUE_SIZE: u32 = 2;
pub const ALLEGRO_ALPHA_SIZE: u32 = 3;
pub const ALLEGRO_RED_SHIFT: u32 = 4;
pub const ALLEGRO_GREEN_SHIFT: u32 = 5;
pub const ALLEGRO_BLUE_SHIFT: u32 = 6;
pub const ALLEGRO_ALPHA_SHIFT: u32 = 7;
pub const ALLEGRO_ACC_RED_SIZE: u32 = 8;
pub const ALLEGRO_ACC_GREEN_SIZE: u32 = 9;
pub const ALLEGRO_ACC_BLUE_SIZE: u32 = 10;
pub const ALLEGRO_ACC_ALPHA_SIZE: u32 = 11;
pub const ALLEGRO_STEREO: u32 = 12;
pub const ALLEGRO_AUX_BUFFERS: u32 = 13;
pub const ALLEGRO_COLOR_SIZE: u32 = 14;
pub const ALLEGRO_DEPTH_SIZE: u32 = 15;
pub const ALLEGRO_STENCIL_SIZE: u32 = 16;
pub const ALLEGRO_SAMPLE_BUFFERS: u32 = 17;
pub const ALLEGRO_SAMPLES: u32 = 18;
pub const ALLEGRO_RENDER_METHOD: u32 = 19;
pub const ALLEGRO_FLOAT_COLOR: u32 = 20;
pub const ALLEGRO_FLOAT_DEPTH: u32 = 21;
pub const ALLEGRO_SINGLE_BUFFER: u32 = 22;
pub const ALLEGRO_SWAP_METHOD: u32 = 23;
pub const ALLEGRO_COMPATIBLE_DISPLAY: u32 = 24;
pub const ALLEGRO_UPDATE_DISPLAY_REGION: u32 = 25;
pub const ALLEGRO_VSYNC: u32 = 26;
pub const ALLEGRO_MAX_BITMAP_SIZE: u32 = 27;
pub const ALLEGRO_SUPPORT_NPOT_BITMAP: u32 = 28;
pub const ALLEGRO_CAN_DRAW_INTO_BITMAP: u32 = 29;
pub const ALLEGRO_SUPPORT_SEPARATE_ALPHA: u32 = 30;
pub const ALLEGRO_DISPLAY_OPTIONS_COUNT: u32 = 31;

pub const ALLEGRO_DONTCARE: u32 = 0;
pub const ALLEGRO_REQUIRE: u32 = 1;
pub const ALLEGRO_SUGGEST: u32 = 2;

pub const ALLEGRO_DISPLAY_ORIENTATION_0_DEGREES: u32 = 0;
pub const ALLEGRO_DISPLAY_ORIENTATION_90_DEGREES: u32 = 1;
pub const ALLEGRO_DISPLAY_ORIENTATION_180_DEGREES: u32 = 2;
pub const ALLEGRO_DISPLAY_ORIENTATION_270_DEGREES: u32 = 3;
pub const ALLEGRO_DISPLAY_ORIENTATION_FACE_UP: u32 = 4;
pub const ALLEGRO_DISPLAY_ORIENTATION_FACE_DOWN: u32 = 5;

opaque!(ALLEGRO_DISPLAY);

extern "C"
{
	pub fn al_set_new_display_refresh_rate(refresh_rate: c_int);
	pub fn al_set_new_display_flags(flags: c_int);
	pub fn al_get_new_display_refresh_rate() -> c_int;
	pub fn al_get_new_display_flags() -> c_int;

	pub fn al_get_display_width(display: *mut ALLEGRO_DISPLAY) -> c_int;
	pub fn al_get_display_height(display: *mut ALLEGRO_DISPLAY) -> c_int;
	pub fn al_get_display_format(display: *mut ALLEGRO_DISPLAY) -> c_int;
	pub fn al_get_display_refresh_rate(display: *mut ALLEGRO_DISPLAY) -> c_int;
	pub fn al_get_display_flags(display: *mut ALLEGRO_DISPLAY) -> c_int;
	pub fn al_set_display_flag(display: *mut ALLEGRO_DISPLAY, flag: c_uint, onoff: c_bool) -> c_bool;

	pub fn al_create_display(w: c_int, h: c_int) -> *mut ALLEGRO_DISPLAY;
	pub fn al_destroy_display(display: *mut ALLEGRO_DISPLAY);
	pub fn al_get_current_display() -> *mut ALLEGRO_DISPLAY;
	pub fn al_set_target_bitmap(bitmap: *mut ALLEGRO_BITMAP);
	pub fn al_set_target_backbuffer(display: *mut ALLEGRO_DISPLAY);
	pub fn al_get_backbuffer(display: *mut ALLEGRO_DISPLAY) -> *mut ALLEGRO_BITMAP;
	pub fn al_get_target_bitmap() -> *mut ALLEGRO_BITMAP;

	pub fn al_acknowledge_resize(display: *mut ALLEGRO_DISPLAY) -> c_bool;
	pub fn al_resize_display(display: *mut ALLEGRO_DISPLAY, width: c_int, height: c_int) -> c_bool;
	pub fn al_flip_display();
	pub fn al_update_display_region(x: c_int, y: c_int, width: c_int, height: c_int);
	pub fn al_is_compatible_bitmap(bitmap: *mut ALLEGRO_BITMAP) -> c_bool;

	pub fn al_wait_for_vsync() -> c_bool;

	pub fn al_get_display_event_source(display: *mut ALLEGRO_DISPLAY) -> *mut ALLEGRO_EVENT_SOURCE;

	pub fn al_set_display_icon(display: *mut ALLEGRO_DISPLAY, icon: *mut ALLEGRO_BITMAP);
	pub fn al_set_display_icons(display: *mut ALLEGRO_DISPLAY, num_icons: c_int, icons: *mut *mut ALLEGRO_BITMAP);

	pub fn al_get_new_display_adapter() -> c_int;
	pub fn al_set_new_display_adapter(adapter: c_int);
	pub fn al_set_new_window_position(x: c_int, y: c_int);
	pub fn al_get_new_window_position(x: *mut c_int, y: *mut c_int);
	pub fn al_set_window_position(display: *mut ALLEGRO_DISPLAY, x: c_int, y: c_int);
	pub fn al_get_window_position(display: *mut ALLEGRO_DISPLAY, x: *mut c_int, y: *mut c_int);

	pub fn al_set_window_title(display: *mut ALLEGRO_DISPLAY, title: *const c_char);

	pub fn al_set_new_display_option(option: c_int, value: c_int, importance: c_int);
	pub fn al_get_new_display_option(option: c_int, importance: *mut c_int) -> c_int;
	pub fn al_reset_new_display_options();
	pub fn al_get_display_option(display: *mut ALLEGRO_DISPLAY, option: c_int) -> c_int;

	pub fn al_hold_bitmap_drawing(hold: c_bool);
	pub fn al_is_bitmap_drawing_held() -> c_bool;
}
