// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![allow(non_snake_case)]

use libc::*;

use events::ALLEGRO_EVENT_SOURCE;
use display::ALLEGRO_DISPLAY;
use allegro_util::c_bool;

opaque!(ALLEGRO_KEYBOARD);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ALLEGRO_KEYBOARD_STATE
{
	pub display: *mut ALLEGRO_DISPLAY,
	pub __key_down__internal__: [c_uint; 8],
}

extern "C"
{
	pub fn al_is_keyboard_installed() -> c_bool;
	pub fn al_install_keyboard() -> c_bool;
	pub fn al_uninstall_keyboard();
	pub fn al_set_keyboard_leds(leds: c_int) -> c_bool;
	pub fn al_keycode_to_name(keycode: c_int) -> *const c_char;
	pub fn al_get_keyboard_state(ret_state: *mut ALLEGRO_KEYBOARD_STATE);
	pub fn al_key_down(arg1: *const ALLEGRO_KEYBOARD_STATE, keycode: c_int) -> c_bool;
	pub fn al_get_keyboard_event_source() -> *mut ALLEGRO_EVENT_SOURCE;
}
