// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name="allegro_ttf"]

#![crate_type = "lib"]
#![feature(globs)]
#![feature(macro_rules)]
#![feature(thread_local)]

extern crate allegro;
extern crate allegro_font;
extern crate libc;

use allegro::Flag;
use allegro_font::{FontAddon, Font};
use ffi::allegro_ttf::*;
use libc::*;

use std::kinds::marker::NoSend;

#[cfg(not(manual_link))]
mod link_name
{
	#[link(name = "allegro_ttf")]
	extern "C" {}
}

pub mod ffi
{
	pub use self::allegro_ttf::*;
	pub mod allegro_ttf
	{
		use libc::*;
		use allegro::c_bool;
		use allegro_font::ffi::ALLEGRO_FONT;

		pub const ALLEGRO_TTF_NO_KERNING: u32  = 1;
		pub const ALLEGRO_TTF_MONOCHROME: u32  = 2;
		pub const ALLEGRO_TTF_NO_AUTOHINT: u32 = 4;

		extern "C"
		{
			pub fn al_load_ttf_font(filename: *const c_char, size: c_int, flags: c_int) -> *mut ALLEGRO_FONT;
			//~ pub fn al_load_ttf_font_f(file: *mut ALLEGRO_FILE, filename: *const c_char, size: c_int, flags: c_int) -> *mut ALLEGRO_FONT;
			pub fn al_load_ttf_font_stretch(filename: *const c_char, w: c_int, h: c_int, flags: c_int) -> *mut ALLEGRO_FONT;
			//~ pub fn al_load_ttf_font_stretch_f(file: *mut ALLEGRO_FILE, filename: *const c_char, w: c_int, h: c_int, flags: c_int) -> *mut ALLEGRO_FONT;
			pub fn al_init_ttf_addon() -> c_bool;
			pub fn al_shutdown_ttf_addon();
			pub fn al_get_allegro_ttf_version() -> uint32_t;
		}
	}
}

#[macro_escape]
#[path = "../../src/common_macros.rs"]
pub mod macros;

static mut initialized: bool = false;
//#[thread_local]
static mut spawned_on_this_thread: bool = false;

flag_type!(
	TtfFlags
	{
		TTF_NO_KERNING = ALLEGRO_TTF_NO_KERNING,
		TTF_MONOCHROME = ALLEGRO_TTF_MONOCHROME,
		TTF_NO_AUTOHINT = ALLEGRO_TTF_NO_AUTOHINT
	}
)

pub struct TtfAddon
{
	no_send_marker: NoSend
}

impl TtfAddon
{
	pub fn init(font_addon: &FontAddon) -> Result<TtfAddon, String>
	{
		let mutex = font_addon.get_core_mutex();
		let _guard = mutex.lock();
		unsafe
		{
			if initialized
			{
				if spawned_on_this_thread
				{
					Err("The ttf addon has already been created in this task.".to_string())
				}
				else
				{
					spawned_on_this_thread = true;
					Ok(TtfAddon{ no_send_marker: NoSend })
				}
			}
			else
			{
				if al_init_ttf_addon() != 0
				{
					initialized = true;
					spawned_on_this_thread = true;
					Ok(TtfAddon{ no_send_marker: NoSend })
				}
				else
				{
					Err("Could not initialize the ttf addon.".to_string())
				}
			}
		}
	}

	pub fn get_version() -> i32
	{
		unsafe
		{
			al_get_allegro_ttf_version() as i32
		}
	}

	pub fn load_ttf_font(&self, filename: &str, size: i32, flags: TtfFlags) -> Result<Font, ()>
	{
		filename.with_c_str(|s|
		{
			unsafe
			{
				Font::wrap_allegro_font(al_load_ttf_font(s, size as c_int, flags.get() as c_int))
			}
		})
	}

	pub fn load_ttf_font_stretch(&self, filename: &str, width: i32, height: i32, flags: TtfFlags) -> Result<Font, String>
	{
		if width < 0 && height >= 0 || width >= 0 && height < 0
		{
			Err("Invalid dimension combination.".to_string())
		}
		else
		{
			filename.with_c_str(|s|
			{
				unsafe
				{
					Font::wrap_allegro_font(al_load_ttf_font_stretch(s, width as c_int, height as c_int, flags.get() as c_int))
						.map_err(|_| "Failed to load the ttf font.".to_string())
				}
			})
		}
	}
}
