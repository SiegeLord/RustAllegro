// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name="allegro_dialog"]
#![crate_type = "lib"]

#![allow(non_upper_case_globals)]

extern crate allegro_dialog_sys;
extern crate allegro;
#[macro_use]
extern crate allegro_util;
extern crate libc;

use allegro::{Core, Flag, Display};
use allegro_dialog_sys::*;

use std::cell::RefCell;
use std::marker::PhantomData;
use std::ffi::CString;

flag_type!
{
	MessageBoxFlags
	{
		MESSAGEBOX_WARN =      ALLEGRO_MESSAGEBOX_WARN,
		MESSAGEBOX_ERROR =     ALLEGRO_MESSAGEBOX_ERROR,
		MESSAGEBOX_OK_CANCEL = ALLEGRO_MESSAGEBOX_OK_CANCEL,
		MESSAGEBOX_YES_NO =    ALLEGRO_MESSAGEBOX_YES_NO,
		MESSAGEBOX_QUESTION =  ALLEGRO_MESSAGEBOX_QUESTION
	}
}

#[derive(Copy, Clone)]
pub enum MessageBoxResult
{
	NoButton,
	Affirmative,
	Negatory
}

static mut initialized: bool = false;
thread_local!(static spawned_on_this_thread: RefCell<bool> = RefCell::new(false));

pub struct DialogAddon
{
	no_send_marker: PhantomData<*mut u8>
}

impl DialogAddon
{
	pub fn init(core: &Core) -> Result<DialogAddon, String>
	{
		let mutex = core.get_core_mutex();
		let _guard = mutex.lock();
		unsafe
		{
			if initialized
			{
				if spawned_on_this_thread.with(|x| *x.borrow())
				{
					Err("The dialog addon has already been created in this task.".to_string())
				}
				else
				{
					spawned_on_this_thread.with(|x| *x.borrow_mut() = true);
					Ok(DialogAddon{ no_send_marker: PhantomData })
				}
			}
			else
			{
				if al_init_native_dialog_addon() != 0
				{
					initialized = true;
					spawned_on_this_thread.with(|x| *x.borrow_mut() = true);
					Ok(DialogAddon{ no_send_marker: PhantomData })
				}
				else
				{
					Err("Could not initialize the dialog addon.".to_string())
				}
			}
		}
	}

	pub fn get_version() -> i32
	{
		unsafe
		{
			al_get_allegro_native_dialog_version() as i32
		}
	}
}

pub fn show_native_message_box(display: Option<&Display>, title: &str, heading: &str, text: &str, buttons: Option<&str>, flags: MessageBoxFlags) -> MessageBoxResult
{
	use std::ptr;
	use libc::c_int;

	let d = display.map_or(ptr::null_mut(), |d| d.get_allegro_display());
	let title = CString::new(title.as_bytes()).unwrap();
	let heading = CString::new(heading.as_bytes()).unwrap();
	let text = CString::new(text.as_bytes()).unwrap();

	let ret = unsafe
	{
		match buttons
		{
			Some(buttons) =>
			{
				let buttons = CString::new(buttons.as_bytes()).unwrap();
				al_show_native_message_box(d, title.as_ptr(), heading.as_ptr(), text.as_ptr(), buttons.as_ptr(), flags.get() as c_int)
			},
			None =>
			{
				al_show_native_message_box(d, title.as_ptr(), heading.as_ptr(), text.as_ptr(), ptr::null(), flags.get() as c_int)
			}
		}
	};

	match ret
	{
		1 => MessageBoxResult::Affirmative,
		2 => MessageBoxResult::Negatory,
		_ => MessageBoxResult::NoButton,
	}
}
