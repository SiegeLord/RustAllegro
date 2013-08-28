use std::libc::*;
use rt = std::rt;
use cast = std::cast;

use ffi::*;

extern "C"
fn allegro_main(argc: int, argv: **u8) -> c_int
{
	unsafe
	{
		rt::start(argc, argv, private::global_data.crate_map.unwrap(), rust_main) as c_int
	}
}

fn rust_main()
{
	unsafe
	{
		if al_install_system(ALLEGRO_VERSION_INT as c_int, None) == 0
		{
			fail!("Could not initialize Allegro. Your C library version probably doesn't match the version of the Rust binding.");
		}
		private::global_data.active = true;
		(private::global_data.main_func.unwrap())();
		private::global_data.active = false;
		al_uninstall_system();
	}
}

pub fn run(argc: int, argv: **u8, crate_map: *u8, main_func: extern fn()) -> int
{
	unsafe
	{
		private::global_data.crate_map = Some(crate_map);
		private::global_data.main_func = Some(main_func);
		al_run_main(argc as c_int, cast::transmute(argv), cast::transmute(allegro_main)) as int
	}
}

mod private
{
	struct GlobalData
	{
		crate_map: Option<*u8>,
		main_func: Option<extern fn()>,
		active: bool
	}

	pub static mut global_data: GlobalData = GlobalData{crate_map: None, main_func: None, active: false};
}
