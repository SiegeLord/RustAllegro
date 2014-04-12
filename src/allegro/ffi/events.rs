use libc::*;
use std::cast;

use rust_util::c_bool;

use ffi::altime::ALLEGRO_TIMEOUT;
use ffi::display::ALLEGRO_DISPLAY;
use ffi::keyboard::ALLEGRO_KEYBOARD;
use ffi::mouse::ALLEGRO_MOUSE;
use ffi::joystick::ALLEGRO_JOYSTICK;
use ffi::timer::ALLEGRO_TIMER;

pub struct ALLEGRO_EVENT_SOURCE
{
	pub __pad: [c_int, ..32]
}

pub type ALLEGRO_EVENT_TYPE = c_uint;

pub static ALLEGRO_EVENT_JOYSTICK_AXIS: u32 = 1;
pub static ALLEGRO_EVENT_JOYSTICK_BUTTON_DOWN: u32 = 2;
pub static ALLEGRO_EVENT_JOYSTICK_BUTTON_UP: u32 = 3;
pub static ALLEGRO_EVENT_JOYSTICK_CONFIGURATION: u32 = 4;
pub static ALLEGRO_EVENT_KEY_DOWN: u32 = 10;
pub static ALLEGRO_EVENT_KEY_CHAR: u32 = 11;
pub static ALLEGRO_EVENT_KEY_UP: u32 = 12;
pub static ALLEGRO_EVENT_MOUSE_AXES: u32 = 20;
pub static ALLEGRO_EVENT_MOUSE_BUTTON_DOWN: u32 = 21;
pub static ALLEGRO_EVENT_MOUSE_BUTTON_UP: u32 = 22;
pub static ALLEGRO_EVENT_MOUSE_ENTER_DISPLAY: u32 = 23;
pub static ALLEGRO_EVENT_MOUSE_LEAVE_DISPLAY: u32 = 24;
pub static ALLEGRO_EVENT_MOUSE_WARPED: u32 = 25;
pub static ALLEGRO_EVENT_TIMER: u32 = 30;
pub static ALLEGRO_EVENT_DISPLAY_EXPOSE: u32 = 40;
pub static ALLEGRO_EVENT_DISPLAY_RESIZE: u32 = 41;
pub static ALLEGRO_EVENT_DISPLAY_CLOSE: u32 = 42;
pub static ALLEGRO_EVENT_DISPLAY_LOST: u32 = 43;
pub static ALLEGRO_EVENT_DISPLAY_FOUND: u32 = 44;
pub static ALLEGRO_EVENT_DISPLAY_SWITCH_IN: u32 = 45;
pub static ALLEGRO_EVENT_DISPLAY_SWITCH_OUT: u32 = 46;
pub static ALLEGRO_EVENT_DISPLAY_ORIENTATION: u32 = 47;

pub struct ALLEGRO_ANY_EVENT
{
	pub _type: ALLEGRO_EVENT_TYPE,
	pub source: *mut ALLEGRO_EVENT_SOURCE,
	pub timestamp: c_double,
}

pub struct ALLEGRO_DISPLAY_EVENT
{
	pub _type: ALLEGRO_EVENT_TYPE,
	pub source: *mut ALLEGRO_DISPLAY,
	pub timestamp: c_double,
	pub x: c_int,
	pub y: c_int,
	pub width: c_int,
	pub height: c_int,
	pub orientation: c_int,
}

pub struct ALLEGRO_JOYSTICK_EVENT
{
	pub _type: ALLEGRO_EVENT_TYPE,
	pub source: *mut ALLEGRO_JOYSTICK,
	pub timestamp: c_double,
	pub id: *mut ALLEGRO_JOYSTICK,
	pub stick: c_int,
	pub axis: c_int,
	pub pos: c_float,
	pub button: c_int,
}

pub struct ALLEGRO_KEYBOARD_EVENT
{
	pub _type: ALLEGRO_EVENT_TYPE,
	pub source: *mut ALLEGRO_KEYBOARD,
	pub timestamp: c_double,
	pub display: *mut ALLEGRO_DISPLAY,
	pub keycode: c_int,
	pub unichar: c_int,
	pub modifiers: c_uint,
	pub repeat: c_bool,
}

pub struct ALLEGRO_MOUSE_EVENT
{
	pub _type: ALLEGRO_EVENT_TYPE,
	pub source: *mut ALLEGRO_MOUSE,
	pub timestamp: c_double,
	pub display: *mut ALLEGRO_DISPLAY,
	pub x: c_int,
	pub y: c_int,
	pub z: c_int,
	pub w: c_int,
	pub dx: c_int,
	pub dy: c_int,
	pub dz: c_int,
	pub dw: c_int,
	pub button: c_uint,
	pub pressure: c_float,
}

pub struct ALLEGRO_TIMER_EVENT
{
	pub _type: ALLEGRO_EVENT_TYPE,
	pub source: *mut ALLEGRO_TIMER,
	pub timestamp: c_double,
	pub count: int64_t,
	pub error: c_double,
}

pub struct ALLEGRO_USER_EVENT
{
	pub _type: ALLEGRO_EVENT_TYPE,
	pub source: *mut ALLEGRO_EVENT_SOURCE,
	pub timestamp: c_double,
	pub _internal__descr: *mut c_void,
	pub data1: intptr_t,
	pub data2: intptr_t,
	pub data3: intptr_t,
	pub data4: intptr_t,
}

pub struct ALLEGRO_EVENT
{
	pub data: [c_bool, ..72u],
}

impl ALLEGRO_EVENT
{
	pub fn new() -> ALLEGRO_EVENT
	{
		ALLEGRO_EVENT{ data: [0, ..72] }
	}

	pub fn _type(&mut self) -> *mut ALLEGRO_EVENT_TYPE
	{
		unsafe { cast::transmute(self) }
	}

	pub fn any(&mut self) -> *mut ALLEGRO_ANY_EVENT
	{
		unsafe { cast::transmute(self) }
	}

	pub fn display(&mut self) -> *mut ALLEGRO_DISPLAY_EVENT
	{
		unsafe { cast::transmute(self) }
	}

	pub fn joystick(&mut self) -> *mut ALLEGRO_JOYSTICK_EVENT
	{
		unsafe { cast::transmute(self) }
	}

	pub fn keyboard(&mut self) -> *mut ALLEGRO_KEYBOARD_EVENT
	{
		unsafe { cast::transmute(self) }
	}

	pub fn mouse(&mut self) -> *mut ALLEGRO_MOUSE_EVENT
	{
		unsafe { cast::transmute(self) }
	}

	pub fn timer(&mut self) -> *mut ALLEGRO_TIMER_EVENT
	{
		unsafe { cast::transmute(self) }
	}

	pub fn user(&mut self) -> *mut ALLEGRO_USER_EVENT
	{
		unsafe { cast::transmute(self) }
	}
}

opaque!(ALLEGRO_EVENT_QUEUE)

extern "C"
{
	pub fn al_init_user_event_source(arg1: *mut ALLEGRO_EVENT_SOURCE);
	pub fn al_destroy_user_event_source(arg1: *mut ALLEGRO_EVENT_SOURCE);
	pub fn al_emit_user_event(arg1: *mut ALLEGRO_EVENT_SOURCE, arg2: *mut ALLEGRO_EVENT, dtor: extern "C" fn(arg1: *mut ALLEGRO_USER_EVENT)) -> c_bool;
	pub fn al_unref_user_event(arg1: *mut ALLEGRO_USER_EVENT);
	pub fn al_set_event_source_data(arg1: *mut ALLEGRO_EVENT_SOURCE, data: intptr_t);
	pub fn al_get_event_source_data(arg1: *ALLEGRO_EVENT_SOURCE) -> intptr_t;
	pub fn al_create_event_queue() -> *mut ALLEGRO_EVENT_QUEUE;
	pub fn al_destroy_event_queue(arg1: *mut ALLEGRO_EVENT_QUEUE);
	pub fn al_register_event_source(arg1: *mut ALLEGRO_EVENT_QUEUE, arg2: *mut ALLEGRO_EVENT_SOURCE);
	pub fn al_unregister_event_source(arg1: *mut ALLEGRO_EVENT_QUEUE, arg2: *mut ALLEGRO_EVENT_SOURCE);
	pub fn al_is_event_queue_empty(arg1: *mut ALLEGRO_EVENT_QUEUE) -> c_bool;
	pub fn al_get_next_event(arg1: *mut ALLEGRO_EVENT_QUEUE, ret_event: *mut ALLEGRO_EVENT) -> c_bool;
	pub fn al_peek_next_event(arg1: *mut ALLEGRO_EVENT_QUEUE, ret_event: *mut ALLEGRO_EVENT) -> c_bool;
	pub fn al_drop_next_event(arg1: *mut ALLEGRO_EVENT_QUEUE) -> c_bool;
	pub fn al_flush_event_queue(arg1: *mut ALLEGRO_EVENT_QUEUE);
	pub fn al_wait_for_event(arg1: *mut ALLEGRO_EVENT_QUEUE, ret_event: *mut ALLEGRO_EVENT);
	pub fn al_wait_for_event_timed(arg1: *mut ALLEGRO_EVENT_QUEUE, ret_event: *mut ALLEGRO_EVENT, secs: c_float) -> c_bool;
	pub fn al_wait_for_event_until(queue: *mut ALLEGRO_EVENT_QUEUE, ret_event: *mut ALLEGRO_EVENT, timeout: *mut ALLEGRO_TIMEOUT) -> c_bool;
}
