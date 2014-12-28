// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;
use std::mem;
use std::rt::args;
use std::rt::unwind::try;

use ffi::*;

static mut global_main_func: Option<extern "Rust" fn()> = None;

pub fn run(argc: int, argv: *const *const u8, main_func: extern "Rust" fn()) -> int
{
	unsafe
	{
		global_main_func = Some(main_func);
		al_run_main(argc as c_int, mem::transmute(argv), mem::transmute(allegro_main)) as int
	}
}

extern "C"
fn allegro_main(argc: int, argv: *const *const u8) -> c_int
{
	unsafe
	{
		let ok = try(move ||
		{
			args::init(argc, argv);
			(global_main_func.unwrap())();
		}).is_ok();
		al_uninstall_system();
		if ok
		{
			0
		}
		else
		{
			-1
		}
	}
}
