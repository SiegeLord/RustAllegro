// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

pub use self::Event::*;
use crate::core::Core;
use crate::joystick::{Joystick, JoystickButton, JoystickStick};
use crate::keycodes::{KeyCode, KeyModifier};

use allegro_sys::*;
use libc::*;
use std::any::Any;
use std::marker::PhantomData;
use std::mem;
use std::ptr;
use std::rc::Rc;

const USER_EVENT_TYPE: u32 = 10556114;

pub struct EventQueue
{
	allegro_queue: *mut ALLEGRO_EVENT_QUEUE,
}

impl EventQueue
{
	pub unsafe fn wrap(queue: *mut ALLEGRO_EVENT_QUEUE) -> EventQueue
	{
		EventQueue {
			allegro_queue: queue,
		}
	}

	pub fn new(_: &Core) -> Result<EventQueue, ()>
	{
		unsafe {
			let q = al_create_event_queue();
			if q.is_null()
			{
				Err(())
			}
			else
			{
				Ok(EventQueue::wrap(q))
			}
		}
	}

	pub fn register_event_source<E: EventSourceLike>(&self, mut src: E)
	{
		unsafe {
			al_register_event_source(self.allegro_queue, src.get_event_source());
		}
	}

	pub fn is_empty(&self) -> bool
	{
		unsafe { al_is_event_queue_empty(self.allegro_queue) != 0 }
	}

	pub fn get_next_event(&self) -> Event
	{
		let mut e = ALLEGRO_EVENT::new();
		unsafe {
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
		unsafe {
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
		unsafe { al_drop_next_event(self.allegro_queue) != 0 }
	}

	pub fn flush(&self)
	{
		unsafe {
			al_flush_event_queue(self.allegro_queue);
		}
	}

	pub fn wait_for_event(&self) -> Event
	{
		let mut e = ALLEGRO_EVENT::new();
		unsafe {
			al_wait_for_event(self.allegro_queue, &mut e);
		}
		Event::from_allegro_event(&mut e)
	}

	pub fn wait_for_event_timed(&self, secs: f64) -> Event
	{
		let mut e = ALLEGRO_EVENT::new();
		unsafe {
			al_wait_for_event_timed(self.allegro_queue, &mut e, secs as c_float);
		}
		Event::from_allegro_event(&mut e)
	}
}

impl Iterator for EventQueue
{
	type Item = Event;
	fn next(&mut self) -> Option<Event>
	{
		match self.get_next_event()
		{
			NoEvent => None,
			e => Some(e),
		}
	}
}

// Not Send just because of the marker
impl Drop for EventQueue
{
	fn drop(&mut self)
	{
		unsafe {
			al_destroy_event_queue(self.allegro_queue);
		}
	}
}

pub trait EventSourceLike
{
	fn get_event_source(&mut self) -> *mut ALLEGRO_EVENT_SOURCE;
}

#[derive(Copy, Clone)]
pub struct EventSource<'l>
{
	allegro_source: *mut ALLEGRO_EVENT_SOURCE,
	marker: PhantomData<&'l ()>,
}

impl<'m> EventSource<'m>
{
	pub unsafe fn wrap<'l>(source: *mut ALLEGRO_EVENT_SOURCE) -> EventSource<'l>
	{
		EventSource {
			allegro_source: source,
			marker: PhantomData,
		}
	}
}

impl<'m> EventSourceLike for EventSource<'m>
{
	fn get_event_source(&mut self) -> *mut ALLEGRO_EVENT_SOURCE
	{
		self.allegro_source
	}
}

pub struct UserEventSource
{
	allegro_source: ALLEGRO_EVENT_SOURCE,
}

impl UserEventSource
{
	pub fn new(_: &Core) -> UserEventSource
	{
		let mut ret = UserEventSource {
			allegro_source: ALLEGRO_EVENT_SOURCE { __pad: [0; 32] },
		};
		unsafe {
			al_init_user_event_source(&mut ret.allegro_source);
		}
		return ret;
	}

	pub fn emit<T: Send + 'static>(&mut self, data: T) -> bool
	{
		let rc: Rc<dyn Any> = Rc::new(data);
		let data = Box::into_raw(Box::new(rc));
		let e = ALLEGRO_USER_EVENT {
			_type: USER_EVENT_TYPE,
			source: &mut self.allegro_source,
			timestamp: 0.,
			_internal__descr: ptr::null_mut(),
			data1: unsafe { mem::transmute(data) },
			data2: 0,
			data3: 0,
			data4: 0,
		};
		unsafe {
			al_emit_user_event(
				&mut self.allegro_source,
				mem::transmute(&e),
				event_destructor,
			) != 0
		}
	}
}

impl<'m> EventSourceLike for &mut UserEventSource
{
	fn get_event_source(&mut self) -> *mut ALLEGRO_EVENT_SOURCE
	{
		&mut self.allegro_source
	}
}

extern "C" fn event_destructor(event: *mut ALLEGRO_USER_EVENT)
{
	unsafe {
		std::panic::catch_unwind(|| {
			let ptr: *mut Rc<dyn Any> = mem::transmute((*event).data1 as *mut c_void);
			let _ = Box::from_raw(ptr);
		})
		.ok();
	}
}

impl Drop for UserEventSource
{
	fn drop(&mut self)
	{
		unsafe {
			al_destroy_user_event_source(&mut self.allegro_source);
		}
	}
}

#[derive(Debug)]
pub enum Event
{
	NoEvent,
	DisplayClose
	{
		source: *mut ALLEGRO_EVENT_SOURCE,
		timestamp: f64,
	},
	DisplaySwitchIn
	{
		source: *mut ALLEGRO_EVENT_SOURCE,
		timestamp: f64,
	},
	DisplaySwitchOut
	{
		source: *mut ALLEGRO_EVENT_SOURCE,
		timestamp: f64,
	},
	DisplayResize
	{
		source: *mut ALLEGRO_EVENT_SOURCE,
		x: i32,
		y: i32,
		width: i32,
		height: i32,
		timestamp: f64,
	},
	JoystickAxes
	{
		source: *mut ALLEGRO_EVENT_SOURCE,
		timestamp: f64,
		joystick: Joystick,
		stick: JoystickStick,
		axis: i32,
		pos: f32,
	},
	JoystickButtonDown
	{
		source: *mut ALLEGRO_EVENT_SOURCE,
		timestamp: f64,
		joystick: Joystick,
		button: JoystickButton,
	},
	JoystickButtonUp
	{
		source: *mut ALLEGRO_EVENT_SOURCE,
		timestamp: f64,
		joystick: Joystick,
		button: JoystickButton,
	},
	JoystickConfiguration
	{
		source: *mut ALLEGRO_EVENT_SOURCE,
		timestamp: f64,
	},
	KeyDown
	{
		source: *mut ALLEGRO_EVENT_SOURCE,
		timestamp: f64,
		keycode: KeyCode,
		display: *mut ALLEGRO_DISPLAY,
	},
	KeyUp
	{
		source: *mut ALLEGRO_EVENT_SOURCE,
		timestamp: f64,
		keycode: KeyCode,
		display: *mut ALLEGRO_DISPLAY,
	},
	KeyChar
	{
		source: *mut ALLEGRO_EVENT_SOURCE,
		timestamp: f64,
		keycode: KeyCode,
		display: *mut ALLEGRO_DISPLAY,
		unichar: char,
		repeat: bool,
		modifiers: KeyModifier,
	},
	MouseAxes
	{
		source: *mut ALLEGRO_EVENT_SOURCE,
		timestamp: f64,
		x: i32,
		y: i32,
		z: i32,
		w: i32,
		dx: i32,
		dy: i32,
		dz: i32,
		dw: i32,
		display: *mut ALLEGRO_DISPLAY,
	},
	MouseButtonDown
	{
		source: *mut ALLEGRO_EVENT_SOURCE,
		timestamp: f64,
		x: i32,
		y: i32,
		z: i32,
		w: i32,
		button: u32,
		display: *mut ALLEGRO_DISPLAY,
	},
	MouseButtonUp
	{
		source: *mut ALLEGRO_EVENT_SOURCE,
		timestamp: f64,
		x: i32,
		y: i32,
		z: i32,
		w: i32,
		button: u32,
		display: *mut ALLEGRO_DISPLAY,
	},
	MouseWarped
	{
		source: *mut ALLEGRO_EVENT_SOURCE,
		timestamp: f64,
		x: i32,
		y: i32,
		z: i32,
		w: i32,
		dx: i32,
		dy: i32,
		dz: i32,
		dw: i32,
		display: *mut ALLEGRO_DISPLAY,
	},
	MouseEnterDisplay
	{
		source: *mut ALLEGRO_EVENT_SOURCE,
		timestamp: f64,
		x: i32,
		y: i32,
		z: i32,
		w: i32,
		display: *mut ALLEGRO_DISPLAY,
	},
	MouseLeaveDisplay
	{
		source: *mut ALLEGRO_EVENT_SOURCE,
		timestamp: f64,
		x: i32,
		y: i32,
		z: i32,
		w: i32,
		display: *mut ALLEGRO_DISPLAY,
	},
	TimerTick
	{
		source: *mut ALLEGRO_EVENT_SOURCE,
		timestamp: f64,
		count: i64,
	},
	User
	{
		source: *mut ALLEGRO_EVENT_SOURCE,
		timestamp: f64,
		data: Rc<dyn Any>,
	},
}

impl Event
{
	fn from_allegro_event(e: &mut ALLEGRO_EVENT) -> Event
	{
		unsafe {
			let src = (*e.any()).source;
			let ts = (*e.any()).timestamp as f64;
			match *e._type() as u32
			{
				ALLEGRO_EVENT_DISPLAY_CLOSE => DisplayClose {
					source: src,
					timestamp: ts,
				},
				ALLEGRO_EVENT_DISPLAY_SWITCH_IN => DisplaySwitchIn {
					source: src,
					timestamp: ts,
				},
				ALLEGRO_EVENT_DISPLAY_SWITCH_OUT => DisplaySwitchOut {
					source: src,
					timestamp: ts,
				},
				ALLEGRO_EVENT_DISPLAY_RESIZE =>
				{
					let a = *e.display();
					DisplayResize {
						source: src,
						x: a.x as i32,
						y: a.y as i32,
						width: a.width as i32,
						height: a.height as i32,
						timestamp: ts,
					}
				}
				ALLEGRO_EVENT_JOYSTICK_AXIS =>
				{
					let a = *e.joystick();
					let joystick = Joystick::wrap(a.id);
					let stick = JoystickStick::from_allegro(&joystick, a.stick);
					JoystickAxes {
						source: src,
						timestamp: ts,
						joystick: joystick,
						stick: stick,
						axis: a.axis as i32,
						pos: a.pos as f32,
					}
				}
				ALLEGRO_EVENT_JOYSTICK_BUTTON_DOWN =>
				{
					let a = *e.joystick();
					let joystick = Joystick::wrap(a.id);
					let button = JoystickButton::from_allegro(&joystick, a.button);
					JoystickButtonDown {
						source: src,
						timestamp: ts,
						joystick: joystick,
						button: button,
					}
				}
				ALLEGRO_EVENT_JOYSTICK_BUTTON_UP =>
				{
					let a = *e.joystick();
					let joystick = Joystick::wrap(a.id);
					let button = JoystickButton::from_allegro(&joystick, a.button);
					JoystickButtonUp {
						source: src,
						timestamp: ts,
						joystick: joystick,
						button: button,
					}
				}
				ALLEGRO_EVENT_JOYSTICK_CONFIGURATION => JoystickConfiguration {
					source: src,
					timestamp: ts,
				},
				ALLEGRO_EVENT_KEY_DOWN =>
				{
					let k = *e.keyboard();
					KeyDown {
						source: src,
						timestamp: ts,
						keycode: KeyCode::from_allegro_key(k.keycode),
						display: k.display,
					}
				}
				ALLEGRO_EVENT_KEY_UP =>
				{
					let k = *e.keyboard();
					KeyUp {
						source: src,
						timestamp: ts,
						keycode: KeyCode::from_allegro_key(k.keycode),
						display: k.display,
					}
				}
				ALLEGRO_EVENT_KEY_CHAR =>
				{
					let k = *e.keyboard();
					KeyChar {
						source: src,
						timestamp: ts,
						keycode: KeyCode::from_allegro_key(k.keycode),
						display: k.display,
						unichar: mem::transmute(k.unichar),
						repeat: k.repeat != 0,
						modifiers: mem::transmute(k.modifiers),
					}
				}
				ALLEGRO_EVENT_MOUSE_AXES =>
				{
					let m = *e.mouse();
					MouseAxes {
						source: src,
						timestamp: ts,
						x: m.x,
						y: m.y,
						z: m.z,
						w: m.w,
						dx: m.dx,
						dy: m.dy,
						dz: m.dz,
						dw: m.dw,
						display: m.display,
					}
				}
				ALLEGRO_EVENT_MOUSE_BUTTON_DOWN =>
				{
					let m = *e.mouse();
					MouseButtonDown {
						source: src,
						timestamp: ts,
						x: m.x,
						y: m.y,
						z: m.z,
						w: m.w,
						button: m.button,
						display: m.display,
					}
				}
				ALLEGRO_EVENT_MOUSE_BUTTON_UP =>
				{
					let m = *e.mouse();
					MouseButtonUp {
						source: src,
						timestamp: ts,
						x: m.x,
						y: m.y,
						z: m.z,
						w: m.w,
						button: m.button,
						display: m.display,
					}
				}
				ALLEGRO_EVENT_MOUSE_WARPED =>
				{
					let m = *e.mouse();
					MouseWarped {
						source: src,
						timestamp: ts,
						x: m.x,
						y: m.y,
						z: m.z,
						w: m.w,
						dx: m.dx,
						dy: m.dy,
						dz: m.dz,
						dw: m.dw,
						display: m.display,
					}
				}
				ALLEGRO_EVENT_MOUSE_ENTER_DISPLAY =>
				{
					let m = *e.mouse();
					MouseEnterDisplay {
						source: src,
						timestamp: ts,
						x: m.x,
						y: m.y,
						z: m.z,
						w: m.w,
						display: m.display,
					}
				}
				ALLEGRO_EVENT_MOUSE_LEAVE_DISPLAY =>
				{
					let m = *e.mouse();
					MouseLeaveDisplay {
						source: src,
						timestamp: ts,
						x: m.x,
						y: m.y,
						z: m.z,
						w: m.w,
						display: m.display,
					}
				}
				ALLEGRO_EVENT_TIMER =>
				{
					let t = *e.timer();
					TimerTick {
						source: src,
						timestamp: ts,
						count: t.count as i64,
					}
				}
				USER_EVENT_TYPE =>
				{
					let mut u = *e.user();
					let ptr: *mut Rc<dyn Any> = mem::transmute(u.data1 as *mut c_void);
					let data = (*ptr).clone();
					al_unref_user_event(&mut u);
					User {
						source: src,
						timestamp: ts,
						data: data,
					}
				}
				_ => NoEvent,
			}
		}
	}
}
