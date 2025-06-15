// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

pub use self::allegro_acodec::*;

pub mod allegro_acodec
{
	use allegro_util::c_bool;

	unsafe extern "C" {
		pub fn al_init_acodec_addon() -> c_bool;
		pub fn al_get_allegro_acodec_version() -> u32;
	}
}
