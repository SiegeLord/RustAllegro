// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_id="allegro_audio#5.0.10.1"]

#![comment = "Allegro 5 audio addon Rust bindings"]
#![license = "zlib"]
#![crate_type = "lib"]
#![feature(globs)]
#![feature(macro_rules)]
#![feature(struct_variant)]
#![feature(thread_local)]

extern crate allegro = "allegro5#5.0.10.1";
extern crate libc;
extern crate sync;

pub use addon::*;
pub use properties::*;
pub use sink::*;

#[link(name = "allegro_audio")]
extern "C" {}

#[macro_escape]
#[path = "../macros.rs"]
pub mod macros;

pub mod ffi;
pub mod addon;
pub mod properties;
pub mod sink;
