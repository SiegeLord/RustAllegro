// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ALLEGRO_TIMEOUT
{
	pub __pad1__: uint64_t,
	pub __pad2__: uint64_t,
}

extern "C"
{
	pub fn al_get_time() -> c_double;
	pub fn al_rest(seconds: c_double);
	pub fn al_init_timeout(timeout: *mut ALLEGRO_TIMEOUT, seconds: c_double);
}
