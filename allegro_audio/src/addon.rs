// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use allegro::Core;
use allegro_audio_sys::*;

pub struct AudioAddon
{
	_dummy: (),
}

impl AudioAddon
{
	pub fn init(_: &Core) -> Result<AudioAddon, String>
	{
		use std::sync::{Once, ONCE_INIT};
		static mut RUN_ONCE: Once = ONCE_INIT;

		let mut res = Err("The audio addon already initialized.".into());
		unsafe {
			RUN_ONCE.call_once(|| {
				res = if al_install_audio() != 0
				{
					Ok(AudioAddon { _dummy: () })
				}
				else
				{
					Err("Could not initialize the audio addon.".into())
				}
			})
		}
		res
	}

	pub fn get_version() -> i32
	{
		unsafe { al_get_allegro_audio_version() as i32 }
	}
}
