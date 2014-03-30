use std::libc::*;
use std::cast;
use std::str;

use ffi::*;

use internal::events::*;
use internal::keycodes::*;

pub mod external
{
	pub use super::Core;
}

pub static mut dummy_target: *mut ALLEGRO_BITMAP = 0 as *mut ALLEGRO_BITMAP;

pub struct Core
{
	priv keyboard_event_source: Option<EventSource>,
	priv mouse_event_source: Option<EventSource>,
	priv joystick_event_source: Option<EventSource>,
}

impl Core
{
	pub fn init() -> Result<Core, ~str>
	{
		unsafe
		{
			/* FIXME: Make this thread-safe */
			if al_install_system(ALLEGRO_VERSION_INT as c_int, None) != 0
			{
				al_set_new_bitmap_flags(ALLEGRO_MEMORY_BITMAP as i32);
				dummy_target = al_create_bitmap(1, 1);
				al_set_new_bitmap_flags(0);
				if dummy_target.is_null()
				{
					Err(~"Failed to create the dummy target... something is very wrong!")
				}
				else
				{
					al_set_target_bitmap(dummy_target);
					Ok
					(
						Core
						{
							keyboard_event_source: None,
							mouse_event_source: None,
							joystick_event_source: None,
						}
					)
				}
			}
			else
			{
				let version = al_get_allegro_version();
				let major = version >> 24;
				let minor = (version >> 16) & 255;
				let revision = (version >> 8) & 255;
				let release = version & 255;

				Err(format!("The system Allegro version ({}.{}.{}.{}) does not match the version of this binding ({}.{}.{}.{})",
				    major, minor, revision, release,
				    ALLEGRO_VERSION, ALLEGRO_SUB_VERSION, ALLEGRO_WIP_VERSION, ALLEGRO_RELEASE_NUMBER))
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

	pub fn install_joystick(&mut self) -> bool
	{
		unsafe
		{
			if al_install_joystick() != 0
			{
				self.joystick_event_source = Some(new_event_source_ref(al_get_joystick_event_source()));
				true
			}
			else
			{
				false
			}
		}
	}

	pub fn is_joystick_installed(&self) -> bool
	{
		unsafe
		{
			al_is_joystick_installed() != 0
		}
	}

	pub fn get_joystick_event_source<'l>(&'l self) -> Option<&'l EventSource>
	{
		match self.joystick_event_source
		{
			Some(ref s) => Some(s),
			None => None
		}
	}

	pub fn reconfigure_joysticks(&self) -> bool
	{
		if self.is_joystick_installed()
		{
			unsafe
			{
				al_reconfigure_joysticks() != 0
			}
		}
		else
		{
			false
		}
	}

	pub fn get_num_joysticks(&self) -> i32
	{
		if self.is_joystick_installed()
		{
			unsafe
			{
				al_get_num_joysticks() as i32
			}
		}
		else
		{
			0
		}
	}
}
