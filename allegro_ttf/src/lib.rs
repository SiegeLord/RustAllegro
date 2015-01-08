// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name="allegro_ttf"]

#![crate_type = "lib"]
#![feature(thread_local)]

extern crate allegro;
extern crate allegro_font;
extern crate "allegro_ttf-sys" as allegro_ttf_sys;
extern crate libc;

use allegro::Flag;
use allegro_font::{FontAddon, Font};
use allegro_ttf_sys::*;
use libc::*;

use std::ffi::CString;
use std::kinds::marker::NoSend;

#[macro_use]
mod macros;

static mut initialized: bool = false;
#[thread_local]
static mut spawned_on_this_thread: bool = false;

flag_type!
{
	TtfFlags
	{
		TTF_NO_KERNING = ALLEGRO_TTF_NO_KERNING,
		TTF_MONOCHROME = ALLEGRO_TTF_MONOCHROME,
		TTF_NO_AUTOHINT = ALLEGRO_TTF_NO_AUTOHINT
	}
}

#[allow(missing_copy_implementations)]
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
		let filename = CString::from_slice(filename.as_bytes());
		unsafe
		{
			Font::wrap_allegro_font(al_load_ttf_font(filename.as_ptr(), size as c_int, flags.get() as c_int))
		}
	}

	pub fn load_ttf_font_stretch(&self, filename: &str, width: i32, height: i32, flags: TtfFlags) -> Result<Font, String>
	{
		if width < 0 && height >= 0 || width >= 0 && height < 0
		{
			Err("Invalid dimension combination.".to_string())
		}
		else
		{
			let filename = CString::from_slice(filename.as_bytes());
			unsafe
			{
				Font::wrap_allegro_font(al_load_ttf_font_stretch(filename.as_ptr(), width as c_int, height as c_int, flags.get() as c_int))
					.map_err(|_| "Failed to load the ttf font.".to_string())
			}
		}
	}
}
