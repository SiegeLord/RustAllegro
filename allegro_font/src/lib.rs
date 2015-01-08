// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name="allegro_font"]

#![crate_type = "lib"]
#![feature(thread_local)]
#![feature(unsafe_destructor)]

extern crate allegro;
extern crate "allegro-sys" as allegro_sys;
extern crate "allegro_font-sys" as allegro_font_sys;
extern crate libc;

pub use font::*;
pub use addon::*;

mod addon;
mod font;

#[macro_use]
mod macros;
