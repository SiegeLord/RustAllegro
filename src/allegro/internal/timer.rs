// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;
use std::option::Some;
use std::mem;
use ffi::*;
use internal::events::*;

pub struct Timer
{
	allegro_timer: *mut ALLEGRO_TIMER,
	event_source: EventSource
}

impl Timer
{
	pub fn new(speed_secs: f64) -> Option<Timer>
	{
		unsafe
		{
			let t = al_create_timer(speed_secs as c_double);
			if !t.is_null()
			{
				Some(Timer{ allegro_timer: t, event_source: new_event_source_ref(al_get_timer_event_source(t)) })
			}
			else
			{
				None
			}
		}
	}

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
			al_get_timer_started(mem::transmute(self.allegro_timer)) != 0
		}
	}

	pub fn get_speed(&self) -> f64
	{
		unsafe
		{
			al_get_timer_speed(mem::transmute(self.allegro_timer)) as f64
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
			al_get_timer_count(mem::transmute(self.allegro_timer)) as i64
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
		&self.event_source
	}
}

// Not Send just because of the marker in the event source
#[unsafe_destructor]
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

impl ::internal::core::Core
{
	pub fn create_timer(&self, speed_secs: f64) -> Option<Timer>
	{
		Timer::new(speed_secs)
	}
}
