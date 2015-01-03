// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;

use bitmap::*;
use color::*;

pub const ALLEGRO_FLIP_HORIZONTAL: u32 = 1;
pub const ALLEGRO_FLIP_VERTICAL: u32 = 2;

extern "C"
{
	pub fn al_draw_bitmap(bitmap: *mut ALLEGRO_BITMAP, dx: c_float, dy: c_float, flags: c_int);
	pub fn al_draw_bitmap_region(bitmap: *mut ALLEGRO_BITMAP, sx: c_float, sy: c_float, sw: c_float, sh: c_float, dx: c_float, dy: c_float, flags: c_int);
	pub fn al_draw_scaled_bitmap(bitmap: *mut ALLEGRO_BITMAP, sx: c_float, sy: c_float, sw: c_float, sh: c_float, dx: c_float, dy: c_float, dw: c_float, dh: c_float, flags: c_int);
	pub fn al_draw_rotated_bitmap(bitmap: *mut ALLEGRO_BITMAP, cx: c_float, cy: c_float, dx: c_float, dy: c_float, angle: c_float, flags: c_int);
	pub fn al_draw_scaled_rotated_bitmap(bitmap: *mut ALLEGRO_BITMAP, cx: c_float, cy: c_float, dx: c_float, dy: c_float, xscale: c_float, yscale: c_float, angle: c_float, flags: c_int);
	pub fn al_draw_tinted_bitmap(bitmap: *mut ALLEGRO_BITMAP, tint: ALLEGRO_COLOR, dx: c_float, dy: c_float, flags: c_int);
	pub fn al_draw_tinted_bitmap_region(bitmap: *mut ALLEGRO_BITMAP, tint: ALLEGRO_COLOR, sx: c_float, sy: c_float, sw: c_float, sh: c_float, dx: c_float, dy: c_float, flags: c_int);
	pub fn al_draw_tinted_scaled_bitmap(bitmap: *mut ALLEGRO_BITMAP, tint: ALLEGRO_COLOR, sx: c_float, sy: c_float, sw: c_float, sh: c_float, dx: c_float, dy: c_float, dw: c_float, dh: c_float, flags: c_int);
	pub fn al_draw_tinted_rotated_bitmap(bitmap: *mut ALLEGRO_BITMAP, tint: ALLEGRO_COLOR, cx: c_float, cy: c_float, dx: c_float, dy: c_float, angle: c_float, flags: c_int);
	pub fn al_draw_tinted_scaled_rotated_bitmap(bitmap: *mut ALLEGRO_BITMAP, tint: ALLEGRO_COLOR, cx: c_float, cy: c_float, dx: c_float, dy: c_float, xscale: c_float, yscale: c_float, angle: c_float, flags: c_int);
	pub fn al_draw_tinted_scaled_rotated_bitmap_region(bitmap: *mut ALLEGRO_BITMAP, sx: c_float, sy: c_float, sw: c_float, sh: c_float, tint: ALLEGRO_COLOR, cx: c_float, cy: c_float, dx: c_float, dy: c_float, xscale: c_float, yscale: c_float, angle: c_float, flags: c_int);
}
