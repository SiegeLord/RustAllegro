use std::libc::*;

use ffi::*;

pub fn rest(seconds: f64)
{
	unsafe
	{
		al_rest(seconds as c_double);
	}
}
