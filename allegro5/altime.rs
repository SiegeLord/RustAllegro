mod c
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
		return c::al_get_time();
	}
}

pub fn al_rest(seconds : f64)
{
	unsafe
	{
		c::al_rest(seconds);
	}
}
