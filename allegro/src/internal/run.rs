// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;
use std::mem;
use std::ptr;
use std::rt::unwind::try;

use ffi::*;

static mut global_main_func: Option<extern "Rust" fn()> = None;

pub fn run(main_func: extern "Rust" fn())
{
	unsafe
	{
		global_main_func = Some(main_func);
		al_run_main(0, ptr::null(), mem::transmute(allegro_main));
	}
}

extern "C"
fn allegro_main(_: isize, _: *const *const u8) -> c_int
{
	unsafe
	{
		let _ = try(move ||
		{
			(global_main_func.unwrap())();
		});
		al_uninstall_system();
	}
	0
}
