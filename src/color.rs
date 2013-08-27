use std::libc::*;

use ffi::*;

pub struct Color(ALLEGRO_COLOR);

impl Color
{
	pub fn map_rgb(r: u8, g: u8, b: u8) -> Color
	{
		unsafe
		{
			Color(al_map_rgb(r as c_uchar, g as c_uchar, b as c_uchar))
		}
	}

	pub fn map_rgb_f(r: float, g: float, b: float) -> Color
	{
		Color(ALLEGRO_COLOR{r: r as f32, g: g as f32, b: b as f32, a: 1.0})
	}
}
