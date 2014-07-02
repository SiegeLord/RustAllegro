// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;
use std::option::Some;
use std::mem;
use std::str;
use std::kinds::marker::NoSend;
use std::task::TaskBuilder;
use native::NativeTaskBuilder;

use sync::{Arc, Mutex};

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
	keyboard_event_source: Option<EventSource>,
	mouse_event_source: Option<EventSource>,
	joystick_event_source: Option<EventSource>,
	mutex: Arc<Mutex<()>>,
	no_send_marker: NoSend,
}

impl Core
{
	pub fn init() -> Result<Core, String>
	{
		use sync::one::{Once, ONCE_INIT};
		static mut run_once: Once = ONCE_INIT;

		let mut res = Err("Already initialized.".to_string());
		unsafe
		{
			run_once.doit(||
			{
				res = if al_install_system(ALLEGRO_VERSION_INT as c_int, None) != 0
				{
					al_set_new_bitmap_flags(ALLEGRO_MEMORY_BITMAP as i32);
					dummy_target = al_create_bitmap(1, 1);
					al_set_new_bitmap_flags(0);
					if dummy_target.is_null()
					{
						Err("Failed to create the dummy target... something is very wrong!".to_string())
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
								mutex: Arc::new(Mutex::new(())),
								no_send_marker: NoSend,
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
				};
			});
		}
		res
	}

	pub fn spawn(&self, thread_proc: proc(Core):Send)
	{
		let mutex = self.get_core_mutex();
		TaskBuilder::new().native().spawn(proc()
		{
			thread_proc(Core
			{
				keyboard_event_source: None,
				mouse_event_source: None,
				joystick_event_source: None,
				mutex: mutex,
				no_send_marker: NoSend,
			});
		});
	}

	pub fn get_core_mutex(&self) -> Arc<Mutex<()>>
	{
		self.mutex.clone()
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
			let ret = al_get_monitor_info(adapter as c_int, mem::transmute(&mut c_info)) != 0;
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

	pub fn get_time(&self) -> f64
	{
		unsafe
		{
			al_get_time() as f64
		}
	}

	pub fn install_keyboard(&self) -> bool
	{
		unsafe
		{
			al_install_keyboard() != 0
		}
	}

	pub fn is_keyboard_installed(&self) -> bool
	{
		unsafe
		{
			al_is_keyboard_installed() != 0
		}
	}

	pub fn get_keyboard_event_source<'l>(&'l mut self) -> Option<&'l EventSource>
	{
		if self.keyboard_event_source.is_none() && self.is_keyboard_installed()
		{
			unsafe
			{
				self.keyboard_event_source = Some(new_event_source_ref(al_get_keyboard_event_source()));
			}
		}

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

	pub fn keycode_to_name(&self, k: key::KeyCode) -> Option<String>
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

	pub fn install_mouse(&self) -> bool
	{
		unsafe
		{
			al_install_mouse() != 0
		}
	}

	pub fn is_mouse_installed(&self) -> bool
	{
		unsafe
		{
			al_is_mouse_installed() != 0
		}
	}

	pub fn get_mouse_event_source<'l>(&'l mut self) -> Option<&'l EventSource>
	{
		if self.mouse_event_source.is_none() && self.is_mouse_installed()
		{
			unsafe
			{
				self.mouse_event_source = Some(new_event_source_ref(al_get_mouse_event_source()));
			}
		}

		match self.mouse_event_source
		{
			Some(ref s) => Some(s),
			None => None
		}
	}

	pub fn install_joystick(&self) -> bool
	{
		unsafe
		{
			al_install_joystick() != 0
		}
	}

	pub fn is_joystick_installed(&self) -> bool
	{
		unsafe
		{
			al_is_joystick_installed() != 0
		}
	}

	pub fn get_joystick_event_source<'l>(&'l mut self) -> Option<&'l EventSource>
	{
		if self.joystick_event_source.is_none() && self.is_joystick_installed()
		{
			unsafe
			{
				self.joystick_event_source = Some(new_event_source_ref(al_get_joystick_event_source()));
			}
		}

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
