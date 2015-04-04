// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;
use allegro_util::c_bool;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ALLEGRO_MONITOR_INFO
{
	pub x1: c_int,
	pub y1: c_int,
	pub x2: c_int,
	pub y2: c_int,
}

pub const ALLEGRO_DEFAULT_DISPLAY_ADAPTER: i32 = -1;

extern "C"
{
	pub fn al_get_num_video_adapters() -> c_int;
	pub fn al_get_monitor_info(adapter: c_int, info: *const ALLEGRO_MONITOR_INFO) -> c_bool;
}
