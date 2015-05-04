// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name="allegro_image_sys"]
#![crate_type = "lib"]

extern crate libc;
#[macro_use]
extern crate allegro_util;

pub use allegro_image::*;

pub mod allegro_image
{
	use libc::*;
	use allegro_util::c_bool;

	extern "C"
	{
		pub fn al_init_image_addon() -> c_bool;
		pub fn al_shutdown_image_addon();
		pub fn al_get_allegro_image_version() -> uint32_t;
	}
}
