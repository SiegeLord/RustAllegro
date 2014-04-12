// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![allow(non_camel_case_types)]

pub use ffi::altime::*;
pub use ffi::base::*;
pub use ffi::bitmap::*;
pub use ffi::bitmap_draw::*;
pub use ffi::bitmap_io::*;
pub use ffi::color::*;
pub use ffi::display::*;
pub use ffi::drawing::*;
pub use ffi::events::*;
pub use ffi::joystick::*;
pub use ffi::keyboard::*;
pub use ffi::keycodes::*;
pub use ffi::monitor::*;
pub use ffi::mouse::*;
pub use ffi::utf8::*;
pub use ffi::system::*;
pub use ffi::timer::*;
pub use ffi::transformations::*;

#[link(name = "allegro")]
extern "C" {}

pub mod altime;
pub mod base;
pub mod color;
pub mod bitmap;
pub mod bitmap_draw;
pub mod bitmap_io;
pub mod display;
pub mod drawing;
pub mod events;
pub mod joystick;
pub mod keyboard;
pub mod keycodes;
pub mod monitor;
pub mod mouse;
pub mod path;
pub mod utf8;
pub mod system;
pub mod timer;
pub mod transformations;
