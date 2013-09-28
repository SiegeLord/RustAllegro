use std::libc::*;
use rt = std::rt;
use cast = std::cast;

use ffi::*;

extern "C"
fn allegro_main(argc: int, argv: **u8) -> c_int
{
	rt::start(argc, argv, rust_main) as c_int
}

fn rust_main()
{
	unsafe
	{
		if al_install_system(ALLEGRO_VERSION_INT as c_int, None) == 0
		{
			fail!("Could not initialize Allegro. Your C library version probably doesn't match the version of the Rust binding.");
		}
		private::global_data.installed = true;
		(private::global_data.main_func.unwrap())();
		private::global_data.installed = false;
		al_uninstall_system();
	}
}

pub fn run(argc: int, argv: **u8, main_func: extern fn()) -> int
{
	unsafe
	{
		private::global_data.main_func = Some(main_func);
		al_run_main(argc as c_int, cast::transmute(argv), cast::transmute(allegro_main)) as int
	}
}

mod private
{
	struct GlobalData
	{
		main_func: Option<extern fn()>,
		installed: bool
	}

	pub static mut global_data: GlobalData = GlobalData{main_func: None, installed: false};
}
