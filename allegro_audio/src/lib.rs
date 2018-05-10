// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name = "allegro_audio"]
#![crate_type = "lib"]

extern crate allegro;
extern crate allegro_audio_sys;
#[macro_use]
extern crate allegro_util;
extern crate libc;

pub use addon::*;
pub use mixer::*;
pub use properties::*;
pub use sample::*;
pub use sink::*;
pub use stream::*;

mod addon;
mod mixer;
mod properties;
mod sample;
mod sink;
mod stream;

mod internal;
