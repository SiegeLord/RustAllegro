// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use allegro::Core;
use allegro_font_sys::*;

pub struct FontAddon
{
	_dummy: (),
}

impl FontAddon
{
	pub fn init(_: &Core) -> Result<FontAddon, String>
	{
		use std::sync::{Once, ONCE_INIT};
		static mut RUN_ONCE: Once = ONCE_INIT;

		let mut res = Err("The font addon already initialized.".into());
		unsafe {
			RUN_ONCE.call_once(|| {
				al_init_font_addon();
				res = Ok(FontAddon { _dummy: () });
			})
		}
		res
	}

	pub fn get_version() -> i32
	{
		unsafe { al_get_allegro_font_version() as i32 }
	}
}
