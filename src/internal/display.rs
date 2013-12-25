use std::cast;
use std::c_str::CString;
use std::i32;
use std::libc::*;
use std::num::Zero;
use std::ptr;

use internal::bitmap::*;
use internal::bitmap_like::*;
use internal::color::*;
use internal::core_drawing::*;
use internal::events::*;

use ffi::*;

pub use self::display_flag::*;
pub use rust_util::c_bool;

pub mod external
{
	pub use super::display_flag::*;
	pub use super::{Display, DisplayOption, DisplayOptionImportance, DisplayOrientation, DisplayOptions};
}

flag_type!(
	mod display_flag
	{
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
	}
)

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

pub enum DisplayOptionImportance
{
	DontCare = ALLEGRO_DONTCARE,
	Require = ALLEGRO_REQUIRE,
	Suggest = ALLEGRO_SUGGEST,
}

pub enum DisplayOrientation
{
	DisplayOrientation0Degrees = ALLEGRO_DISPLAY_ORIENTATION_0_DEGREES,
	DisplayOrientation90Degrees = ALLEGRO_DISPLAY_ORIENTATION_90_DEGREES,
	DisplayOrientation180Degrees = ALLEGRO_DISPLAY_ORIENTATION_180_DEGREES,
	DisplayOrientation270Degrees = ALLEGRO_DISPLAY_ORIENTATION_270_DEGREES,
	DisplayOrientationFaceUp = ALLEGRO_DISPLAY_ORIENTATION_FACE_UP,
	DisplayOrientationFaceDown = ALLEGRO_DISPLAY_ORIENTATION_FACE_DOWN,
}

pub struct DisplayOptions<'m>
{
	flags: DisplayFlags,
	refresh_rate: Option<i32>,
	adapter: Option<i32>,
	window_position: Option<[i32, ..2]>,
	options: Option<&'m [(DisplayOption, i32, DisplayOptionImportance)]>
}

impl<'m> DisplayOptions<'m>
{
	pub fn new() -> DisplayOptions
	{
		DisplayOptions{ flags: Zero::zero(), refresh_rate: None, adapter: None, window_position: None, options: None }
	}
}

pub struct Display
{
	priv allegro_display: *mut ALLEGRO_DISPLAY,
	priv backbuffer: Bitmap,
	priv event_source: EventSource
}

impl Display
{
	fn select_this_display(&self)
	{
		unsafe
		{
			if self.allegro_display != al_get_current_display()
			{
				al_set_target_bitmap(al_get_backbuffer(self.allegro_display))
			}
		}
	}

	fn new(w: i32, h: i32) -> Option<Display>
	{
		unsafe
		{
			let d = al_create_display(w as c_int, h as c_int);
			if ptr::is_null(d)
			{
				None
			}
			else
			{
				Some(Display{ allegro_display: d, backbuffer: new_bitmap_ref(al_get_backbuffer(d)),
							  event_source: new_event_source_ref(al_get_display_event_source(d)) })
			}
		}
	}

	fn new_with_options(w: i32, h: i32, opt: &DisplayOptions) -> Option<Display>
	{
		unsafe
		{
			al_set_new_display_flags(opt.flags.get() as c_int);

			match opt.refresh_rate
			{
				Some(r) => al_set_new_display_refresh_rate(r as c_int),
				None => al_set_new_display_refresh_rate(0)
			}

			match opt.adapter
			{
				Some(a) => al_set_new_display_adapter(a as c_int),
				None => al_set_new_display_adapter(ALLEGRO_DEFAULT_DISPLAY_ADAPTER),
			}

			match opt.window_position
			{
				Some([x, y]) =>	al_set_new_window_position(x as c_int, y as c_int),
				None =>	al_set_new_window_position(i32::max_value, i32::max_value)
			}

			al_reset_new_display_options();

			match opt.options
			{
				Some(options) =>
				{
					for &(option, value, importance) in options.iter()
					{
						al_set_new_display_option(option as c_int, value as c_int, importance as c_int);
					}
				},
				None => ()
			}
		}

		Display::new(w, h)
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
			cast::transmute(al_get_display_format(self.allegro_display) as u32)
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
			cast::transmute(al_get_display_flags(self.allegro_display))
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

	pub fn acknowledge_resize(&self) -> bool
	{
		unsafe
		{
			al_acknowledge_resize(self.allegro_display) != 0
		}
	}

	pub fn resize(&self, w: i32, h: i32) -> bool
	{
		unsafe
		{
			al_resize_display(self.allegro_display, w as c_int, h as c_int) != 0
		}
	}

	pub fn set_icon<T: BitmapLike>(&self, icon: &T)
	{
		unsafe
		{
			al_set_display_icon(self.allegro_display, icon.get_bitmap());
		}
	}

	pub fn set_icons<T: BitmapLike, U: Iterator<T>>(&self, icons: U)
	{
		let mut c_icons = icons.map(|ref b| b.get_bitmap()).to_owned_vec();
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
			title.with_ref(|c_str|
			{
				al_set_window_title(self.allegro_display, c_str);
			});
		}
	}

	pub fn get_option(&self, option: DisplayOption) -> i32
	{
		unsafe
		{
			al_get_display_option(self.allegro_display, option as c_int) as i32
		}
	}

	pub fn convert_bitmap<T: BitmapLike>(&self, bmp: &T) -> Option<Bitmap>
	{
		self.select_this_display();
		clone_bitmap(bmp.get_bitmap())
	}

	pub fn get_event_source<'l>(&'l self) -> &'l EventSource
	{
		&'l self.event_source
	}

	pub fn get_allegro_display(&self) -> *mut ALLEGRO_DISPLAY
	{
		self.allegro_display
	}

	pub fn flip(&self)
	{
		self.select_this_display();
		unsafe
		{
			al_flip_display();
		}
	}

	pub fn update_region(&self, x: i32, y: i32, width: i32, height: i32)
	{
		self.select_this_display();
		unsafe
		{
			al_update_display_region(x as c_int, y as c_int, width as c_int, height as c_int);
		}
	}

	pub fn is_compatible_bitmap<T: BitmapLike>(&self, bitmap: &T) -> bool
	{
		self.select_this_display();
		unsafe
		{
			al_is_compatible_bitmap(bitmap.get_bitmap()) != 0
		}
	}

	pub fn wait_for_vsync(&self) -> bool
	{
		self.select_this_display();
		unsafe
		{
			al_wait_for_vsync() != 0
		}
	}

	pub fn hold_bitmap_drawing(&self, hold: bool)
	{
		self.select_this_display();
		unsafe
		{
			al_hold_bitmap_drawing(hold as c_bool);
		}
	}

	pub fn is_bitmap_drawing_held(&self) -> bool
	{
		self.select_this_display();
		unsafe
		{
			al_is_bitmap_drawing_held() != 0
		}
	}
}

impl Drop for Display
{
	fn drop(&mut self)
	{
		unsafe
		{
			al_destroy_display(self.allegro_display);
		}
	}
}

impl CoreDrawing for Display {}

impl DrawTarget for Display
{
	fn get_target_bitmap(&self) -> *mut ALLEGRO_BITMAP
	{
		unsafe
		{
			al_get_backbuffer(self.allegro_display)
		}
	}
}

impl ::internal::core::Core
{
	pub fn create_display(&self, w: i32, h: i32) -> Option<Display>
	{
		Display::new(w, h)
	}

	pub fn create_display_with_options(&self, w: i32, h: i32, opt: &DisplayOptions) -> Option<Display>
	{
		Display::new_with_options(w, h, opt)
	}
}
