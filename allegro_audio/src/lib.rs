// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name="allegro_audio"]

#![comment = "Allegro 5 audio addon Rust bindings"]
#![license = "zlib"]
#![crate_type = "lib"]
#![feature(globs)]
#![feature(macro_rules)]
#![feature(struct_variant)]
#![feature(thread_local)]

extern crate allegro;
extern crate libc;
extern crate sync;

pub use addon::*;
pub use stream::*;
pub use properties::*;
pub use sink::*;
pub use mixer::*;
pub use sample::*;

#[cfg(not(manual_link))]
mod link_name
{
	#[link(name = "allegro_audio")]
	extern "C" {}
}

#[macro_escape]
#[path = "../../src/common_macros.rs"]
pub mod macros;

pub mod ffi;
pub mod addon;
pub mod stream;
pub mod properties;
pub mod sink;
pub mod mixer;
pub mod sample;

mod internal;
