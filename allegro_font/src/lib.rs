// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name="allegro_font"]

#![crate_type = "lib"]
#![feature(globs)]
#![feature(macro_rules)]
#![feature(thread_local)]
#![feature(unsafe_destructor)]

extern crate allegro;
extern crate libc;

pub use font::*;
pub use addon::*;

#[cfg(not(manual_link))]
mod link_name
{
	#[link(name = "allegro_font")]
	extern "C" {}
}

pub mod ffi;
pub mod addon;
pub mod font;

#[macro_escape]
pub mod macros;
