// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use allegro::Core;
use allegro_audio_sys::*;

use std::sync::{Arc, Mutex};

static mut initialized: bool = false;
#[thread_local]
static mut spawned_on_this_thread: bool = false;

pub struct AudioAddon
{
	core_mutex: Arc<Mutex<()>>,
}

impl !Send for AudioAddon {}

impl AudioAddon
{
	pub fn init(core: &Core) -> Result<AudioAddon, String>
	{
		let mutex = core.get_core_mutex();
		let _guard = mutex.lock();
		unsafe
		{
			if initialized
			{
				if spawned_on_this_thread
				{
					Err("The audio addon has already been created in this task.".to_string())
				}
				else
				{
					// TODO: re-enable when this works on windows
					// spawned_on_this_thread = true;
					Ok(AudioAddon{ core_mutex: core.get_core_mutex() })
				}
			}
			else
			{
				if al_install_audio() != 0
				{
					initialized = true;
					// TODO: re-enable when this works on windows
					// spawned_on_this_thread = true;
					Ok(AudioAddon{ core_mutex: core.get_core_mutex() })
				}
				else
				{
					Err("Could not initialize the audio addon.".to_string())
				}
			}
		}
	}

	pub fn get_version() -> i32
	{
		unsafe
		{
			al_get_allegro_audio_version() as i32
		}
	}

	pub fn get_core_mutex(&self) -> Arc<Mutex<()>>
	{
		self.core_mutex.clone()
	}
}
