// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_id="allegro_image#5.0.10.1"]

#![comment = "Allegro 5 image addon Rust bindings"]
#![license = "zlib"]
#![crate_type = "lib"]
#![feature(globs)]
#![feature(macro_rules)]
#![feature(struct_variant)]
#![feature(thread_local)]

extern crate allegro = "allegro5#5.0.10.1";
extern crate libc;

use std::kinds::marker::NoSend;

use allegro::Core;
use ffi::allegro_image::*;

pub mod ffi
{
	pub use self::allegro_image::*;
	pub mod allegro_image
	{
		use libc::*;
		use allegro::c_bool;

		#[link(name = "allegro_image")]
		extern "C" {
			pub fn al_init_image_addon() -> c_bool;
			pub fn al_shutdown_image_addon();
			pub fn al_get_allegro_image_version() -> uint32_t;
		}
	}
}

#[macro_escape]
#[path = "../macros.rs"]
pub mod macros;

static mut initialized: bool = false;
//#[thread_local]
static mut spawned_on_this_thread: bool = false;

pub struct ImageAddon
{
	no_send_marker: NoSend
}

impl ImageAddon
{
	pub fn init(core: &Core) -> Option<ImageAddon>
	{
		let mutex = core.get_core_mutex();
		let _guard = mutex.lock();
		unsafe
		{
			if initialized
			{
				if spawned_on_this_thread
				{
					None
				}
				else
				{
					spawned_on_this_thread = true;
					Some(ImageAddon{ no_send_marker: NoSend })
				}
			}
			else
			{
				if al_init_image_addon() != 0
				{
					initialized = true;
					spawned_on_this_thread = true;
					Some(ImageAddon{ no_send_marker: NoSend })
				}
				else
				{
					None
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
