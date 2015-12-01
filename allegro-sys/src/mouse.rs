// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;
use display::ALLEGRO_DISPLAY;
use events::ALLEGRO_EVENT_SOURCE;
use allegro_util::c_bool;

opaque!(ALLEGRO_MOUSE);

pub const ALLEGRO_MOUSE_MAX_EXTRA_AXES: u32 = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ALLEGRO_MOUSE_STATE
{
    pub x: c_int,
    pub y: c_int,
    pub z: c_int,
    pub w: c_int,
    pub more_axes: [c_int; ALLEGRO_MOUSE_MAX_EXTRA_AXES as usize],
    pub buttons: c_int,
    pub pressure: c_float,
    pub display: *mut ALLEGRO_DISPLAY,
}

extern "C"
{
	pub fn al_is_mouse_installed() -> c_bool;
	pub fn al_install_mouse() -> c_bool;
	pub fn al_uninstall_mouse();
	pub fn al_get_mouse_num_buttons() -> c_uint;
	pub fn al_get_mouse_num_axes() -> c_uint;
	pub fn al_set_mouse_xy(display: *mut ALLEGRO_DISPLAY, x: c_int, y: c_int) -> c_bool;
	pub fn al_set_mouse_z(z: c_int) -> c_bool;
	pub fn al_set_mouse_w(w: c_int) -> c_bool;
	pub fn al_set_mouse_axis(axis: c_int, value: c_int) -> c_bool;
	pub fn al_get_mouse_state(ret_state: *mut ALLEGRO_MOUSE_STATE);
	pub fn al_mouse_button_down(state: *const ALLEGRO_MOUSE_STATE, button: c_int) -> c_bool;
	pub fn al_get_mouse_state_axis(state: *const ALLEGRO_MOUSE_STATE, axis: c_int) -> c_int;
	pub fn al_get_mouse_cursor_position(ret_x: *mut c_int, ret_y: *mut c_int) -> c_bool;
	pub fn al_grab_mouse(display: *mut ALLEGRO_DISPLAY) -> c_bool;
	pub fn al_ungrab_mouse() -> c_bool;
	pub fn al_get_mouse_event_source() -> *mut ALLEGRO_EVENT_SOURCE;
}
