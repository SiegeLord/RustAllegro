use std::libc::*;
use std::cast;
use ffi::*;
use internal::events::*;
use std::ptr;

pub mod external
{
	pub use super::Timer;
}

pub struct Timer
{
	priv allegro_timer: *mut ALLEGRO_TIMER,
	priv event_source: EventSource
}

impl Timer
{
	pub fn start(&self)
	{
		unsafe
		{
			al_start_timer(self.allegro_timer);
		}
	}

	pub fn stop(&self)
	{
		unsafe
		{
			al_stop_timer(self.allegro_timer);
		}
	}

	pub fn is_started(&self) -> bool
	{
		unsafe
		{
			al_get_timer_started(cast::transmute(self.allegro_timer)) != 0
		}
	}

	pub fn get_speed(&self) -> f64
	{
		unsafe
		{
			al_get_timer_speed(cast::transmute(self.allegro_timer)) as f64
		}
	}

	pub fn set_speed(&self, speed_secs: f64)
	{
		unsafe
		{
			al_set_timer_speed(self.allegro_timer, speed_secs as c_double);
		}
	}

	pub fn get_count(&self) -> i64
	{
		unsafe
		{
			al_get_timer_count(cast::transmute(self.allegro_timer)) as i64
		}
	}

	pub fn set_count(&self, count: i64)
	{
		unsafe
		{
			al_set_timer_count(self.allegro_timer, count as int64_t);
		}
	}

	pub fn add_count(&self, diff: i64)
	{
		unsafe
		{
			al_add_timer_count(self.allegro_timer, diff as int64_t);
		}
	}

	pub fn get_event_source<'l>(&'l self) -> &'l EventSource
	{
		&'l self.event_source
	}
}

impl Drop for Timer
{
	fn drop(&mut self)
	{
		unsafe
		{
			al_destroy_timer(self.allegro_timer);
		}
	}
}

pub fn new_timer(speed_secs: f64) -> Option<Timer>
{
	unsafe
	{
		let t = al_create_timer(speed_secs as c_double);
		if !ptr::is_null(t)
		{
			Some(Timer{ allegro_timer: t, event_source: new_event_source_ref(al_get_timer_event_source(t)) })
		}
		else
		{
			None
		}
	}
}
