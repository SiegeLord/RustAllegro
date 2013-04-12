use core::libc::*;
use allegro5::base::*;

mod c
{
	use core::libc::*;

	pub extern "C"
	{
		fn atexit(cb : extern "C" fn()) -> c_int;

		fn al_install_system(version : c_int, atexit_ptr : extern "C" fn(cb : extern "C" fn()) -> c_int) -> bool;
	}
}

pub fn al_init() -> bool
{
	extern "C"
	fn atexit_adaptor(cb : extern "C" fn()) -> c_int
	{
		unsafe
		{
			return c::atexit(cb);
		}
	}

	unsafe
	{
		return al_install_system(ALLEGRO_VERSION_INT, cast::transmute(atexit_adaptor));
	}
}

pub fn al_install_system(version : u32, atexit_ptr : extern "C" fn(cb : extern "C" fn()) -> c_int ) -> bool
{
	unsafe
	{
		return c::al_install_system(version as i32, atexit_ptr);
	}
}
