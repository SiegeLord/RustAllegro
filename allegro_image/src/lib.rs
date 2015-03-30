// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name="allegro_image"]
#![crate_type = "lib"]
#![allow(non_upper_case_globals)]

#![feature(thread_local)]
#![feature(optin_builtin_traits)]
#![feature(libc)]

extern crate libc;
extern crate allegro;
extern crate "allegro_image-sys" as ffi;

use std::cell::RefCell;

use allegro::Core;
use ffi::allegro_image::*;

#[macro_use]
mod macros;

static mut initialized: bool = false;
thread_local!(static spawned_on_this_thread: RefCell<bool> = RefCell::new(false));

#[allow(missing_copy_implementations)]
pub struct ImageAddon;

impl !Send for ImageAddon {}

impl ImageAddon
{
	pub fn init(core: &Core) -> Result<ImageAddon, String>
	{
		let mutex = core.get_core_mutex();
		let _guard = mutex.lock();
		unsafe
		{
			if initialized
			{
				if spawned_on_this_thread.with(|x| *x.borrow())
				{
					Err("The image addon has already been created in this task.".to_string())
				}
				else
				{
                    spawned_on_this_thread.with(|x| *x.borrow_mut() = true);
					Ok(ImageAddon)
				}
			}
			else
			{
				if al_init_image_addon() != 0
				{
					initialized = true;
                    spawned_on_this_thread.with(|x| *x.borrow_mut() = true);
					Ok(ImageAddon)
				}
				else
				{
					Err("Could not initialize the image addon.".to_string())
				}
			}
		}
	}

	pub fn get_version() -> i32
	{
		unsafe
		{
			al_get_allegro_image_version() as i32
		}
	}
}
