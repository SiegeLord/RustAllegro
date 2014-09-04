// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;
use std::option::Some as RealSome;
use std::mem;
use std::string;
use std::kinds::marker::NoSend;
use std::task::TaskBuilder;
use native::NativeTaskBuilder;

use sync::{Arc, Mutex};

use ffi::*;

use internal::events::{EventSource, new_event_source_ref};
use internal::keycodes::*;
use internal::display::Display;

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

	pub fn get_monitor_info(&self, adapter: i32) -> Result<(i32, i32, i32, i32), ()>
	{
		unsafe
		{
			let mut c_info = ALLEGRO_MONITOR_INFO{ x1: 0, y1: 0, x2: 0, y2: 0 };
			if al_get_monitor_info(adapter as c_int, mem::transmute(&mut c_info)) != 0
			{
				Ok((c_info.x1 as i32, c_info.y1 as i32, c_info.x2 as i32, c_info.y2 as i32))
			}
			else
			{
				Err(())
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

	pub fn install_keyboard(&self) -> Result<(), ()>
	{
		unsafe
		{
			if al_install_keyboard() != 0
			{
				Ok(())
			}
			else
			{
				Err(())
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

	pub fn get_keyboard_event_source(&mut self) -> &EventSource
	{
		if self.keyboard_event_source.is_none() && self.is_keyboard_installed()
		{
			unsafe
			{
				self.keyboard_event_source = RealSome(new_event_source_ref(al_get_keyboard_event_source()));
			}
		}

		self.keyboard_event_source.as_ref().expect("Keyboard not installed")
	}

	pub fn set_keyboard_leds(&self, leds: KeyModifier) -> Result<(), ()>
	{
		assert!(self.is_keyboard_installed());
		unsafe
		{
			if al_set_keyboard_leds(leds.get() as c_int) != 0
			{
				Ok(())
			}
			else
			{
				Err(())
			}
		}
	}

	pub fn keycode_to_name(&self, k: key::KeyCode) -> String
	{
		assert!(self.is_keyboard_installed());
		unsafe
		{
			string::raw::from_buf(al_keycode_to_name(k as c_int) as *const _)
		}
	}

	pub fn install_mouse(&self) -> Result<(), ()>
	{
		unsafe
		{
			if al_install_mouse() != 0
			{
				Ok(())
			}
			else
			{
				Err(())
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

	pub fn get_mouse_event_source(&mut self) -> &EventSource
	{
		if self.mouse_event_source.is_none() && self.is_mouse_installed()
		{
			unsafe
			{
				self.mouse_event_source = RealSome(new_event_source_ref(al_get_mouse_event_source()));
			}
		}

		self.mouse_event_source.as_ref().expect("Mouse not installed")
	}

	pub fn install_joystick(&self) -> Result<(), ()>
	{
		unsafe
		{
			if al_install_joystick() != 0
			{
				Ok(())
			}
			else
			{
				Err(())
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

	pub fn get_joystick_event_source(&mut self) -> &EventSource
	{
		if self.joystick_event_source.is_none() && self.is_joystick_installed()
		{
			unsafe
			{
				self.joystick_event_source = RealSome(new_event_source_ref(al_get_joystick_event_source()));
			}
		}

		self.joystick_event_source.as_ref().expect("Joystick not installed")
	}

	pub fn reconfigure_joysticks(&self) -> Result<(), ()>
	{
		assert!(self.is_joystick_installed());
		unsafe
		{
			if al_reconfigure_joysticks() != 0
			{
				Ok(())
			}
			else
			{
				Err(())
			}
		}
	}

	pub fn get_num_joysticks(&self) -> i32
	{
		assert!(self.is_joystick_installed());
		unsafe
		{
			al_get_num_joysticks() as i32
		}
	}

	pub fn get_mouse_num_buttons(&self) -> u32
	{
		assert!(self.is_mouse_installed())
		unsafe
		{
			al_get_mouse_num_buttons() as u32
		}
	}

	pub fn get_mouse_num_axes(&self) -> u32
	{
		assert!(self.is_mouse_installed())
		unsafe
		{
			al_get_mouse_num_axes() as u32
		}
	}

	pub fn set_mouse_xy(&self, display: &Display, x: i32, y: i32) -> Result<(), ()>
	{
		assert!(self.is_mouse_installed())
		unsafe
		{
			if al_set_mouse_xy(display.get_allegro_display(), x as c_int, y as c_int) != 0
			{
				Ok(())
			}
			else
			{
				Err(())
			}
		}
	}

	pub fn set_mouse_z(&self, z: i32) -> Result<(), ()>
	{
		assert!(self.is_mouse_installed())
		unsafe
		{
			if al_set_mouse_z(z as c_int) != 0
			{
				Ok(())
			}
			else
			{
				Err(())
			}
		}
	}

	pub fn set_mouse_w(&self, w: i32) -> Result<(), ()>
	{
		assert!(self.is_mouse_installed())
		unsafe
		{
			if al_set_mouse_w(w as c_int) != 0
			{
				Ok(())
			}
			else
			{
				Err(())
			}
		}
	}

	pub fn set_mouse_axis(&self, axis: i32, value: i32) -> Result<(), ()>
	{
		assert!(self.is_mouse_installed())
		unsafe
		{
			if al_set_mouse_axis(axis as c_int, value as c_int) != 0
			{
				Ok(())
			}
			else
			{
				Err(())
			}
		}
	}

	pub fn grab_mouse(&self, display: &Display) -> Result<(), ()>
	{
		assert!(self.is_mouse_installed())
		unsafe
		{
			if al_grab_mouse(display.get_allegro_display()) != 0
			{
				Ok(())
			}
			else
			{
				Err(())
			}
		}
	}

	pub fn ungrab_mouse(&self) -> Result<(), ()>
	{
		assert!(self.is_mouse_installed())
		unsafe
		{
			if al_ungrab_mouse() != 0
			{
				Ok(())
			}
			else
			{
				Err(())
			}
		}
	}

}
