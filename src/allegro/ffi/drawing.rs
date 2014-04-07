use libc::*;

use ffi::color::*;

extern "C"
{
	pub fn al_clear_to_color(color: ALLEGRO_COLOR);
	pub fn al_draw_pixel(x: c_float, y: c_float, color: ALLEGRO_COLOR);
}
