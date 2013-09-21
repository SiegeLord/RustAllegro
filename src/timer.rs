use std::libc::*;
use std::ptr;
use std::cast;
use ffi::*;

struct Timer
{
	allegro_timer: *mut ALLEGRO_TIMER
}

impl Timer
{
	fn new(speed_secs: f64) -> Option<Timer>
	{
		unsafe
		{
			let t = al_create_timer(speed_secs as c_double);
			if ptr::is_null(t)
			{
				Some(Timer{ allegro_timer: t })
			}
			else
			{
				None
			}
		}
	}

	fn start(&self)
	{
		unsafe
		{
			al_start_timer(self.allegro_timer);
		}
	}

	fn stop(&self)
	{
		unsafe
		{
			al_stop_timer(self.allegro_timer);
		}
	}

	fn is_started(&self) -> bool
	{
		unsafe
		{
			al_get_timer_started(cast::transmute(self.allegro_timer)) != 0
		}
	}

	fn get_speed(&self) -> f64
	{
		unsafe
		{
			al_get_timer_speed(cast::transmute(self.allegro_timer)) as f64
		}
	}

	fn set_speed(&self, speed_secs: f64)
	{
		unsafe
		{
			al_set_timer_speed(self.allegro_timer, speed_secs as c_double);
		}
	}

	fn get_count(&self) -> i64
	{
		unsafe
		{
			al_get_timer_count(cast::transmute(self.allegro_timer)) as i64
		}
	}

	fn set_count(&self, count: i64)
	{
		unsafe
		{
			al_set_timer_count(self.allegro_timer, count as int64_t);
		}
	}

	fn add_count(&self, diff: i64)
	{
		unsafe
		{
			al_add_timer_count(self.allegro_timer, diff as int64_t);
		}
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
