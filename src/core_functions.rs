use std::libc::*;
use std::cast;

use ffi::*;
use run::private::global_data;

pub struct MonitorInfo
{
	x1: int,
	y1: int,
	x2: int,
	y2: int,
}

impl MonitorInfo
{
	pub fn new() -> MonitorInfo
	{
		MonitorInfo{ x1: 0, y1: 0, x2: 0, y2: 0 }
	}
}

pub fn get_num_video_adapters() -> int
{
	unsafe
	{
		al_get_num_video_adapters() as int
	}
}

pub fn get_monitor_info(adapter: int, info: &mut MonitorInfo) -> bool
{
	unsafe
	{
		let mut c_info = ALLEGRO_MONITOR_INFO{ x1: 0, y1: 0, x2: 0, y2: 0 };
		let ret = al_get_monitor_info(adapter as c_int, cast::transmute(&mut c_info)) != 0;
		info.x1 = c_info.x1 as int;
		info.y1 = c_info.y1 as int;
		info.x2 = c_info.x2 as int;
		info.y2 = c_info.y2 as int;
		ret
	}
}

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
