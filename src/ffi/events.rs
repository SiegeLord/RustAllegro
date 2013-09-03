use std::libc::*;

pub struct ALLEGRO_EVENT_SOURCE
{
	priv __pad: [c_int, ..32]
}

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
