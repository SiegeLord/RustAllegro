// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name="allegro_image"]

#![crate_type = "lib"]
#![feature(globs)]
#![feature(macro_rules)]
#![feature(thread_local)]

extern crate allegro;
extern crate libc;

use std::kinds::marker::NoSend;

use allegro::Core;
use ffi::allegro_image::*;

#[cfg(not(manual_link))]
mod link_name
{
	#[link(name = "allegro_image")]
	extern "C" {}
}

pub mod ffi
{
	pub use self::allegro_image::*;
	pub mod allegro_image
	{
		use libc::*;
		use allegro::c_bool;

		extern "C"
		{
			pub fn al_init_image_addon() -> c_bool;
			pub fn al_shutdown_image_addon();
			pub fn al_get_allegro_image_version() -> uint32_t;
		}
	}
}

#[macro_escape]
pub mod macros;

static mut initialized: bool = false;
//#[thread_local]
static mut spawned_on_this_thread: bool = false;

#[allow(missing_copy_implementations)]
pub struct ImageAddon
{
	no_send_marker: NoSend
}

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
				if spawned_on_this_thread
				{
					Err("The image addon has already been created in this task.".to_string())
				}
				else
				{
					spawned_on_this_thread = true;
					Ok(ImageAddon{ no_send_marker: NoSend })
				}
			}
			else
			{
				if al_init_image_addon() != 0
				{
					initialized = true;
					spawned_on_this_thread = true;
					Ok(ImageAddon{ no_send_marker: NoSend })
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
