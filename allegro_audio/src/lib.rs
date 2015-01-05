// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name="allegro_audio"]

#![crate_type = "lib"]
#![feature(globs)]
#![feature(macro_rules)]
#![feature(default_type_params)]
#![feature(associated_types)]
#![feature(thread_local)]

extern crate allegro;
extern crate "allegro_audio-sys" as allegro_audio_sys;
extern crate libc;

pub use addon::*;
pub use stream::*;
pub use properties::*;
pub use sink::*;
pub use mixer::*;
pub use sample::*;

#[macro_escape]
mod macros;

mod addon;
mod stream;
mod properties;
mod sink;
mod mixer;
mod sample;

mod internal;
