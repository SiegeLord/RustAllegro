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

	pub fn get_num_video_adapters(&self) -> int
	{
		unsafe
		{
			al_get_num_video_adapters() as int
		}
	}
	
	pub fn get_monitor_info(&self, adapter: int, info: &mut MonitorInfo) -> bool
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

	pub fn rest(&self, seconds: f64)
	{
		unsafe
		{
			al_rest(seconds as c_double);
		}
	}
	
	pub fn create_display(&self, w: int, h: int) -> Option<Display>
	{
		new_display(w, h)
	}
	
	pub fn create_display_with_options(&self, w: int, h: int, opt: &DisplayOptions) -> Option<Display>
	{
		new_display_with_options(w, h, opt)
	}

	pub fn create_bitmap(&self, w: int, h: int) -> Option<Bitmap>
	{
		new_bitmap(w, h)
	}

	pub fn create_bitmap_with_options(&self, w: int, h: int, opt: &BitmapOptions) -> Option<Bitmap>
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

	pub fn map_rgb_f(&self, r: float, g: float, b: float) -> Color
	{
		Color(ALLEGRO_COLOR{r: r as f32, g: g as f32, b: b as f32, a: 1.0})
	}

	pub fn map_rgba_f(&self, r: float, g: float, b: float, a: float) -> Color
	{
		Color(ALLEGRO_COLOR{r: r as f32, g: g as f32, b: b as f32, a: a as f32})
	}

	pub fn create_event_queue(&self) -> Option<EventQueue>
	{
		new_queue()
	}
}
