// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name="allegro_sys"]
#![crate_type = "lib"]

#![allow(non_camel_case_types)]

extern crate libc;
#[macro_use]
extern crate allegro_util;

pub use altime::*;
pub use base::*;
pub use bitmap::*;
pub use bitmap_draw::*;
pub use bitmap_io::*;
pub use blend::*;
pub use color::*;
pub use config::*;
pub use display::*;
pub use drawing::*;
pub use events::*;
pub use file::*;
pub use joystick::*;
pub use keyboard::*;
pub use keycodes::*;
pub use monitor::*;
pub use mouse::*;
pub use utf8::*;
pub use render_state::*;
pub use shader::*;
pub use system::*;
pub use timer::*;
pub use transformations::*;

pub mod altime;
pub mod base;
pub mod bitmap;
pub mod bitmap_draw;
pub mod bitmap_io;
pub mod blend;
pub mod color;
pub mod config;
pub mod display;
pub mod drawing;
pub mod events;
pub mod file;
pub mod joystick;
pub mod keyboard;
pub mod keycodes;
pub mod monitor;
pub mod mouse;
pub mod path;
pub mod utf8;
pub mod render_state;
pub mod shader;
pub mod system;
pub mod timer;
pub mod transformations;
