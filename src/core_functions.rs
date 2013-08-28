use std::libc::*;

use ffi::*;
use run::private::global_data;

pub fn rest(seconds: f64)
{
	unsafe
	{
		al_rest(seconds as c_double);
	}
}

pub fn is_installed() -> bool
{
	unsafe
	{
		global_data.installed
	}
}
