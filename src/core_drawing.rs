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

	fn draw_bitmap<T: BitmapLike>(&self, bitmap: &T, x: float, y: float, flags: BitmapDrawingFlags)
	{
		target_bitmap_check(self.get_target_bitmap());
		unsafe
		{
			al_draw_bitmap(bitmap.get_bitmap(), x as c_float, y as c_float, (flags.get() << 1) as c_int);
		}
	}
}
