use std::cast;
use std::libc::*;
use std::kinds::marker::NoSend;

use internal::keycodes::*;
use ffi::*;

pub mod external
{
	pub use super::{Event, NoEvent, DisplayClose, KeyDown, KeyUp, KeyChar,
	                MouseAxes, MouseButtonDown, MouseButtonUp, MouseWarped,
	                MouseEnterDisplay, MouseLeaveDisplay, TimerTick,
	                EventQueue, EventSource};
}

pub struct EventQueue
{
	allegro_queue: *mut ALLEGRO_EVENT_QUEUE,
	no_send_marker: NoSend,
}

impl EventQueue
{
	pub fn new() -> Option<EventQueue>
	{
		unsafe
		{
			let q = al_create_event_queue();
			if q.is_null()
			{
				None
			}
			else
			{
				Some(EventQueue{ allegro_queue: q, no_send_marker: NoSend })
			}
		}
	}

	pub fn register_event_source(&self, src: &EventSource)
	{
		unsafe
		{
			al_register_event_source(self.allegro_queue, src.allegro_source);
		}
	}

	pub fn is_empty(&self) -> bool
	{
		unsafe
		{
			al_is_event_queue_empty(self.allegro_queue) != 0
		}
	}

	pub fn get_next_event(&self) -> Event
	{
		let mut e = ALLEGRO_EVENT::new();
		unsafe
		{
			if al_get_next_event(self.allegro_queue, &mut e) != 0
			{
				Event::from_allegro_event(&mut e)
			}
			else
			{
				NoEvent
			}
		}
	}

	pub fn peek_next_event(&self) -> Event
	{
		let mut e = ALLEGRO_EVENT::new();
		unsafe
		{
			if al_peek_next_event(self.allegro_queue, &mut e) != 0
			{
				Event::from_allegro_event(&mut e)
			}
			else
			{
				NoEvent
			}
		}
	}

	pub fn drop_next_event(&self) -> bool
	{
		unsafe
		{
			al_drop_next_event(self.allegro_queue) != 0
		}
	}

	pub fn flush(&self)
	{
		unsafe
		{
			al_flush_event_queue(self.allegro_queue);
		}
	}

	pub fn wait_for_event(&self) -> Event
	{
		let mut e = ALLEGRO_EVENT::new();
		unsafe
		{
			al_wait_for_event(self.allegro_queue, &mut e);
		}
		Event::from_allegro_event(&mut e)
	}

	pub fn wait_for_event_timed(&self, secs: f64) -> Event
	{
		let mut e = ALLEGRO_EVENT::new();
		unsafe
		{
			al_wait_for_event_timed(self.allegro_queue, &mut e, secs as c_float);
		}
		Event::from_allegro_event(&mut e)
	}
}

impl Iterator<Event> for EventQueue
{
	fn next(&mut self) -> Option<Event>
	{
		match self.get_next_event()
		{
			NoEvent => None,
			e => Some(e)
		}
	}
}

// Not Send just because of the marker
#[unsafe_destructor]
impl Drop for EventQueue
{
	fn drop(&mut self)
	{
		unsafe
		{
			al_destroy_event_queue(self.allegro_queue);
		}
	}
}

pub struct EventSource
{
	allegro_source: *mut ALLEGRO_EVENT_SOURCE,
	no_send_marker: NoSend,
}

impl EventSource
{
	pub fn get_event_source(&self) -> *mut ALLEGRO_EVENT_SOURCE
	{
		self.allegro_source
	}
}

pub enum Event
{
	NoEvent,
	DisplayClose
	{
		pub source: *mut ALLEGRO_EVENT_SOURCE,
		pub timestamp: f64
	},
	JoystickAxes
	{
		pub source: *mut ALLEGRO_EVENT_SOURCE,
		pub timestamp: f64,
		pub id: *mut ALLEGRO_JOYSTICK,
		pub stick: i32,
		pub axis: i32,
		pub pos: f32
	},
	JoystickButtonDown
	{
		pub source: *mut ALLEGRO_EVENT_SOURCE,
		pub timestamp: f64,
		pub id: *mut ALLEGRO_JOYSTICK,
		pub button: i32
	},
	JoystickButtonUp
	{
		pub source: *mut ALLEGRO_EVENT_SOURCE,
		pub timestamp: f64,
		pub id: *mut ALLEGRO_JOYSTICK,
		pub button: i32
	},
	JoystickConfiguration
	{
		pub source: *mut ALLEGRO_EVENT_SOURCE,
		pub timestamp: f64
	},
	KeyDown
	{
		pub source: *mut ALLEGRO_EVENT_SOURCE,
		pub timestamp: f64,
		pub keycode: key::KeyCode,
		pub display: *mut ALLEGRO_DISPLAY
	},
	KeyUp
	{
		pub source: *mut ALLEGRO_EVENT_SOURCE,
		pub timestamp: f64,
		pub keycode: key::KeyCode,
		pub display: *mut ALLEGRO_DISPLAY
	},
	KeyChar
	{
		pub source: *mut ALLEGRO_EVENT_SOURCE,
		pub timestamp: f64,
		pub keycode: key::KeyCode,
		pub display: *mut ALLEGRO_DISPLAY,
		pub unichar: char,
		pub repeat: bool,
		pub modifiers: KeyModifier
	},
	MouseAxes
	{
		pub source: *mut ALLEGRO_EVENT_SOURCE,
		pub timestamp: f64,
		pub x: i32,
		pub y: i32,
		pub z: i32,
		pub w: i32,
		pub dx: i32,
		pub dy: i32,
		pub dz: i32,
		pub dw: i32,
		pub display: *mut ALLEGRO_DISPLAY
	},
	MouseButtonDown
	{
		pub source: *mut ALLEGRO_EVENT_SOURCE,
		pub timestamp: f64,
		pub x: i32,
		pub y: i32,
		pub z: i32,
		pub w: i32,
		pub button: u32,
		pub display: *mut ALLEGRO_DISPLAY
	},
	MouseButtonUp
	{
		pub source: *mut ALLEGRO_EVENT_SOURCE,
		pub timestamp: f64,
		pub x: i32,
		pub y: i32,
		pub z: i32,
		pub w: i32,
		pub button: u32,
		pub display: *mut ALLEGRO_DISPLAY
	},
	MouseWarped
	{
		pub source: *mut ALLEGRO_EVENT_SOURCE,
		pub timestamp: f64,
		pub x: i32,
		pub y: i32,
		pub z: i32,
		pub w: i32,
		pub dx: i32,
		pub dy: i32,
		pub dz: i32,
		pub dw: i32,
		pub display: *mut ALLEGRO_DISPLAY
	},
	MouseEnterDisplay
	{
		pub source: *mut ALLEGRO_EVENT_SOURCE,
		pub timestamp: f64,
		pub x: i32,
		pub y: i32,
		pub z: i32,
		pub w: i32,
		pub display: *mut ALLEGRO_DISPLAY
	},
	MouseLeaveDisplay
	{
		pub source: *mut ALLEGRO_EVENT_SOURCE,
		pub timestamp: f64,
		pub x: i32,
		pub y: i32,
		pub z: i32,
		pub w: i32,
		pub display: *mut ALLEGRO_DISPLAY
	},
	TimerTick
	{
		pub source: *mut ALLEGRO_EVENT_SOURCE,
		pub timestamp: f64,
		pub count: i64
	},
}

impl Event
{
	fn from_allegro_event(e: &mut ALLEGRO_EVENT) -> Event
	{
		unsafe
		{
			let src = (*e.any()).source;
			let ts = (*e.any()).timestamp as f64;
			match *e._type() as u32
			{
				ALLEGRO_EVENT_DISPLAY_CLOSE =>
				{
					DisplayClose{source: src, timestamp: ts}
				},
				ALLEGRO_EVENT_JOYSTICK_AXIS =>
				{
					let a = *e.joystick();
					JoystickAxes{source: src, timestamp: ts, id: a.id, stick: a.stick as i32, axis: a.axis as i32, pos: a.pos as f32}
				},
				ALLEGRO_EVENT_JOYSTICK_BUTTON_DOWN =>
				{
					let a = *e.joystick();
					JoystickButtonDown{source: src, timestamp: ts, id: a.id, button: a.button as i32}
				},
				ALLEGRO_EVENT_JOYSTICK_BUTTON_UP =>
				{
					let a = *e.joystick();
					JoystickButtonUp{source: src, timestamp: ts, id: a.id, button: a.button as i32}
				},
				ALLEGRO_EVENT_JOYSTICK_CONFIGURATION =>
				{
					JoystickConfiguration{source: src, timestamp: ts}
				},
				ALLEGRO_EVENT_KEY_DOWN =>
				{
					let k = *e.keyboard();
					KeyDown{source: src, timestamp: ts, keycode: key::KeyCode::from_allegro_key(k.keycode), display: k.display}
				},
				ALLEGRO_EVENT_KEY_UP =>
				{
					let k = *e.keyboard();
					KeyUp{source: src, timestamp: ts, keycode: key::KeyCode::from_allegro_key(k.keycode), display: k.display}
				},
				ALLEGRO_EVENT_KEY_CHAR =>
				{
					let k = *e.keyboard();
					KeyChar{source: src, timestamp: ts, keycode: key::KeyCode::from_allegro_key(k.keycode), display: k.display,
					        unichar: cast::transmute(k.unichar), repeat: k.repeat != 0, modifiers: cast::transmute(k.modifiers)}
				},
				ALLEGRO_EVENT_MOUSE_AXES =>
				{
					let m = *e.mouse();
					MouseAxes{source: src, timestamp: ts, x: m.x, y: m.y, z: m.z, w: m.w, dx: m.dx, dy: m.dy, dz: m.dz, dw: m.dw, display: m.display}
				},
				ALLEGRO_EVENT_MOUSE_BUTTON_DOWN =>
				{
					let m = *e.mouse();
					MouseButtonDown{source: src, timestamp: ts, x: m.x, y: m.y, z: m.z, w: m.w, button: m.button, display: m.display}
				},
				ALLEGRO_EVENT_MOUSE_BUTTON_UP =>
				{
					let m = *e.mouse();
					MouseButtonUp{source: src, timestamp: ts, x: m.x, y: m.y, z: m.z, w: m.w, button: m.button, display: m.display}
				},
				ALLEGRO_EVENT_MOUSE_WARPED =>
				{
					let m = *e.mouse();
					MouseWarped{source: src, timestamp: ts, x: m.x, y: m.y, z: m.z, w: m.w, dx: m.dx, dy: m.dy, dz: m.dz, dw: m.dw, display: m.display}
				},
				ALLEGRO_EVENT_MOUSE_ENTER_DISPLAY =>
				{
					let m = *e.mouse();
					MouseEnterDisplay{source: src, timestamp: ts, x: m.x, y: m.y, z: m.z, w: m.w, display: m.display}
				},
				ALLEGRO_EVENT_MOUSE_LEAVE_DISPLAY =>
				{
					let m = *e.mouse();
					MouseEnterDisplay{source: src, timestamp: ts, x: m.x, y: m.y, z: m.z, w: m.w, display: m.display}
				},
				ALLEGRO_EVENT_TIMER =>
				{
					let t = *e.timer();
					TimerTick{source: src, timestamp: ts, count: t.count as i64}
				},
				_ => NoEvent
			}
		}
	}
}

pub fn new_event_source_ref(source: *mut ALLEGRO_EVENT_SOURCE) -> EventSource
{
	EventSource{ allegro_source: source, no_send_marker: NoSend }
}

impl ::internal::core::Core
{
	pub fn create_event_queue(&self) -> Option<EventQueue>
	{
		EventQueue::new()
	}
}
