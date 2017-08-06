// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.


use ffi::*;
use libc::*;
use std::process;
use std::ptr;
use std::thread::spawn;

static mut GLOBAL_MAIN_FUNC: Option<fn()> = None;

pub fn run(main_func: fn()) -> !
{
	unsafe {
		GLOBAL_MAIN_FUNC = Some(main_func);
		let ret = al_run_main(0, ptr::null(), allegro_main);
		process::exit(ret)
	}
}

extern "C" fn allegro_main(_: i32, _: *const *const i8) -> c_int
{
	unsafe {
		let ok = spawn(move || { (GLOBAL_MAIN_FUNC.unwrap())(); })
			.join()
			.is_ok();
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
