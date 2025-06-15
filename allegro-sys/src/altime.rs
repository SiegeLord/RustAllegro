// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ALLEGRO_TIMEOUT
{
	pub __pad1__: u64,
	pub __pad2__: u64,
}

unsafe extern "C" {
	pub fn al_get_time() -> c_double;
	pub fn al_rest(seconds: c_double);
	pub fn al_init_timeout(timeout: *mut ALLEGRO_TIMEOUT, seconds: c_double);
}
