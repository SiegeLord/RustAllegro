// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;
use std::mem;
use std::kinds::marker::NoSend;
use std::task::spawn;
use std::sync::{Arc, Mutex};

use ffi::*;

use internal::events::{EventSource, new_event_source_ref};
use internal::keycodes::{KeyCode, KeyModifier};
use internal::display::{Display, DisplayOption, DisplayOptionImportance, DisplayFlags};
use internal::color::{Color, PixelFormat};
use internal::bitmap_like::{BitmapLike, BitmapFlags};
use internal::transformations::{Transform, new_transform_wrap};
use rust_util::Flag;

flag_type!(
	BitmapDrawingFlags
	{
		FLIP_NONE = 0x1,
		FLIP_HORIZONTAL = ALLEGRO_FLIP_HORIZONTAL << 1,
		FLIP_VERTICAL = ALLEGRO_FLIP_VERTICAL << 1
	}
)

pub mod external
{
	pub use super::
	{
		Core,
		BitmapDrawingFlags,
		FLIP_NONE,
		FLIP_HORIZONTAL,
		FLIP_VERTICAL
	};
}

pub static mut dummy_target: *mut ALLEGRO_BITMAP = 0 as *mut ALLEGRO_BITMAP;

pub struct Core
{
	keyboard_event_source: Option<EventSource>,
	mouse_event_source: Option<EventSource>,
	joystick_event_source: Option<EventSource>,
	mutex: Arc<Mutex<()>>,
	no_send_marker: NoSend,
}

impl Core
{
	pub fn init() -> Result<Core, String>
	{
		use std::sync::{Once, ONCE_INIT};
		static mut run_once: Once = ONCE_INIT;

		let mut res = Err("Already initialized.".to_string());
		unsafe
		{
			run_once.doit(||
			{
				res = if al_install_system(ALLEGRO_VERSION_INT as c_int, None) != 0
				{
					al_set_new_bitmap_flags(ALLEGRO_MEMORY_BITMAP as i32);
					dummy_target = al_create_bitmap(1, 1);
					al_set_new_bitmap_flags(0);
					if dummy_target.is_null()
					{
						Err("Failed to create the dummy target... something is very wrong!".to_string())
					}
					else
					{
						al_set_target_bitmap(dummy_target);
						Ok
						(
							Core
							{
								keyboard_event_source: None,
								mouse_event_source: None,
								joystick_event_source: None,
								mutex: Arc::new(Mutex::new(())),
								no_send_marker: NoSend,
							}
						)
					}
				}
				else
				{
					let version = al_get_allegro_version();
					let major = version >> 24;
					let minor = (version >> 16) & 255;
					let revision = (version >> 8) & 255;
					let release = version & 255;

					Err(format!("The system Allegro version ({}.{}.{}.{}) does not match the version of this binding ({}.{}.{}.{})",
					    major, minor, revision, release,
					    ALLEGRO_VERSION, ALLEGRO_SUB_VERSION, ALLEGRO_WIP_VERSION, ALLEGRO_RELEASE_NUMBER))
				};
			});
		}
		res
	}

	pub fn spawn(&self, thread_proc: proc(Core):Send)
	{
		let mutex = self.get_core_mutex();
		spawn(proc()
		{
			thread_proc(Core
			{
				keyboard_event_source: None,
				mouse_event_source: None,
				joystick_event_source: None,
				mutex: mutex,
				no_send_marker: NoSend,
			});
		});
	}

	pub fn get_core_mutex(&self) -> Arc<Mutex<()>>
	{
		self.mutex.clone()
	}

	pub fn get_num_video_adapters(&self) -> i32
	{
		unsafe
		{
			al_get_num_video_adapters() as i32
		}
	}

	pub fn get_monitor_info(&self, adapter: i32) -> Result<(i32, i32, i32, i32), ()>
	{
		unsafe
		{
			let mut c_info = ALLEGRO_MONITOR_INFO{ x1: 0, y1: 0, x2: 0, y2: 0 };
			if al_get_monitor_info(adapter as c_int, mem::transmute(&mut c_info)) != 0
			{
				Ok((c_info.x1 as i32, c_info.y1 as i32, c_info.x2 as i32, c_info.y2 as i32))
			}
			else
			{
				Err(())
			}
		}
	}

	pub fn rest(&self, seconds: f64)
	{
		unsafe
		{
			al_rest(seconds as c_double);
		}
	}

	pub fn get_time(&self) -> f64
	{
		unsafe
		{
			al_get_time() as f64
		}
	}

	pub fn install_keyboard(&self) -> Result<(), ()>
	{
		unsafe
		{
			if al_install_keyboard() != 0
			{
				Ok(())
			}
			else
			{
				Err(())
			}
		}
	}

	pub fn is_keyboard_installed(&self) -> bool
	{
		unsafe
		{
			al_is_keyboard_installed() != 0
		}
	}

	pub fn get_keyboard_event_source(&mut self) -> &EventSource
	{
		if self.keyboard_event_source.is_none() && self.is_keyboard_installed()
		{
			unsafe
			{
				self.keyboard_event_source = Some(new_event_source_ref(al_get_keyboard_event_source()));
			}
		}

		self.keyboard_event_source.as_ref().expect("Keyboard not installed")
	}

	pub fn set_keyboard_leds(&self, leds: KeyModifier) -> Result<(), ()>
	{
		assert!(self.is_keyboard_installed());
		unsafe
		{
			if al_set_keyboard_leds(leds.get() as c_int) != 0
			{
				Ok(())
			}
			else
			{
				Err(())
			}
		}
	}

	pub fn keycode_to_name(&self, k: KeyCode) -> String
	{
		assert!(self.is_keyboard_installed());
		unsafe
		{
			String::from_raw_buf(al_keycode_to_name(k as c_int) as *const _)
		}
	}

	pub fn install_mouse(&self) -> Result<(), ()>
	{
		unsafe
		{
			if al_install_mouse() != 0
			{
				Ok(())
			}
			else
			{
				Err(())
			}
		}
	}

	pub fn is_mouse_installed(&self) -> bool
	{
		unsafe
		{
			al_is_mouse_installed() != 0
		}
	}

	pub fn get_mouse_event_source(&mut self) -> &EventSource
	{
		if self.mouse_event_source.is_none() && self.is_mouse_installed()
		{
			unsafe
			{
				self.mouse_event_source = Some(new_event_source_ref(al_get_mouse_event_source()));
			}
		}

		self.mouse_event_source.as_ref().expect("Mouse not installed")
	}

	pub fn install_joystick(&self) -> Result<(), ()>
	{
		unsafe
		{
			if al_install_joystick() != 0
			{
				Ok(())
			}
			else
			{
				Err(())
			}
		}
	}

	pub fn is_joystick_installed(&self) -> bool
	{
		unsafe
		{
			al_is_joystick_installed() != 0
		}
	}

	pub fn get_joystick_event_source(&mut self) -> &EventSource
	{
		if self.joystick_event_source.is_none() && self.is_joystick_installed()
		{
			unsafe
			{
				self.joystick_event_source = Some(new_event_source_ref(al_get_joystick_event_source()));
			}
		}

		self.joystick_event_source.as_ref().expect("Joystick not installed")
	}

	pub fn reconfigure_joysticks(&self) -> Result<(), ()>
	{
		assert!(self.is_joystick_installed());
		unsafe
		{
			if al_reconfigure_joysticks() != 0
			{
				Ok(())
			}
			else
			{
				Err(())
			}
		}
	}

	pub fn get_num_joysticks(&self) -> i32
	{
		assert!(self.is_joystick_installed());
		unsafe
		{
			al_get_num_joysticks() as i32
		}
	}

	pub fn get_mouse_num_buttons(&self) -> u32
	{
		assert!(self.is_mouse_installed())
		unsafe
		{
			al_get_mouse_num_buttons() as u32
		}
	}

	pub fn get_mouse_num_axes(&self) -> u32
	{
		assert!(self.is_mouse_installed())
		unsafe
		{
			al_get_mouse_num_axes() as u32
		}
	}

	pub fn set_mouse_xy(&self, display: &Display, x: i32, y: i32) -> Result<(), ()>
	{
		assert!(self.is_mouse_installed())
		unsafe
		{
			if al_set_mouse_xy(display.get_allegro_display(), x as c_int, y as c_int) != 0
			{
				Ok(())
			}
			else
			{
				Err(())
			}
		}
	}

	pub fn set_mouse_z(&self, z: i32) -> Result<(), ()>
	{
		assert!(self.is_mouse_installed())
		unsafe
		{
			if al_set_mouse_z(z as c_int) != 0
			{
				Ok(())
			}
			else
			{
				Err(())
			}
		}
	}

	pub fn set_mouse_w(&self, w: i32) -> Result<(), ()>
	{
		assert!(self.is_mouse_installed())
		unsafe
		{
			if al_set_mouse_w(w as c_int) != 0
			{
				Ok(())
			}
			else
			{
				Err(())
			}
		}
	}

	pub fn set_mouse_axis(&self, axis: i32, value: i32) -> Result<(), ()>
	{
		assert!(self.is_mouse_installed())
		unsafe
		{
			if al_set_mouse_axis(axis as c_int, value as c_int) != 0
			{
				Ok(())
			}
			else
			{
				Err(())
			}
		}
	}

	pub fn grab_mouse(&self, display: &Display) -> Result<(), ()>
	{
		assert!(self.is_mouse_installed())
		unsafe
		{
			if al_grab_mouse(display.get_allegro_display()) != 0
			{
				Ok(())
			}
			else
			{
				Err(())
			}
		}
	}

	pub fn ungrab_mouse(&self) -> Result<(), ()>
	{
		assert!(self.is_mouse_installed())
		unsafe
		{
			if al_ungrab_mouse() != 0
			{
				Ok(())
			}
			else
			{
				Err(())
			}
		}
	}

	pub fn set_new_bitmap_flags(&self, flags: BitmapFlags)
	{
		unsafe
		{
			al_set_new_bitmap_flags(flags.get() as c_int);
		}
	}

	pub fn get_new_bitmap_flags(&self) -> BitmapFlags
	{
		unsafe
		{
			mem::transmute(al_get_new_bitmap_flags() as u32)
		}
	}

	pub fn set_new_bitmap_format(&self, format: PixelFormat)
	{
		unsafe
		{
			al_set_new_bitmap_format(format as c_int);
		}
	}

	pub fn get_new_bitmap_format(&self) -> PixelFormat
	{
		unsafe
		{
			mem::transmute(al_get_new_bitmap_format() as u32)
		}
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

	pub fn set_target_bitmap<T: BitmapLike>(&self, bmp: &T)
	{
		unsafe
		{
			al_set_target_bitmap(bmp.get_allegro_bitmap());
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
			al_draw_bitmap(bitmap.get_allegro_bitmap(), dx as c_float, dy as c_float, (flags.get() >> 1) as c_int);
		}
	}

	pub fn draw_bitmap_region<T: BitmapLike>(&self, bitmap: &T, sx: f32, sy: f32, sw: f32, sh: f32, dx: f32, dy: f32, flags: BitmapDrawingFlags)
    {
        unsafe
        {
            al_draw_bitmap_region(bitmap.get_allegro_bitmap(), sx as c_float, sy as c_float, sw as c_float, sh as c_float, dx as c_float, dy as c_float, (flags.get() >> 1) as c_int);
        }
    }

	pub fn draw_scaled_bitmap<T: BitmapLike>(&self, bitmap: &T, sx: f32, sy: f32, sw: f32, sh: f32, dx: f32, dy: f32, dw: f32, dh: f32, flags: BitmapDrawingFlags)
	{
		unsafe
		{
			al_draw_scaled_bitmap(bitmap.get_allegro_bitmap(), sx as c_float, sy as c_float, sw as c_float, sh as c_float, dx as c_float, dy as c_float, dw as c_float, dh as c_float, (flags.get() >> 1) as c_int);
		}
	}

	pub fn draw_rotated_bitmap<T: BitmapLike>(&self, bitmap: &T, cx: f32, cy: f32, dx: f32, dy: f32, angle: f32, flags: BitmapDrawingFlags)
	{
		unsafe
		{
			al_draw_rotated_bitmap(bitmap.get_allegro_bitmap(), cx as c_float, cy as c_float, dx as c_float, dy as c_float, angle as c_float, (flags.get() >> 1) as c_int);
		}
	}

	pub fn draw_scaled_rotated_bitmap<T: BitmapLike>(&self, bitmap: &T, cx: f32, cy: f32, dx: f32, dy: f32, xscale: f32, yscale: f32, angle: f32, flags: BitmapDrawingFlags)
	{
		unsafe
		{
			al_draw_scaled_rotated_bitmap(bitmap.get_allegro_bitmap(), cx as c_float, cy as c_float, dx as c_float, dy as c_float, xscale as c_float, yscale as c_float, angle as c_float, (flags.get() >> 1) as c_int);
		}
	}

	pub fn draw_tinted_bitmap<T: BitmapLike>(&self, bitmap: &T, tint: Color, dx: f32, dy: f32, flags: BitmapDrawingFlags)
	{
		unsafe
		{
			al_draw_tinted_bitmap(bitmap.get_allegro_bitmap(), *tint, dx as c_float, dy as c_float, (flags.get() >> 1) as c_int);
		}
	}

	pub fn draw_tinted_bitmap_region<T: BitmapLike>(&self, bitmap: &T, tint: Color, sx: f32, sy: f32, sw: f32, sh: f32, dx: f32, dy: f32, flags: BitmapDrawingFlags)
	{
		unsafe
		{
			al_draw_tinted_bitmap_region(bitmap.get_allegro_bitmap(), *tint, sx as c_float, sy as c_float, sw as c_float, sh as c_float, dx as c_float, dy as c_float, (flags.get() >> 1) as c_int);
		}
	}

	pub fn draw_tinted_scaled_bitmap<T: BitmapLike>(&self, bitmap: &T, tint: Color, sx: f32, sy: f32, sw: f32, sh: f32, dx: f32, dy: f32, dw: f32, dh: f32, flags: BitmapDrawingFlags)
	{
		unsafe
		{
			al_draw_tinted_scaled_bitmap(bitmap.get_allegro_bitmap(), *tint, sx as c_float, sy as c_float, sw as c_float, sh as c_float, dx as c_float, dy as c_float, dw as c_float, dh as c_float, (flags.get() >> 1) as c_int);
		}
	}

	pub fn draw_tinted_rotated_bitmap<T: BitmapLike>(&self, bitmap: &T, tint: Color, cx: f32, cy: f32, dx: f32, dy: f32, angle: f32, flags: BitmapDrawingFlags)
	{
		unsafe
		{
			al_draw_tinted_rotated_bitmap(bitmap.get_allegro_bitmap(), *tint, cx as c_float, cy as c_float, dx as c_float, dy as c_float, angle as c_float, (flags.get() >> 1) as c_int);
		}
	}

	pub fn draw_tinted_scaled_rotated_bitmap<T: BitmapLike>(&self, bitmap: &T, tint: Color, cx: f32, cy: f32, dx: f32, dy: f32, xscale: f32, yscale: f32, angle: f32, flags: BitmapDrawingFlags)
	{
		unsafe
		{
			al_draw_tinted_scaled_rotated_bitmap(bitmap.get_allegro_bitmap(), *tint, cx as c_float, cy as c_float, dx as c_float, dy as c_float, xscale as c_float, yscale as c_float, angle as c_float, (flags.get() >> 1) as c_int);
		}
	}

	pub fn draw_tinted_scaled_rotated_bitmap_region<T: BitmapLike>(&self, bitmap: &T, sx: f32, sy: f32, sw: f32, sh: f32, tint: Color, cx: f32, cy: f32, dx: f32, dy: f32, xscale: f32, yscale: f32, angle: f32, flags: BitmapDrawingFlags)
	{
		unsafe
		{
			al_draw_tinted_scaled_rotated_bitmap_region(bitmap.get_allegro_bitmap(), sx as c_float, sy as c_float, sw as c_float, sh as c_float, *tint, cx as c_float, cy as c_float, dx as c_float, dy as c_float, xscale as c_float, yscale as c_float, angle as c_float, (flags.get() >> 1) as c_int);
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

	pub fn set_new_display_flags(&self, flags: DisplayFlags)
	{
		unsafe
		{
			al_set_new_display_flags(flags.get() as c_int);
		}
	}

	pub fn get_new_display_flags(&self) -> DisplayFlags
	{
		unsafe
		{
			mem::transmute(al_get_new_display_flags())
		}
	}

	pub fn set_new_display_refresh_rate(&self, rate: i32)
	{
		unsafe
		{
			al_set_new_display_refresh_rate(rate as c_int);
		}
	}

	pub fn get_new_display_refresh_rate(&self) -> i32
	{
		unsafe
		{
			al_get_new_display_refresh_rate() as i32
		}
	}

	pub fn set_new_display_adapter(&self, adapter: i32)
	{
		unsafe
		{
			al_set_new_display_adapter(adapter as c_int);
		}
	}

	pub fn get_new_display_adapter(&self) -> i32
	{
		unsafe
		{
			al_get_new_display_adapter() as i32
		}
	}

	pub fn set_new_window_position(&self, x: i32, y: i32)
	{
		unsafe
		{
			al_set_new_window_position(x as c_int, y as c_int);
		}
	}

	pub fn get_new_window_position(&self) -> (i32, i32)
	{
		unsafe
		{
			use std::mem::uninitialized;

			let mut x: c_int = uninitialized();
			let mut y: c_int = uninitialized();
			al_get_new_window_position(&mut x, &mut y);
			(x as i32, y as i32)
		}
	}

	pub fn reset_new_display_options(&self)
	{
		unsafe
		{
			al_reset_new_display_options();
		}
	}

	pub fn set_new_display_option(&self, option: DisplayOption, value: i32, importance: DisplayOptionImportance)
	{
		unsafe
		{
			al_set_new_display_option(option as c_int, value as c_int, importance as c_int);
		}
	}

	pub fn get_new_display_option(&self, option: DisplayOption) -> (i32, DisplayOptionImportance)
	{
		unsafe
		{
			use std::mem::uninitialized;

			let mut imp: c_int = uninitialized();

			let val = al_get_new_display_option(option as c_int, &mut imp);
			(val as i32, mem::transmute(imp))
		}
	}

	pub fn get_current_transform(&self) -> Transform
	{
		let t = unsafe
		{
			al_get_current_transform()
		};
		if t.is_null()
		{
			/* We always have a valid target */
			unreachable!();
		}
		unsafe
		{
			new_transform_wrap(*t)
		}
	}

	pub fn use_transform(&self, trans: &Transform)
	{
		unsafe
		{
			al_use_transform(&**trans);
		}
	}
}
