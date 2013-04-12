mod C
{
	use core::libc::*;
	
	pub extern "C"
	{
		fn al_get_time() -> c_double;
		fn al_rest(seconds : c_double);
	}
}

pub fn al_get_time() -> f64
{
	unsafe
	{
		return C::al_get_time();
	}
}

pub fn al_rest(seconds : f64)
{
	unsafe
	{
		C::al_rest(seconds);
	}
}
