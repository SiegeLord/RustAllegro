// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name = "allegro_acodec"]
#![crate_type = "lib"]

extern crate allegro;
extern crate allegro_acodec_sys;
extern crate allegro_audio;

use allegro_acodec_sys::*;
use allegro_audio::AudioAddon;

pub struct AcodecAddon
{
	_dummy: (),
}

impl AcodecAddon
{
	pub fn init(_: &AudioAddon) -> Result<AcodecAddon, String>
	{
		use std::sync::{Once, ONCE_INIT};
		static mut RUN_ONCE: Once = ONCE_INIT;

		let mut res = Err("The acodec addon already initialized.".into());
		unsafe {
			RUN_ONCE.call_once(|| {
				res = if al_init_acodec_addon() != 0
				{
					Ok(AcodecAddon { _dummy: () })
				}
				else
				{
					Err("Could not initialize the acodec addon.".into())
				}
			})
		}
		res
	}

	pub fn get_version() -> i32
	{
		unsafe { al_get_allegro_acodec_version() as i32 }
	}
}
