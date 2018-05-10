// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name = "allegro_font"]
#![crate_type = "lib"]

extern crate allegro;
extern crate allegro_font_sys;
extern crate allegro_sys;
extern crate allegro_util;
extern crate libc;

pub use addon::*;
pub use font::*;

mod addon;
mod font;
