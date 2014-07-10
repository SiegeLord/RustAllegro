// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name="allegro_acodec"]

#![comment = "Allegro 5 Acodec addon Rust bindings"]
#![license = "zlib"]
#![crate_type = "lib"]
#![feature(globs)]
#![feature(macro_rules)]
#![feature(struct_variant)]
#![feature(thread_local)]

extern crate allegro;
extern crate allegro_audio;
extern crate libc;

use allegro_audio::AudioAddon;
use ffi::allegro_acodec::*;

use std::option::Some;
use std::kinds::marker::NoSend;

#[cfg(not(manual_link))]
mod link_name
{
	#[link(name = "allegro_acodec")]
	extern "C" {}
}

pub mod ffi
{
	pub use self::allegro_acodec::*;
	pub mod allegro_acodec
	{
		use libc::*;
		use allegro::c_bool;

		extern "C"
		{
			pub fn al_init_acodec_addon() -> c_bool;
			pub fn al_get_allegro_acodec_version() -> uint32_t;
		}
	}
}

#[macro_escape]
#[path = "../../src/common_macros.rs"]
pub mod macros;

static mut initialized: bool = false;
//#[thread_local]
static mut spawned_on_this_thread: bool = false;

pub struct AcodecAddon
{
	no_send_marker: NoSend
}

impl AcodecAddon
{
	pub fn init(audio_addon: &AudioAddon) -> Option<AcodecAddon>
	{
		let mutex = audio_addon.get_core_mutex();
		let _guard = mutex.lock();
		unsafe
		{
			if initialized
			{
				if spawned_on_this_thread
				{
					None
				}
				else
				{
					spawned_on_this_thread = true;
					Some(AcodecAddon{ no_send_marker: NoSend })
				}
			}
			else
			{
				if al_init_acodec_addon() != 0
				{
					initialized = true;
					spawned_on_this_thread = true;
					Some(AcodecAddon{ no_send_marker: NoSend })
				}
				else
				{
					None
				}
			}
		}
	}

	pub fn get_version() -> i32
	{
		unsafe
		{
			al_get_allegro_acodec_version() as i32
		}
	}
}
