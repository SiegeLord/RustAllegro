// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;
use std::mem;
use std::c_str::CString;

use internal::bitmap::*;
use internal::bitmap_like::*;
use internal::color::*;
use internal::core::dummy_target;
use internal::events::*;
use rust_util::Flag;

use ffi::*;

pub use rust_util::c_bool;

flag_type!(
	DisplayFlags
	{
		WINDOWED                  = ALLEGRO_WINDOWED,
		FULLSCREEN                = ALLEGRO_FULLSCREEN,
		OPENGL                    = ALLEGRO_OPENGL,
		DIRECT3D                  = ALLEGRO_DIRECT3D_INTERNAL,
		RESIZABLE                 = ALLEGRO_RESIZABLE,
		FRAMELESS                 = ALLEGRO_FRAMELESS,
		GENERATE_EXPOSE_EVENTS    = ALLEGRO_GENERATE_EXPOSE_EVENTS,
		OPENGL_3_0                = ALLEGRO_OPENGL_3_0,
		OPENGL_FORWARD_COMPATIBLE = ALLEGRO_OPENGL_FORWARD_COMPATIBLE,
		FULLSCREEN_WINDOW         = ALLEGRO_FULLSCREEN_WINDOW,
		MINIMIZED                 = ALLEGRO_MINIMIZED
	}
)

#[repr(u32)]
pub enum DisplayOption
{
	RedSize = ALLEGRO_RED_SIZE,
	GreenSize = ALLEGRO_GREEN_SIZE,
	BlueSize = ALLEGRO_BLUE_SIZE,
	AlphaSize = ALLEGRO_ALPHA_SIZE,
	RedShift = ALLEGRO_RED_SHIFT,
	GreenShift = ALLEGRO_GREEN_SHIFT,
	BlueShift = ALLEGRO_BLUE_SHIFT,
	AlphaShift = ALLEGRO_ALPHA_SHIFT,
	AccRedSize = ALLEGRO_ACC_RED_SIZE,
	AccGreenSize = ALLEGRO_ACC_GREEN_SIZE,
	AccBlueSize = ALLEGRO_ACC_BLUE_SIZE,
	AccAlphaSize = ALLEGRO_ACC_ALPHA_SIZE,
	Stereo = ALLEGRO_STEREO,
	AuxBuffers = ALLEGRO_AUX_BUFFERS,
	ColorSize = ALLEGRO_COLOR_SIZE,
	DepthSize = ALLEGRO_DEPTH_SIZE,
	StencilSize = ALLEGRO_STENCIL_SIZE,
	SampleBuffers = ALLEGRO_SAMPLE_BUFFERS,
	Samples = ALLEGRO_SAMPLES,
	RenderMethod = ALLEGRO_RENDER_METHOD,
	FloatColor = ALLEGRO_FLOAT_COLOR,
	FloatDepth = ALLEGRO_FLOAT_DEPTH,
	SingleBuffer = ALLEGRO_SINGLE_BUFFER,
	SwapMethod = ALLEGRO_SWAP_METHOD,
	CompatibleDisplay = ALLEGRO_COMPATIBLE_DISPLAY,
	UpdateDisplayRegion = ALLEGRO_UPDATE_DISPLAY_REGION,
	Vsync = ALLEGRO_VSYNC,
	MaxBitmapSize = ALLEGRO_MAX_BITMAP_SIZE,
	SupportNpotBitmap = ALLEGRO_SUPPORT_NPOT_BITMAP,
	CanDrawIntoBitmap = ALLEGRO_CAN_DRAW_INTO_BITMAP,
	SupportSeparateAlpha = ALLEGRO_SUPPORT_SEPARATE_ALPHA,
}

#[repr(u32)]
pub enum DisplayOptionImportance
{
	DontCare = ALLEGRO_DONTCARE,
	Require = ALLEGRO_REQUIRE,
	Suggest = ALLEGRO_SUGGEST,
}

#[repr(u32)]
pub enum DisplayOrientation
{
	DisplayOrientation0Degrees = ALLEGRO_DISPLAY_ORIENTATION_0_DEGREES,
	DisplayOrientation90Degrees = ALLEGRO_DISPLAY_ORIENTATION_90_DEGREES,
	DisplayOrientation180Degrees = ALLEGRO_DISPLAY_ORIENTATION_180_DEGREES,
	DisplayOrientation270Degrees = ALLEGRO_DISPLAY_ORIENTATION_270_DEGREES,
	DisplayOrientationFaceUp = ALLEGRO_DISPLAY_ORIENTATION_FACE_UP,
	DisplayOrientationFaceDown = ALLEGRO_DISPLAY_ORIENTATION_FACE_DOWN,
}

pub struct Display
{
	allegro_display: *mut ALLEGRO_DISPLAY,
	backbuffer: Bitmap,
	event_source: EventSource,
}

impl Display
{
	fn new(w: i32, h: i32) -> Result<Display, String>
	{
		unsafe
		{
			if !al_get_current_display().is_null()
			{
				Err("Only one display is allowed per thread.".to_string())
			}
			else
			{
				let d = al_create_display(w as c_int, h as c_int);
				if d.is_null()
				{
					Err("Could not create the display".to_string())
				}
				else
				{
					Ok
					(
						Display
						{
							allegro_display: d,
							backbuffer: new_bitmap_ref(al_get_backbuffer(d)),
							event_source: new_event_source_ref(al_get_display_event_source(d)),
						}
					)
				}
			}
		}
	}

	pub fn get_width(&self) -> i32
	{
		unsafe
		{
			al_get_display_width(self.allegro_display) as i32
		}
	}

	pub fn get_height(&self) -> i32
	{
		unsafe
		{
			al_get_display_height(self.allegro_display) as i32
		}
	}

	pub fn get_format(&self) -> PixelFormat
	{
		unsafe
		{
			mem::transmute(al_get_display_format(self.allegro_display) as u32)
		}
	}

	pub fn get_refresh_rate(&self) -> i32
	{
		unsafe
		{
			al_get_display_refresh_rate(self.allegro_display) as i32
		}
	}

	pub fn get_flags(&self) -> DisplayFlags
	{
		unsafe
		{
			mem::transmute(al_get_display_flags(self.allegro_display))
		}
	}

	pub fn set_flag(&self, flag: DisplayFlags, onoff: bool) -> bool
	{
		unsafe
		{
			al_set_display_flag(self.allegro_display, flag.get(), onoff as u8) != 0
		}
	}

	pub fn get_backbuffer<'l>(&'l self) -> &'l Bitmap
	{
		&self.backbuffer
	}

	pub fn acknowledge_resize(&self) -> Result<(), ()>
	{
		unsafe
		{
			if al_acknowledge_resize(self.allegro_display) != 0
			{
				Ok(())
			}
			else
			{
				Err(())
			}
		}
	}

	pub fn resize(&self, w: i32, h: i32) -> Result<(), ()>
	{
		unsafe
		{
			if al_resize_display(self.allegro_display, w as c_int, h as c_int) != 0
			{
				Ok(())
			}
			else
			{
				Err(())
			}
		}
	}

	pub fn set_icon<T: BitmapLike>(&self, icon: &T)
	{
		unsafe
		{
			al_set_display_icon(self.allegro_display, icon.get_allegro_bitmap());
		}
	}

	pub fn set_icons<'l, U: Iterator<&'l BitmapLike + 'l>>(&self, icons: U)
	{
		let mut c_icons: Vec<_> = icons.map(|b| b.get_allegro_bitmap()).collect();
		unsafe
		{
			al_set_display_icons(self.allegro_display, c_icons.len() as c_int, c_icons.as_mut_ptr());
		}
	}

	pub fn set_window_position(&self, x: i32, y: i32)
	{
		unsafe
		{
			al_set_window_position(self.allegro_display, x as c_int, y as c_int);
		}
	}

	pub fn get_window_position(&self) -> (i32, i32)
	{
		unsafe
		{
			let mut x = 0 as c_int;
			let mut y = 0 as c_int;
			al_get_window_position(self.allegro_display, &mut x, &mut y);
			(x as i32, y as i32)
		}
	}

	pub fn set_window_title(&self, title: &CString)
	{
		unsafe
		{
			al_set_window_title(self.allegro_display, title.as_ptr());
		}
	}

	pub fn get_option(&self, option: DisplayOption) -> i32
	{
		unsafe
		{
			al_get_display_option(self.allegro_display, option as c_int) as i32
		}
	}

	pub fn convert_bitmap<T: BitmapLike>(&self, bmp: &T) -> Result<Bitmap, ()>
	{
		clone_bitmap(bmp.get_allegro_bitmap())
	}

	pub fn get_event_source<'l>(&'l self) -> &'l EventSource
	{
		&self.event_source
	}

	pub fn get_allegro_display(&self) -> *mut ALLEGRO_DISPLAY
	{
		self.allegro_display
	}

	pub fn flip(&self)
	{
		unsafe
		{
			al_flip_display();
		}
	}

	pub fn update_region(&self, x: i32, y: i32, width: i32, height: i32)
	{
		unsafe
		{
			al_update_display_region(x as c_int, y as c_int, width as c_int, height as c_int);
		}
	}

	pub fn is_compatible_bitmap<T: BitmapLike>(&self, bitmap: &T) -> bool
	{
		unsafe
		{
			al_is_compatible_bitmap(bitmap.get_allegro_bitmap()) != 0
		}
	}

	pub fn wait_for_vsync(&self) -> Result<(), ()>
	{
		unsafe
		{
			if al_wait_for_vsync() != 0
			{
				Ok(())
			}
			else
			{
				Err(())
			}
		}
	}

	pub fn hold_bitmap_drawing(&self, hold: bool)
	{
		unsafe
		{
			al_hold_bitmap_drawing(hold as c_bool);
		}
	}

	pub fn is_bitmap_drawing_held(&self) -> bool
	{
		unsafe
		{
			al_is_bitmap_drawing_held() != 0
		}
	}
}

// Not Send just because of the marker in the event source
#[unsafe_destructor]
impl Drop for Display
{
	fn drop(&mut self)
	{
		unsafe
		{
			/* If it's a video bitmap, it's about to be reset, so we fall back to the dummy target */
			if al_get_bitmap_flags(al_get_target_bitmap()) & (ALLEGRO_VIDEO_BITMAP as i32) != 0
			{
				al_set_target_bitmap(dummy_target);
			}
			al_destroy_display(self.allegro_display);
		}
	}
}

impl ::internal::core::Core
{
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

	pub fn create_display(&self, w: i32, h: i32) -> Result<Display, String>
	{
		Display::new(w, h)
	}
}
