// Copyright (c) 2018 by RustAllegro contributors.
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name = "allegro_memfile_sys"]
#![crate_type = "lib"]

extern crate allegro_sys;
extern crate libc;

pub use self::allegro_memfile::*;

pub mod allegro_memfile
{
	use allegro_sys::*;
	use libc::*;

	extern "C" {
		pub fn al_open_memfile(mem: *mut c_void, size: i64, mode: *const c_char) -> *mut ALLEGRO_FILE;
		pub fn al_get_allegro_memfile_version() -> u32;
	}
}
