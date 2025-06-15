// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

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
		use std::sync::Once;
		static RUN_ONCE: Once = Once::new();

		let mut res = Err("The image addon already initialized.".into());
		RUN_ONCE.call_once(|| unsafe {
			res = if al_init_image_addon() != 0
			{
				Ok(ImageAddon { _dummy: () })
			}
			else
			{
				Err("Could not initialize the image addon.".into())
			}
		});
		res
	}

	pub fn get_version() -> i32
	{
		unsafe { al_get_allegro_image_version() as i32 }
	}
}
