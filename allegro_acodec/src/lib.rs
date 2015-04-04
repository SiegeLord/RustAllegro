// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name="allegro_acodec"]
#![crate_type = "lib"]

#![feature(optin_builtin_traits)]
#![allow(non_upper_case_globals)]

extern crate allegro;
extern crate allegro_audio;
extern crate allegro_acodec_sys;

use std::cell::RefCell;
use allegro_audio::AudioAddon;
use allegro_acodec_sys::*;

static mut initialized: bool = false;
thread_local!(static spawned_on_this_thread: RefCell<bool> = RefCell::new(false));

#[allow(missing_copy_implementations)]
pub struct AcodecAddon;

impl !Send for AcodecAddon {}

impl AcodecAddon
{
	pub fn init(audio_addon: &AudioAddon) -> Result<AcodecAddon, String>
	{
		let mutex = audio_addon.get_core_mutex();
		let _guard = mutex.lock();
		unsafe
		{
			if initialized
			{
				if spawned_on_this_thread.with(|x| *x.borrow())
				{
					Err("The acodec addon has already been created in this task.".to_string())
				}
				else
				{
					spawned_on_this_thread.with(|x| *x.borrow_mut() = true);
					Ok(AcodecAddon)
				}
			}
			else
			{
				if al_init_acodec_addon() != 0
				{
					initialized = true;
					spawned_on_this_thread.with(|x| *x.borrow_mut() = true);
					Ok(AcodecAddon)
				}
				else
				{
					Err("Could not initialize the acodec addon.".to_string())
				}
			}
		}
	}

	pub fn get_version() -> i32
	{
		unsafe
		{
			al_get_allegro_acodec_version() as i32
		}
	}
}
