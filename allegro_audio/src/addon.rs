// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![allow(non_upper_case_globals)]

use allegro::Core;
use allegro_audio_sys::*;

use std::cell::RefCell;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

static mut initialized: bool = false;
thread_local!(static spawned_on_this_thread: RefCell<bool> = RefCell::new(false));

pub struct AudioAddon
{
	core_mutex: Arc<Mutex<()>>,
	no_send_marker: PhantomData<*mut u8>,
}

impl AudioAddon
{
	pub fn init(core: &Core) -> Result<AudioAddon, String>
	{
		let mutex = core.get_core_mutex();
		let _guard = mutex.lock();
		unsafe {
			if initialized
			{
				if spawned_on_this_thread.with(|x| *x.borrow())
				{
					Err("The audio addon has already been created in this task.".to_string())
				}
				else
				{
					spawned_on_this_thread.with(|x| *x.borrow_mut() = true);
					Ok(AudioAddon {
						core_mutex: core.get_core_mutex(),
						no_send_marker: PhantomData,
					})
				}
			}
			else
			{
				if al_install_audio() != 0
				{
					initialized = true;
					spawned_on_this_thread.with(|x| *x.borrow_mut() = true);
					Ok(AudioAddon {
						core_mutex: core.get_core_mutex(),
						no_send_marker: PhantomData,
					})
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
		unsafe { al_get_allegro_audio_version() as i32 }
	}

	pub fn get_core_mutex(&self) -> Arc<Mutex<()>>
	{
		self.core_mutex.clone()
	}
}
