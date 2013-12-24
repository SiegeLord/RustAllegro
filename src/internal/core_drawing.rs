use std::libc::*;

use internal::bitmap_like::*;
use internal::color::*;
use ffi::*;

pub use self::bitmap_drawing_flag::*;

flag_type!(
	mod bitmap_drawing_flag
	{
		BitmapDrawingFlags
		{
			FLIP_NONE = 0x1,
			FLIP_HORIZONTAL = ALLEGRO_FLIP_HORIZONTAL << 1,
			FLIP_VERTICAL = ALLEGRO_FLIP_VERTICAL << 1
		}
	}
)

impl ::internal::core::Core
{
	pub fn set_target_bitmap<B: BitmapLike>(&self, bitmap: &B)
	{
		unsafe
		{
			al_set_target_bitmap(bitmap.get_bitmap());
		}
	}

	pub fn clear_to_color(&self, color: Color)
	{
		unsafe
		{
			al_clear_to_color(*color);
		}
	}

	pub fn draw_pixel(&self, x: f32, y: f32, color: Color)
	{
		unsafe
		{
			al_draw_pixel(x as c_float, y as c_float, *color);
		}
	}

	pub fn put_pixel(&self, x: i32, y: i32, color: Color)
	{
		unsafe
		{
			al_put_pixel(x as c_int, y as c_int, *color);
		}
	}

	pub fn put_blended_pixel(&self, x: i32, y: i32, color: Color)
	{
		unsafe
		{
			al_put_blended_pixel(x as c_int, y as c_int, *color);
		}
	}

	pub fn draw_bitmap<T: BitmapLike>(&self, bitmap: &T, dx: f32, dy: f32, flags: BitmapDrawingFlags)
	{
		unsafe
		{
			al_draw_bitmap(bitmap.get_bitmap(), dx as c_float, dy as c_float, (flags.get() >> 1) as c_int);
		}
	}

	pub fn draw_bitmap_region<T: BitmapLike>(&self, bitmap: &T, sx: f32, sy: f32, sw: f32, sh: f32, dx: f32, dy: f32, flags: BitmapDrawingFlags)
    {
        unsafe
        {
            al_draw_bitmap_region(bitmap.get_bitmap(), sx as c_float, sy as c_float, sw as c_float, sh as c_float, dx as c_float, dy as c_float, (flags.get() >> 1) as c_int);
        }
    }

	pub fn draw_scaled_bitmap<T: BitmapLike>(&self, bitmap: &T, sx: f32, sy: f32, sw: f32, sh: f32, dx: f32, dy: f32, dw: f32, dh: f32, flags: BitmapDrawingFlags)
	{
		unsafe
		{
			al_draw_scaled_bitmap(bitmap.get_bitmap(), sx as c_float, sy as c_float, sw as c_float, sh as c_float, dx as c_float, dy as c_float, dw as c_float, dh as c_float, (flags.get() >> 1) as c_int);
		}
	}

	pub fn draw_rotated_bitmap<T: BitmapLike>(&self, bitmap: &T, cx: f32, cy: f32, dx: f32, dy: f32, angle: f32, flags: BitmapDrawingFlags)
	{
		unsafe
		{
			al_draw_rotated_bitmap(bitmap.get_bitmap(), cx as c_float, cy as c_float, dx as c_float, dy as c_float, angle as c_float, (flags.get() >> 1) as c_int);
		}
	}

	pub fn draw_scaled_rotated_bitmap<T: BitmapLike>(&self, bitmap: &T, cx: f32, cy: f32, dx: f32, dy: f32, xscale: f32, yscale: f32, angle: f32, flags: BitmapDrawingFlags)
	{
		unsafe
		{
			al_draw_scaled_rotated_bitmap(bitmap.get_bitmap(), cx as c_float, cy as c_float, dx as c_float, dy as c_float, xscale as c_float, yscale as c_float, angle as c_float, (flags.get() >> 1) as c_int);
		}
	}

	pub fn draw_tinted_bitmap<T: BitmapLike>(&self, bitmap: &T, tint: Color, dx: f32, dy: f32, flags: BitmapDrawingFlags)
	{
		unsafe
		{
			al_draw_tinted_bitmap(bitmap.get_bitmap(), *tint, dx as c_float, dy as c_float, (flags.get() >> 1) as c_int);
		}
	}

	pub fn draw_tinted_bitmap_region<T: BitmapLike>(&self, bitmap: &T, tint: Color, sx: f32, sy: f32, sw: f32, sh: f32, dx: f32, dy: f32, flags: BitmapDrawingFlags)
	{
		unsafe
		{
			al_draw_tinted_bitmap_region(bitmap.get_bitmap(), *tint, sx as c_float, sy as c_float, sw as c_float, sh as c_float, dx as c_float, dy as c_float, (flags.get() >> 1) as c_int);
		}
	}

	pub fn draw_tinted_scaled_bitmap<T: BitmapLike>(&self, bitmap: &T, tint: Color, sx: f32, sy: f32, sw: f32, sh: f32, dx: f32, dy: f32, dw: f32, dh: f32, flags: BitmapDrawingFlags)
	{
		unsafe
		{
			al_draw_tinted_scaled_bitmap(bitmap.get_bitmap(), *tint, sx as c_float, sy as c_float, sw as c_float, sh as c_float, dx as c_float, dy as c_float, dw as c_float, dh as c_float, (flags.get() >> 1) as c_int);
		}
	}

	pub fn draw_tinted_rotated_bitmap<T: BitmapLike>(&self, bitmap: &T, tint: Color, cx: f32, cy: f32, dx: f32, dy: f32, angle: f32, flags: BitmapDrawingFlags)
	{
		unsafe
		{
			al_draw_tinted_rotated_bitmap(bitmap.get_bitmap(), *tint, cx as c_float, cy as c_float, dx as c_float, dy as c_float, angle as c_float, (flags.get() >> 1) as c_int);
		}
	}

	pub fn draw_tinted_scaled_rotated_bitmap<T: BitmapLike>(&self, bitmap: &T, tint: Color, cx: f32, cy: f32, dx: f32, dy: f32, xscale: f32, yscale: f32, angle: f32, flags: BitmapDrawingFlags)
	{
		unsafe
		{
			al_draw_tinted_scaled_rotated_bitmap(bitmap.get_bitmap(), *tint, cx as c_float, cy as c_float, dx as c_float, dy as c_float, xscale as c_float, yscale as c_float, angle as c_float, (flags.get() >> 1) as c_int);
		}
	}

	pub fn draw_tinted_scaled_rotated_bitmap_region<T: BitmapLike>(&self, bitmap: &T, sx: f32, sy: f32, sw: f32, sh: f32, tint: Color, cx: f32, cy: f32, dx: f32, dy: f32, xscale: f32, yscale: f32, angle: f32, flags: BitmapDrawingFlags)
	{
		unsafe
		{
			al_draw_tinted_scaled_rotated_bitmap_region(bitmap.get_bitmap(), sx as c_float, sy as c_float, sw as c_float, sh as c_float, *tint, cx as c_float, cy as c_float, dx as c_float, dy as c_float, xscale as c_float, yscale as c_float, angle as c_float, (flags.get() >> 1) as c_int);
		}
	}

	pub fn set_clipping_rectangle(&self, x: i32, y: i32, width: i32, height: i32)
	{
		unsafe
		{
			al_set_clipping_rectangle(x as c_int, y as c_int, width as c_int, height as c_int);
		}
	}

	pub fn reset_clipping_rectangle(&self)
	{
		unsafe
		{
			al_reset_clipping_rectangle();
		}
	}

	pub fn get_clipping_rectangle(&self) -> (i32, i32, i32, i32)
	{
		unsafe
		{
			let mut x: c_int = 0;
			let mut y: c_int = 0;
			let mut width: c_int = 0;
			let mut height: c_int = 0;
			al_get_clipping_rectangle(&mut x, &mut y, &mut width, &mut height);
			(x as i32, y as i32, width as i32, height as i32)
		}
	}
}
