// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name="allegro_font"]
#![crate_type = "lib"]

#![feature(optin_builtin_traits)]
#![feature(libc)]

extern crate allegro;
extern crate allegro_sys;
extern crate allegro_font_sys;
#[macro_use]
extern crate allegro_util;
extern crate libc;

pub use font::*;
pub use addon::*;

mod addon;
mod font;
