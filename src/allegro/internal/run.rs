// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;
use std::option::Some;
use std::cast;

use ffi::*;

static mut global_main_func: Option<extern "Rust" fn()> = None;

pub fn run(argc: int, argv: **u8, main_func: extern "Rust" fn()) -> int
{
	unsafe
	{
		global_main_func = Some(main_func);
		al_run_main(argc as c_int, cast::transmute(argv), cast::transmute(allegro_main)) as int
	}
}

extern "C"
fn allegro_main(argc: int, argv: **u8) -> c_int
{
	use native;
	native::start(argc, argv, rust_main) as c_int
}

fn rust_main()
{
	unsafe
	{
		(global_main_func.unwrap())();
		al_uninstall_system();
	}
}
