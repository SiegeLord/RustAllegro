use std::cast;
use std::libc::*;

use keycodes::*;
use ffi::*;

pub struct EventQueue
{
	priv allegro_queue: *mut ALLEGRO_EVENT_QUEUE
}

impl EventQueue
{
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
	priv allegro_source: *mut ALLEGRO_EVENT_SOURCE
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
		source: *mut ALLEGRO_EVENT_SOURCE,
		timestamp: f64
	},
	KeyDown
	{
		source: *mut ALLEGRO_EVENT_SOURCE,
		timestamp: f64,
		keycode: key::KeyCode,
		display: *mut ALLEGRO_DISPLAY
	},
	KeyUp
	{
		source: *mut ALLEGRO_EVENT_SOURCE,
		timestamp: f64,
		keycode: key::KeyCode,
		display: *mut ALLEGRO_DISPLAY
	},
	KeyChar
	{
		source: *mut ALLEGRO_EVENT_SOURCE,
		timestamp: f64,
		keycode: key::KeyCode,
		display: *mut ALLEGRO_DISPLAY,
		unichar: char,
		repeat: bool,
		modifiers: KeyModifier
	},
	TimerTick
	{
		source: *mut ALLEGRO_EVENT_SOURCE,
		timestamp: f64,
		count: i64
	},
}

impl Event
{
	fn from_allegro_event(e: &mut ALLEGRO_EVENT) -> Event
	{
		unsafe
		{
			match *e._type() as u32
			{
				ALLEGRO_EVENT_DISPLAY_CLOSE =>
				{
					DisplayClose{ source: cast::transmute((*e.display()).source), timestamp: (*e.display()).timestamp as f64}
				},
				ALLEGRO_EVENT_KEY_DOWN =>
				{
					let k = *e.keyboard();
					KeyDown{ source: cast::transmute(k.source), timestamp: k.timestamp as f64, keycode: key::KeyCode::from_allegro_key(k.keycode), display: k.display }
				},
				ALLEGRO_EVENT_KEY_UP =>
				{
					let k = *e.keyboard();
					KeyUp{ source: cast::transmute(k.source), timestamp: k.timestamp as f64, keycode: key::KeyCode::from_allegro_key(k.keycode), display: k.display }
				},
				ALLEGRO_EVENT_KEY_CHAR =>
				{
					let k = *e.keyboard();
					KeyChar{ source: cast::transmute(k.source), timestamp: k.timestamp as f64, keycode: key::KeyCode::from_allegro_key(k.keycode), display: k.display,
					         unichar: cast::transmute(k.unichar), repeat: k.repeat != 0, modifiers: cast::transmute(k.modifiers) }
				},
				ALLEGRO_EVENT_TIMER =>
				{
					let t = *e.timer();
					TimerTick{ source: cast::transmute(t.source), timestamp: t.timestamp as f64, count: t.count as i64 }
				},
				_ => NoEvent
			}
		}
	}
}

mod private
{
	use super::*;
	use std::ptr;

	use ffi::*;

	pub fn new_event_source_ref(source: *mut ALLEGRO_EVENT_SOURCE) -> super::EventSource
	{
		super::EventSource{ allegro_source: source }
	}

	pub fn new_queue() -> Option<EventQueue>
	{
		unsafe
		{
			let q = al_create_event_queue();
			if ptr::is_null(q)
			{
				None
			}
			else
			{
				Some(EventQueue{ allegro_queue: q })
			}
		}
	}
}
