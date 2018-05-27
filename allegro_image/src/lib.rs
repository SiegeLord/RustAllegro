// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name = "allegro_image"]
#![crate_type = "lib"]

extern crate allegro;
extern crate allegro_image_sys;
extern crate allegro_util;
extern crate libc;

use allegro::Core;
use allegro_image_sys::*;

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
