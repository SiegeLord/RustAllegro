// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

pub use allegro_image::*;

pub mod allegro_image
{
	use allegro_util::c_bool;

	unsafe extern "C" {
		pub fn al_init_image_addon() -> c_bool;
		pub fn al_shutdown_image_addon();
		pub fn al_get_allegro_image_version() -> u32;
	}
}
