// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use crate::events::ALLEGRO_EVENT_SOURCE;
#[cfg(all(feature = "unstable", allegro_5_2_11))]
use crate::file::ALLEGRO_FILE;

use allegro_util::c_bool;
use libc::*;

allegro_util::opaque!(ALLEGRO_JOYSTICK);

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

pub const ALLEGRO_JOYFLAG_DIGITAL: c_int = 1;
pub const ALLEGRO_JOYFLAG_ANALOGUE: c_int = 2;

#[cfg(all(feature = "unstable", allegro_5_2_11))]
pub const ALLEGRO_GAMEPAD_BUTTON_A: c_int = 0;
#[cfg(all(feature = "unstable", allegro_5_2_11))]
pub const ALLEGRO_GAMEPAD_BUTTON_B: c_int = 1;
#[cfg(all(feature = "unstable", allegro_5_2_11))]
pub const ALLEGRO_GAMEPAD_BUTTON_X: c_int = 2;
#[cfg(all(feature = "unstable", allegro_5_2_11))]
pub const ALLEGRO_GAMEPAD_BUTTON_Y: c_int = 3;
#[cfg(all(feature = "unstable", allegro_5_2_11))]
pub const ALLEGRO_GAMEPAD_BUTTON_LEFT_SHOULDER: c_int = 4;
#[cfg(all(feature = "unstable", allegro_5_2_11))]
pub const ALLEGRO_GAMEPAD_BUTTON_RIGHT_SHOULDER: c_int = 5;
#[cfg(all(feature = "unstable", allegro_5_2_11))]
pub const ALLEGRO_GAMEPAD_BUTTON_BACK: c_int = 6;
#[cfg(all(feature = "unstable", allegro_5_2_11))]
pub const ALLEGRO_GAMEPAD_BUTTON_START: c_int = 7;
#[cfg(all(feature = "unstable", allegro_5_2_11))]
pub const ALLEGRO_GAMEPAD_BUTTON_GUIDE: c_int = 8;
#[cfg(all(feature = "unstable", allegro_5_2_11))]
pub const ALLEGRO_GAMEPAD_BUTTON_LEFT_THUMB: c_int = 9;
#[cfg(all(feature = "unstable", allegro_5_2_11))]
pub const ALLEGRO_GAMEPAD_BUTTON_RIGHT_THUMB: c_int = 10;

#[cfg(all(feature = "unstable", allegro_5_2_11))]
pub const ALLEGRO_GAMEPAD_STICK_DPAD: c_int = 0;
#[cfg(all(feature = "unstable", allegro_5_2_11))]
pub const ALLEGRO_GAMEPAD_STICK_LEFT_THUMB: c_int = 1;
#[cfg(all(feature = "unstable", allegro_5_2_11))]
pub const ALLEGRO_GAMEPAD_STICK_RIGHT_THUMB: c_int = 2;
#[cfg(all(feature = "unstable", allegro_5_2_11))]
pub const ALLEGRO_GAMEPAD_STICK_LEFT_TRIGGER: c_int = 3;
#[cfg(all(feature = "unstable", allegro_5_2_11))]
pub const ALLEGRO_GAMEPAD_STICK_RIGHT_TRIGGER: c_int = 4;

#[cfg(all(feature = "unstable", allegro_5_2_11))]
pub const ALLEGRO_JOYSTICK_TYPE_UNKNOWN: c_int = 0;
#[cfg(all(feature = "unstable", allegro_5_2_11))]
pub const ALLEGRO_JOYSTICK_TYPE_GAMEPAD: c_int = 1;

#[cfg(all(feature = "unstable", allegro_5_2_11))]
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct ALLEGRO_JOYSTICK_GUID
{
	pub val: [u8; 16],
}

unsafe extern "C" {
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
	pub fn al_get_joystick_axis_name(
		arg1: *mut ALLEGRO_JOYSTICK, stick: c_int, axis: c_int,
	) -> *const c_char;
	pub fn al_get_joystick_num_buttons(arg1: *mut ALLEGRO_JOYSTICK) -> c_int;
	pub fn al_get_joystick_button_name(
		arg1: *mut ALLEGRO_JOYSTICK, buttonn: c_int,
	) -> *const c_char;
	pub fn al_get_joystick_state(
		arg1: *mut ALLEGRO_JOYSTICK, ret_state: *mut ALLEGRO_JOYSTICK_STATE,
	);
	pub fn al_get_joystick_event_source() -> *mut ALLEGRO_EVENT_SOURCE;
}

#[cfg(all(feature = "unstable", allegro_5_2_11))]
unsafe extern "C" {
	pub fn al_get_joystick_guid(joy: *mut ALLEGRO_JOYSTICK) -> ALLEGRO_JOYSTICK_GUID;
	pub fn al_get_joystick_type(joy: *mut ALLEGRO_JOYSTICK) -> c_int;
	pub fn al_set_joystick_mappings(filename: *const c_char) -> c_bool;
	pub fn al_set_joystick_mappings_f(file: *mut ALLEGRO_FILE) -> c_bool;
}
