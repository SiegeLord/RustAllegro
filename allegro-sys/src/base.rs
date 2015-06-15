// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;

include!(concat!(env!("OUT_DIR"), "/versions.rs"));

pub const ALLEGRO_VERSION_INT: u32          = ((ALLEGRO_VERSION << 24) | (ALLEGRO_SUB_VERSION << 16) | (ALLEGRO_WIP_VERSION << 8) | ALLEGRO_RELEASE_NUMBER);

extern "C"
{
	pub fn al_get_allegro_version() -> uint32_t;
	pub fn al_run_main(argc: c_int, argv: *const *const c_char, user_main: extern "C" fn(argc: c_int, argv: *const *const c_char) -> c_int) -> c_int;
}

pub const ALLEGRO_PI: f64 = 3.14159265358979323846;

#[allow(non_snake_case)]
pub fn AL_ID(a: u8, b: u8, c: u8, d: u8) -> u32
{
	((a as u32)<<24) | ((b as u32)<<16) | ((c as u32)<<8) | (d as u32)
}
