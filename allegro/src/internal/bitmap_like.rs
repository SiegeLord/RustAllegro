// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use std::mem;
use libc::*;

use internal::color::{Color, PixelFormat};
use allegro_util::Flag;

use ffi::*;

flag_type!{
	BitmapFlags
	{
		MEMORY_BITMAP = ALLEGRO_MEMORY_BITMAP,
		KEEP_BITMAP_FORMAT = ALLEGRO_KEEP_BITMAP_FORMAT,
		FORCE_LOCKING = ALLEGRO_FORCE_LOCKING,
		NO_PRESERVE_TEXTURE = ALLEGRO_NO_PRESERVE_TEXTURE,
		ALPHA_TEST = ALLEGRO_ALPHA_TEST,
		MIN_LINEAR = ALLEGRO_MIN_LINEAR,
		MAG_LINEAR = ALLEGRO_MAG_LINEAR,
		MIPMAP = ALLEGRO_MIPMAP,
		NO_PREMULTIPLIED_ALPHA = ALLEGRO_NO_PREMULTIPLIED_ALPHA,
		VIDEO_BITMAP = ALLEGRO_VIDEO_BITMAP
	}
}

pub trait BitmapLike
{
	fn get_allegro_bitmap(&self) -> *mut ALLEGRO_BITMAP;

	fn get_width(&self) -> i32
	{
		unsafe
		{
			al_get_bitmap_width(self.get_allegro_bitmap()) as i32
		}
	}

	fn get_height(&self) -> i32
	{
		unsafe
		{
			al_get_bitmap_height(self.get_allegro_bitmap()) as i32
		}
	}

	fn get_format(&self) -> PixelFormat
	{
		unsafe
		{
			mem::transmute(al_get_bitmap_format(self.get_allegro_bitmap()) as u32)
		}
	}

	fn get_flags(&self) -> BitmapFlags
	{
		unsafe
		{
			mem::transmute(al_get_bitmap_flags(self.get_allegro_bitmap()) as u32)
		}
	}

	fn get_pixel(&self, x: i32, y: i32) -> Color
	{
		unsafe
		{
			Color(al_get_pixel(self.get_allegro_bitmap(), x as c_int, y as c_int))
		}
	}

	fn convert_mask_to_alpha(&self, mask_color: Color)
	{
		unsafe
		{
			al_convert_mask_to_alpha(self.get_allegro_bitmap(), mask_color.0);
		}
	}
}
