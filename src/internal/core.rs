use std::libc::*;
use std::cast;
use std::str;

use ffi::*;

use internal::events::*;
use internal::keycodes::*;
use internal::timer::*;

pub struct Core
{
	priv keyboard_event_source: Option<EventSource>,
	priv mouse_event_source: Option<EventSource>
}

impl Core
{
	pub fn init() -> Option<Core>
	{
		unsafe
		{
			/* FIXME: Make this thread-safe */
			if al_install_system(ALLEGRO_VERSION_INT as c_int, None) != 0
			{
				Some(Core{ keyboard_event_source: None, mouse_event_source: None })
			}
			else
			{
				None
			}
		}
	}

	pub fn get_num_video_adapters(&self) -> i32
	{
		unsafe
		{
			al_get_num_video_adapters() as i32
		}
	}

	pub fn get_monitor_info(&self, adapter: i32) -> Option<(i32, i32, i32, i32)>
	{
		unsafe
		{
			let mut c_info = ALLEGRO_MONITOR_INFO{ x1: 0, y1: 0, x2: 0, y2: 0 };
			let ret = al_get_monitor_info(adapter as c_int, cast::transmute(&mut c_info)) != 0;
			if ret
			{
				Some((c_info.x1 as i32, c_info.y1 as i32, c_info.x2 as i32, c_info.y2 as i32))
			}
			else
			{
				None
			}
		}
	}

	pub fn rest(&self, seconds: f64)
	{
		unsafe
		{
			al_rest(seconds as c_double);
		}
	}

	pub fn create_timer(&self, speed_secs: f64) -> Option<Timer>
	{
		new_timer(speed_secs)
	}

	pub fn create_event_queue(&self) -> Option<EventQueue>
	{
		new_queue()
	}

	pub fn install_keyboard(&mut self) -> bool
	{
		unsafe
		{
			if al_install_keyboard() != 0
			{
				self.keyboard_event_source = Some(new_event_source_ref(al_get_keyboard_event_source()));
				true
			}
			else
			{
				false
			}
		}
	}

	pub fn is_keyboard_installed(&self) -> bool
	{
		unsafe
		{
			al_is_keyboard_installed() != 0
		}
	}

	pub fn get_keyboard_event_source<'l>(&'l self) -> Option<&'l EventSource>
	{
		match self.keyboard_event_source
		{
			Some(ref s) => Some(s),
			None => None
		}
	}

	pub fn set_keyboard_leds(&self, leds: KeyModifier) -> bool
	{
		if self.is_keyboard_installed()
		{
			unsafe
			{
				al_set_keyboard_leds(leds.get() as c_int) != 0
			}
		}
		else
		{
			false
		}
	}

	pub fn keycode_to_name(&self, k: key::KeyCode) -> Option<~str>
	{
		if self.is_keyboard_installed()
		{
			unsafe
			{
				Some(str::raw::from_c_str(al_keycode_to_name(k as c_int)))
			}
		}
		else
		{
			None
		}
	}

	pub fn install_mouse(&mut self) -> bool
	{
		unsafe
		{
			if al_install_mouse() != 0
			{
				self.mouse_event_source = Some(new_event_source_ref(al_get_mouse_event_source()));
				true
			}
			else
			{
				false
			}
		}
	}

	pub fn is_mouse_installed(&self) -> bool
	{
		unsafe
		{
			al_is_mouse_installed() != 0
		}
	}

	pub fn get_mouse_event_source<'l>(&'l self) -> Option<&'l EventSource>
	{
		match self.mouse_event_source
		{
			Some(ref s) => Some(s),
			None => None
		}
	}
}
