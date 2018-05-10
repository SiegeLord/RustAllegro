// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name = "allegro_ttf"]
#![crate_type = "lib"]

extern crate allegro;
extern crate allegro_font;
extern crate allegro_ttf_sys;
#[macro_use]
extern crate allegro_util;
extern crate libc;

use allegro::Flag;
use allegro_font::{Font, FontAddon};
use allegro_ttf_sys::*;
use libc::*;

use std::ffi::CString;

flag_type!
{
	TtfFlags
	{
		TTF_NO_KERNING = ALLEGRO_TTF_NO_KERNING,
		TTF_MONOCHROME = ALLEGRO_TTF_MONOCHROME,
		TTF_NO_AUTOHINT = ALLEGRO_TTF_NO_AUTOHINT
	}
}

pub struct TtfAddon
{
	_dummy: (),
}

impl TtfAddon
{
	pub fn init(_: &FontAddon) -> Result<TtfAddon, String>
	{
		use std::sync::{Once, ONCE_INIT};
		static mut RUN_ONCE: Once = ONCE_INIT;

		let mut res = Err("The TTF addon already initialized.".into());
		unsafe {
			RUN_ONCE.call_once(|| {
				res = if al_init_ttf_addon() != 0
				{
					Ok(TtfAddon { _dummy: () })
				}
				else
				{
					Err("Could not initialize the TTF addon.".into())
				}
			})
		}
		res
	}

	pub fn get_version() -> i32
	{
		unsafe { al_get_allegro_ttf_version() as i32 }
	}

	pub fn load_ttf_font(&self, filename: &str, size: i32, flags: TtfFlags) -> Result<Font, ()>
	{
		let filename = CString::new(filename.as_bytes()).unwrap();
		unsafe { Font::wrap_allegro_font(al_load_ttf_font(filename.as_ptr(), size as c_int, flags.get() as c_int)) }
	}

	pub fn load_ttf_font_stretch(&self, filename: &str, width: i32, height: i32, flags: TtfFlags) -> Result<Font, String>
	{
		if width < 0 && height >= 0 || width >= 0 && height < 0
		{
			Err("Invalid dimension combination.".to_string())
		}
		else
		{
			let filename = CString::new(filename.as_bytes()).unwrap();
			unsafe {
				Font::wrap_allegro_font(al_load_ttf_font_stretch(
					filename.as_ptr(),
					width as c_int,
					height as c_int,
					flags.get() as c_int,
				)).map_err(|_| "Failed to load the ttf font.".to_string())
			}
		}
	}
}
