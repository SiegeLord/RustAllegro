// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;
use allegro_util::c_bool;

use events::ALLEGRO_EVENT_SOURCE;

opaque!(ALLEGRO_JOYSTICK);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ALLEGRO_JOYSTICK_STATE
{
	pub stick: [Stick; 8],
	pub button: [c_int; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Stick
{
	pub axis: [c_float; 3],
}

pub const ALLEGRO_JOYFLAG_DIGITAL: c_uint = 1;
pub const ALLEGRO_JOYFLAG_ANALOGUE: c_uint = 2;

extern "C"
{
	pub fn al_install_joystick() -> c_bool;
	pub fn al_uninstall_joystick();
	pub fn al_is_joystick_installed() -> c_bool;
	pub fn al_reconfigure_joysticks() -> c_bool;
	pub fn al_get_num_joysticks() -> c_int;
	pub fn al_get_joystick(joyn: c_int) -> *mut ALLEGRO_JOYSTICK;
	pub fn al_release_joystick(arg1: *mut ALLEGRO_JOYSTICK);
	pub fn al_get_joystick_active(arg1: *mut ALLEGRO_JOYSTICK) -> c_bool;
	pub fn al_get_joystick_name(arg1: *mut ALLEGRO_JOYSTICK) -> *const c_char;
	pub fn al_get_joystick_num_sticks(arg1: *mut ALLEGRO_JOYSTICK) -> c_int;
	pub fn al_get_joystick_stick_flags(arg1: *mut ALLEGRO_JOYSTICK, stick: c_int) -> c_int;
	pub fn al_get_joystick_stick_name(arg1: *mut ALLEGRO_JOYSTICK, stick: c_int) -> *const c_char;
	pub fn al_get_joystick_num_axes(arg1: *mut ALLEGRO_JOYSTICK, stick: c_int) -> c_int;
	pub fn al_get_joystick_axis_name(arg1: *mut ALLEGRO_JOYSTICK, stick: c_int, axis: c_int) -> *const c_char;
	pub fn al_get_joystick_num_buttons(arg1: *mut ALLEGRO_JOYSTICK) -> c_int;
	pub fn al_get_joystick_button_name(arg1: *mut ALLEGRO_JOYSTICK, buttonn: c_int) -> *const c_char;
	pub fn al_get_joystick_state(arg1: *mut ALLEGRO_JOYSTICK, ret_state: *mut ALLEGRO_JOYSTICK_STATE);
	pub fn al_get_joystick_event_source() -> *mut ALLEGRO_EVENT_SOURCE;
}
