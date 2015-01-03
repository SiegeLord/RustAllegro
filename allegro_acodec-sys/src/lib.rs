// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name="allegro_acodec-sys"]

#![crate_type = "lib"]
#![feature(globs)]

extern crate libc;

pub use self::allegro_acodec::*;

mod rust_util;

pub mod allegro_acodec
{
	use libc::*;
	use rust_util::c_bool;

	extern "C"
	{
		pub fn al_init_acodec_addon() -> c_bool;
		pub fn al_get_allegro_acodec_version() -> uint32_t;
	}
}
