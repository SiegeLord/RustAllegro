// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name = "allegro_image"]
#![crate_type = "lib"]

extern crate allegro;
extern crate allegro_util;
extern crate libc;

use allegro::Core;
use ffi::allegro_image::*;

pub mod ffi
{
	pub use self::allegro_image::*;
	pub mod allegro_image
	{
		use allegro::c_bool;
		use libc::*;

		extern "C" {
			pub fn al_init_image_addon() -> c_bool;
			pub fn al_shutdown_image_addon();
			pub fn al_get_allegro_image_version() -> uint32_t;
		}
	}
}

pub struct ImageAddon
{
	_dummy: (),
}

impl ImageAddon
{
	pub fn init(_: &Core) -> Result<ImageAddon, String>
	{
		use std::sync::{Once, ONCE_INIT};
		static mut RUN_ONCE: Once = ONCE_INIT;

		let mut res = Err("The image addon already initialized.".into());
		unsafe {
			RUN_ONCE.call_once(|| {
				res = if al_init_image_addon() != 0
				{
					Ok(ImageAddon { _dummy: () })
				}
				else
				{
					Err("Could not initialize the image addon.".into())
				}
			})
		}
		res
	}

	pub fn get_version() -> i32
	{
		unsafe { al_get_allegro_image_version() as i32 }
	}
}
