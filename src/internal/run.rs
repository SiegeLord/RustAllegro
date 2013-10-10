use std::libc::*;
use rt = std::rt;
use cast = std::cast;

use ffi::*;

static mut global_main_func: Option<extern fn()> = None;

pub fn run(argc: int, argv: **u8, main_func: extern fn()) -> int
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
	rt::start(argc, argv, rust_main) as c_int
}

fn rust_main()
{
	unsafe
	{
		(global_main_func.unwrap())();
		al_uninstall_system();
	}
}
