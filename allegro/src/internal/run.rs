// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;
use std::mem;
use std::ptr;
use std::rt::unwind::try;
use std::env::{get_exit_status, set_exit_status};

use ffi::*;

static mut global_main_func: Option<extern "Rust" fn()> = None;

pub fn run(main_func: extern "Rust" fn())
{
	unsafe
	{
		global_main_func = Some(main_func);
		let ret = al_run_main(0, ptr::null(), mem::transmute(allegro_main));
		if get_exit_status() == 0
		{
			set_exit_status(ret);
		}
	}
}

extern "C"
fn allegro_main(_: isize, _: *const *const u8) -> c_int
{
	unsafe
	{
		let ok = try(move ||
		{
			(global_main_func.unwrap())();
		}).is_ok();
		al_uninstall_system();
		if ok
		{
			0
		}
		else
		{
			1
		}
	}
}
