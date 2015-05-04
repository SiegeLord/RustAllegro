// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name="allegro_acodec_sys"]
#![crate_type = "lib"]

extern crate libc;
#[macro_use]
extern crate allegro_util;

pub use self::allegro_acodec::*;

pub mod allegro_acodec
{
	use libc::*;
	use allegro_util::c_bool;

	extern "C"
	{
		pub fn al_init_acodec_addon() -> c_bool;
		pub fn al_get_allegro_acodec_version() -> uint32_t;
	}
}
