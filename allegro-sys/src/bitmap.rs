// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;

use color::ALLEGRO_COLOR;
use allegro_util::c_bool;

opaque!(ALLEGRO_BITMAP);

pub const ALLEGRO_MEMORY_BITMAP: u32 = 1;
pub const ALLEGRO_KEEP_BITMAP_FORMAT: u32 = 2;
pub const ALLEGRO_FORCE_LOCKING: u32 = 4;
pub const ALLEGRO_NO_PRESERVE_TEXTURE: u32 = 8;
pub const ALLEGRO_ALPHA_TEST: u32 = 16;
pub const _ALLEGRO_INTERNAL_OPENGL: u32 = 32;
pub const ALLEGRO_MIN_LINEAR: u32 = 64;
pub const ALLEGRO_MAG_LINEAR: u32 = 128;
pub const ALLEGRO_MIPMAP: u32 = 256;
pub const ALLEGRO_NO_PREMULTIPLIED_ALPHA: u32 = 512;
pub const ALLEGRO_VIDEO_BITMAP: u32 = 1024;

#[repr(C)]
pub struct ALLEGRO_LOCKED_REGION {
	pub data: *const c_void,
	pub format: c_int,
	pub pitch: c_int,
	pub pixel_size: c_int,
}

pub const ALLEGRO_LOCK_READWRITE: u32 = 0;
pub const ALLEGRO_LOCK_READONLY: u32 = 1;
pub const ALLEGRO_LOCK_WRITEONLY: u32 = 2;

extern "C"
{
	pub fn al_set_new_bitmap_format(format: c_int);
	pub fn al_set_new_bitmap_flags(flags: c_int);
	pub fn al_get_new_bitmap_format() -> c_int;
	pub fn al_get_new_bitmap_flags() -> c_int;
	pub fn al_add_new_bitmap_flag(flag: c_int);

	pub fn al_get_bitmap_width(bitmap: *mut ALLEGRO_BITMAP) -> c_int;
	pub fn al_get_bitmap_height(bitmap: *mut ALLEGRO_BITMAP) -> c_int;
	pub fn al_get_bitmap_format(bitmap: *mut ALLEGRO_BITMAP) -> c_int;
	pub fn al_get_bitmap_flags(bitmap: *mut ALLEGRO_BITMAP) -> c_int;

	pub fn al_create_bitmap(w: c_int, h: c_int) -> *mut ALLEGRO_BITMAP;
	pub fn al_destroy_bitmap(bitmap: *mut ALLEGRO_BITMAP);

	pub fn al_put_pixel(x: c_int, y: c_int, color: ALLEGRO_COLOR);
	pub fn al_put_blended_pixel(x: c_int, y: c_int, color: ALLEGRO_COLOR);
	pub fn al_get_pixel(bitmap: *mut ALLEGRO_BITMAP, x: c_int, y: c_int) -> ALLEGRO_COLOR;

	pub fn al_convert_mask_to_alpha(bitmap: *mut ALLEGRO_BITMAP, mask_color: ALLEGRO_COLOR);

	pub fn al_set_clipping_rectangle(x: c_int, y: c_int, width: c_int, height: c_int);
	pub fn al_reset_clipping_rectangle();
	pub fn al_get_clipping_rectangle(x: *mut c_int, y: *mut c_int, w: *mut c_int, h: *mut c_int);

	pub fn al_create_sub_bitmap(parent: *mut ALLEGRO_BITMAP, x: c_int, y: c_int, w: c_int, h: c_int) -> *mut ALLEGRO_BITMAP;
	pub fn al_is_sub_bitmap(bitmap: *mut ALLEGRO_BITMAP) -> c_bool;
	pub fn al_get_parent_bitmap(bitmap: *mut ALLEGRO_BITMAP) -> *mut ALLEGRO_BITMAP;

	pub fn al_clone_bitmap(bitmap: *mut ALLEGRO_BITMAP) -> *mut ALLEGRO_BITMAP;

	pub fn al_lock_bitmap(bitmap: *mut ALLEGRO_BITMAP, format: c_int, flags: c_int) -> *mut ALLEGRO_LOCKED_REGION;
	pub fn al_unlock_bitmap(bitmap: *mut ALLEGRO_BITMAP);
}
