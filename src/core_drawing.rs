use std::libc::*;

use color::*;
use ffi::*;
pub use self::bitmap_flag::*;

fn target_bitmap_check(desired_target: *mut ALLEGRO_BITMAP)
{
	unsafe
	{
		if al_get_target_bitmap() != desired_target
		{
			al_set_target_bitmap(desired_target)
		}
	}
}

flag_type!(
	mod bitmap_flag
	{
		BitmapDrawingFlags
		{
			FLIP_NONE = 0x1,
			FLIP_HORIZONTAL = 0x2,
			FLIP_VERTICAL = 0x4
		}
	}
)

pub trait BitmapLike
{
	fn get_bitmap(&self) -> *mut ALLEGRO_BITMAP;
}

pub trait DrawTarget
{
	fn get_target_bitmap(&self) -> *mut ALLEGRO_BITMAP;
}

pub trait CoreDrawing : DrawTarget
{
	fn clear_to_color(&self, color: Color)
	{
		target_bitmap_check(self.get_target_bitmap());
		unsafe
		{
			al_clear_to_color(*color);
		}
	}

	fn draw_pixel(&self, x: float, y: float, color: Color)
	{
		target_bitmap_check(self.get_target_bitmap());
		unsafe
		{
			al_draw_pixel(x as c_float, y as c_float, *color);
		}
	}

	fn draw_bitmap<T: BitmapLike>(&self, bitmap: &T, dx: float, dy: float, flags: BitmapDrawingFlags)
	{
		target_bitmap_check(self.get_target_bitmap());
		unsafe
		{
			al_draw_bitmap(bitmap.get_bitmap(), dx as c_float, dy as c_float, (flags.get() << 1) as c_int);
		}
	}

	fn draw_bitmap_region<T: BitmapLike>(&self, bitmap: &T, sx: float, sy: float, sw: float, sh: float, dx: float, dy: float, flags: BitmapDrawingFlags)
    {
        target_bitmap_check(self.get_target_bitmap());
        unsafe
        {
            al_draw_bitmap_region(bitmap.get_bitmap(), sx as c_float, sy as c_float, sw as c_float, sh as c_float, dx as c_float, dy as c_float, (flags.get() << 1) as c_int);
        }
    }

	fn draw_scaled_bitmap<T: BitmapLike>(&self, bitmap: &T, sx: float, sy: float, sw: float, sh: float, dx: float, dy: float, dw: float, dh: float, flags: BitmapDrawingFlags)
	{
		target_bitmap_check(self.get_target_bitmap());
		unsafe
		{
			al_draw_scaled_bitmap(bitmap.get_bitmap(), sx as c_float, sy as c_float, sw as c_float, sh as c_float, dx as c_float, dy as c_float, dw as c_float, dh as c_float, (flags.get() << 1) as c_int);
		}
	}

	fn draw_rotated_bitmap<T: BitmapLike>(&self, bitmap: &T, cx: float, cy: float, dx: float, dy: float, angle: float, flags: BitmapDrawingFlags)
	{
		target_bitmap_check(self.get_target_bitmap());
		unsafe
		{
			al_draw_rotated_bitmap(bitmap.get_bitmap(), cx as c_float, cy as c_float, dx as c_float, dy as c_float, angle as c_float, (flags.get() << 1) as c_int);
		}
	}

	fn draw_scaled_rotated_bitmap<T: BitmapLike>(&self, bitmap: &T, cx: float, cy: float, dx: float, dy: float, xscale: float, yscale: float, angle: float, flags: BitmapDrawingFlags)
	{
		target_bitmap_check(self.get_target_bitmap());
		unsafe
		{
			al_draw_scaled_rotated_bitmap(bitmap.get_bitmap(), cx as c_float, cy as c_float, dx as c_float, dy as c_float, xscale as c_float, yscale as c_float, angle as c_float, (flags.get() << 1) as c_int);
		}
	}

	fn draw_tinted_bitmap<T: BitmapLike>(&self, bitmap: &T, tint: Color, dx: float, dy: float, flags: BitmapDrawingFlags)
	{
		target_bitmap_check(self.get_target_bitmap());
		unsafe
		{
			al_draw_tinted_bitmap(bitmap.get_bitmap(), *tint, dx as c_float, dy as c_float, (flags.get() << 1) as c_int);
		}
	}

	fn draw_tinted_bitmap_region<T: BitmapLike>(&self, bitmap: &T, tint: Color, sx: float, sy: float, sw: float, sh: float, dx: float, dy: float, flags: BitmapDrawingFlags)
	{
		target_bitmap_check(self.get_target_bitmap());
		unsafe
		{
			al_draw_tinted_bitmap_region(bitmap.get_bitmap(), *tint, sx as c_float, sy as c_float, sw as c_float, sh as c_float, dx as c_float, dy as c_float, (flags.get() << 1) as c_int);
		}
	}

	fn draw_tinted_scaled_bitmap<T: BitmapLike>(&self, bitmap: &T, tint: Color, sx: float, sy: float, sw: float, sh: float, dx: float, dy: float, dw: float, dh: float, flags: BitmapDrawingFlags)
	{
		target_bitmap_check(self.get_target_bitmap());
		unsafe
		{
			al_draw_tinted_scaled_bitmap(bitmap.get_bitmap(), *tint, sx as c_float, sy as c_float, sw as c_float, sh as c_float, dx as c_float, dy as c_float, dw as c_float, dh as c_float, (flags.get() << 1) as c_int);
		}
	}

	fn draw_tinted_rotated_bitmap<T: BitmapLike>(&self, bitmap: &T, tint: Color, cx: float, cy: float, dx: float, dy: float, angle: float, flags: BitmapDrawingFlags)
	{
		target_bitmap_check(self.get_target_bitmap());
		unsafe
		{
			al_draw_tinted_rotated_bitmap(bitmap.get_bitmap(), *tint, cx as c_float, cy as c_float, dx as c_float, dy as c_float, angle as c_float, (flags.get() << 1) as c_int);
		}
	}

	fn draw_tinted_scaled_rotated_bitmap<T: BitmapLike>(&self, bitmap: &T, tint: Color, cx: float, cy: float, dx: float, dy: float, xscale: float, yscale: float, angle: float, flags: BitmapDrawingFlags)
	{
		target_bitmap_check(self.get_target_bitmap());
		unsafe
		{
			al_draw_tinted_scaled_rotated_bitmap(bitmap.get_bitmap(), *tint, cx as c_float, cy as c_float, dx as c_float, dy as c_float, xscale as c_float, yscale as c_float, angle as c_float, (flags.get() << 1) as c_int);
		}
	}

	fn draw_tinted_scaled_rotated_bitmap_region<T: BitmapLike>(&self, bitmap: &T, sx: float, sy: float, sw: float, sh: float, tint: Color, cx: float, cy: float, dx: float, dy: float, xscale: float, yscale: float, angle: float, flags: BitmapDrawingFlags)
	{
		target_bitmap_check(self.get_target_bitmap());
		unsafe
		{
			al_draw_tinted_scaled_rotated_bitmap_region(bitmap.get_bitmap(), sx as c_float, sy as c_float, sw as c_float, sh as c_float, *tint, cx as c_float, cy as c_float, dx as c_float, dy as c_float, xscale as c_float, yscale as c_float, angle as c_float, (flags.get() << 1) as c_int);
		}
	}
}
