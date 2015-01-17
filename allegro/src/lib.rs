// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name="allegro"]

#![crate_type = "lib"]
#![allow(unstable)]
#![feature(unsafe_destructor)]
#![feature(optin_builtin_traits)]

extern crate libc;
extern crate "allegro-sys" as ffi;

pub use internal::bitmap::external::*;
pub use internal::bitmap_like::*;
pub use internal::color::*;
pub use internal::core::external::*;
pub use internal::display::*;
pub use internal::events::external::*;
pub use internal::joystick::*;
pub use internal::keycodes::*;
pub use internal::run::*;
pub use internal::timer::*;
pub use internal::transformations::external::*;
pub use rust_util::*;

#[macro_use]
mod macros;
mod rust_util;

mod internal
{
	pub mod bitmap;
	pub mod bitmap_like;
	pub mod color;
	pub mod core;
	pub mod display;
	pub mod events;
	pub mod joystick;
	pub mod keycodes;
	pub mod run;
	pub mod timer;
	pub mod transformations;
}
pub mod allegro_main;
