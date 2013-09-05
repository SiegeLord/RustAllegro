use std::libc::*;
use rust_util::c_bool;

use ffi::events::ALLEGRO_EVENT_SOURCE;

pub struct ALLEGRO_JOYSTICK;

pub struct ALLEGRO_JOYSTICK_STATE
{
	stick: [Stick, ..8u],
	button: [c_int, ..32u],
}

pub struct Stick
{
	axis: [c_float, ..3u],
}

pub static ALLEGRO_JOYFLAG_DIGITAL: c_uint = 1;
pub static ALLEGRO_JOYFLAG_ANALOGUE: c_uint = 2;

externfn!(fn al_install_joystick() -> c_uchar)
externfn!(fn al_uninstall_joystick())
externfn!(fn al_is_joystick_installed() -> c_uchar)
externfn!(fn al_reconfigure_joysticks() -> c_uchar)
externfn!(fn al_get_num_joysticks() -> c_int)
externfn!(fn al_get_joystick(joyn: c_int) -> *mut ALLEGRO_JOYSTICK)
externfn!(fn al_release_joystick(arg1: *mut ALLEGRO_JOYSTICK))
externfn!(fn al_get_joystick_active(arg1: *mut ALLEGRO_JOYSTICK) -> c_bool)
externfn!(fn al_get_joystick_name(arg1: *mut ALLEGRO_JOYSTICK) -> *c_schar)
externfn!(fn al_get_joystick_num_sticks(arg1: *mut ALLEGRO_JOYSTICK) -> c_int)
externfn!(fn al_get_joystick_stick_flags(arg1: *mut ALLEGRO_JOYSTICK, stick: c_int) -> c_int)
externfn!(fn al_get_joystick_stick_name(arg1: *mut ALLEGRO_JOYSTICK, stick: c_int) -> *c_schar)
externfn!(fn al_get_joystick_num_axes(arg1: *mut ALLEGRO_JOYSTICK, stick: c_int) -> c_int)
externfn!(fn al_get_joystick_axis_name(arg1: *mut ALLEGRO_JOYSTICK, stick: c_int, axis: c_int) -> *c_schar)
externfn!(fn al_get_joystick_num_buttons(arg1: *mut ALLEGRO_JOYSTICK) -> c_int)
externfn!(fn al_get_joystick_button_name(arg1: *mut ALLEGRO_JOYSTICK, buttonn: c_int) -> *c_schar)
externfn!(fn al_get_joystick_state(arg1: *mut ALLEGRO_JOYSTICK, ret_state: *mut ALLEGRO_JOYSTICK_STATE))
externfn!(fn al_get_joystick_event_source() -> *mut ALLEGRO_EVENT_SOURCE)
