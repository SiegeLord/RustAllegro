use std::libc::*;
use std::cast;

use ffi::*;

use bitmap::*;
use bitmap::private::*;
use color::*;
use display::*;
use display::private::*;
use events::*;
use events::private::*;

pub struct Core
{
	priv dummy: ()
}

pub struct MonitorInfo
{
	x1: i32,
	y1: i32,
	x2: i32,
	y2: i32,
}

impl MonitorInfo
{
	pub fn new() -> MonitorInfo
	{
		MonitorInfo{ x1: 0, y1: 0, x2: 0, y2: 0 }
	}
}

impl Core
{
	pub fn init() -> Option<Core>
	{
		unsafe
		{
			/* FIXME: Make this thread-safe */
			if al_install_system(ALLEGRO_VERSION_INT as c_int, None) != 0
			{
				Some(Core{ dummy: () })
			}
			else
			{
				None
			}
		}
	}

	pub fn get_num_video_adapters(&self) -> i32
	{
		unsafe
		{
			al_get_num_video_adapters() as i32
		}
	}
	
	pub fn get_monitor_info(&self, adapter: i32, info: &mut MonitorInfo) -> bool
	{
		unsafe
		{
			let mut c_info = ALLEGRO_MONITOR_INFO{ x1: 0, y1: 0, x2: 0, y2: 0 };
			let ret = al_get_monitor_info(adapter as c_int, cast::transmute(&mut c_info)) != 0;
			info.x1 = c_info.x1 as i32;
			info.y1 = c_info.y1 as i32;
			info.x2 = c_info.x2 as i32;
			info.y2 = c_info.y2 as i32;
			ret
		}
	}

	pub fn rest(&self, seconds: f64)
	{
		unsafe
		{
			al_rest(seconds as c_double);
		}
	}
	
	pub fn create_display(&self, w: i32, h: i32) -> Option<Display>
	{
		new_display(w, h)
	}
	
	pub fn create_display_with_options(&self, w: i32, h: i32, opt: &DisplayOptions) -> Option<Display>
	{
		new_display_with_options(w, h, opt)
	}

	pub fn create_bitmap(&self, w: i32, h: i32) -> Option<Bitmap>
	{
		new_bitmap(w, h)
	}

	pub fn create_bitmap_with_options(&self, w: i32, h: i32, opt: &BitmapOptions) -> Option<Bitmap>
	{
		new_bitmap_with_options(w, h, opt)
	}
	
	pub fn map_rgb(&self, r: u8, g: u8, b: u8) -> Color
	{
		unsafe
		{
			Color(al_map_rgb(r as c_uchar, g as c_uchar, b as c_uchar))
		}
	}

	pub fn map_rgba(&self, r: u8, g: u8, b: u8, a: u8) -> Color
	{
		unsafe
		{
			Color(al_map_rgba(r as c_uchar, g as c_uchar, b as c_uchar, a as c_uchar))
		}
	}

	pub fn map_rgb_f(&self, r: f32, g: f32, b: f32) -> Color
	{
		Color(ALLEGRO_COLOR{r: r, g: g, b: b, a: 1.0})
	}

	pub fn map_rgba_f(&self, r: f32, g: f32, b: f32, a: f32) -> Color
	{
		Color(ALLEGRO_COLOR{r: r, g: g, b: b, a: a})
	}

	pub fn create_event_queue(&self) -> Option<EventQueue>
	{
		new_queue()
	}
}
