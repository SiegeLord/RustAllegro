// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name="allegro_image-sys"]

#![crate_type = "lib"]
#![allow(unstable)]

extern crate libc;

pub use allegro_image::*;

mod rust_util;

pub mod allegro_image
{
	use libc::*;
	use rust_util::c_bool;

	extern "C"
	{
		pub fn al_init_image_addon() -> c_bool;
		pub fn al_shutdown_image_addon();
		pub fn al_get_allegro_image_version() -> uint32_t;
	}
}
