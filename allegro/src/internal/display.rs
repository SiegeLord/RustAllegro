// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;
use std::cell::Cell;
use std::mem;
use std::ffi::CString;
use std::rc::Rc;

use internal::bitmap::{Bitmap, SubBitmap, SharedBitmap, new_bitmap_ref, clone_bitmap};
use internal::bitmap_like::BitmapLike;
use internal::color::PixelFormat;
use internal::core::{Core, dummy_target};
use internal::events::{EventSource, new_event_source_ref};

use ffi::*;

pub mod external
{
	pub use super::{DisplayOption, DisplayOptionImportance, DisplayOrientation, Display};
	pub use super::export_helper::*;
}

pub use self::export_helper::*;

mod export_helper
{
	use allegro_util::Flag;
	use ffi::*;

	flag_type!
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
	#[cfg(any(allegro_5_2_0, allegro_5_1_6))]
	flags!
	{
		DisplayFlags
		{
			PROGRAMMABLE_PIPELINE = ALLEGRO_PROGRAMMABLE_PIPELINE
		}
	}
	#[cfg(any(allegro_5_2_0, allegro_5_1_12))]
	flags!
	{
		DisplayFlags
		{
			MAXIMIZED = ALLEGRO_MAXIMIZED
		}
	}
}

#[repr(u32)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
pub enum DisplayOptionImportance
{
	DontCare = ALLEGRO_DONTCARE,
	Require = ALLEGRO_REQUIRE,
	Suggest = ALLEGRO_SUGGEST,
}

#[repr(u32)]
#[derive(Copy, Clone)]
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
	_backbuffer: Rc<Bitmap>,
	backbuffer_subbitmap: SubBitmap,
	event_source: EventSource,
	#[cfg(any(allegro_5_2_0, allegro_5_1_0))]
	shaders: Vec<(Rc<Cell<bool>>, *mut ALLEGRO_SHADER)>
}

impl Display
{
	pub fn new(_: &Core, w: i32, h: i32) -> Result<Display, ()>
	{
		unsafe
		{
			let d = al_create_display(w as c_int, h as c_int);
			if d.is_null()
			{
				Err(())
			}
			else
			{
				let backbuffer = Rc::new(new_bitmap_ref(al_get_backbuffer(d)));
				let backbuffer_subbitmap = try!(backbuffer.create_sub_bitmap(0, 0, backbuffer.get_width(), backbuffer.get_height()));
				Ok
				(
					Display::new_impl(d, backbuffer, backbuffer_subbitmap, new_event_source_ref(al_get_display_event_source(d)))
				)
			}
		}
	}

	#[cfg(not(any(allegro_5_2_0, allegro_5_1_0)))]
	fn new_impl(d: *mut ALLEGRO_DISPLAY, backbuffer: Rc<Bitmap>, backbuffer_subbitmap: SubBitmap, event_source: EventSource) -> Display
	{
		Display
		{
			allegro_display: d,
			_backbuffer: backbuffer,
			backbuffer_subbitmap: backbuffer_subbitmap,
			event_source: event_source,
		}
	}

	#[cfg(any(allegro_5_2_0, allegro_5_1_0))]
	fn new_impl(d: *mut ALLEGRO_DISPLAY, backbuffer: Rc<Bitmap>, backbuffer_subbitmap: SubBitmap, event_source: EventSource) -> Display
	{
		Display
		{
			allegro_display: d,
			_backbuffer: backbuffer,
			backbuffer_subbitmap: backbuffer_subbitmap,
			event_source: event_source,
			shaders: vec![]
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

	pub fn get_backbuffer(&self) -> &SubBitmap
	{
		&self.backbuffer_subbitmap
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

	pub fn set_icons<'l, U: Iterator<Item = &'l(BitmapLike + 'l)>>(&self, icons: U)
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

	pub fn set_window_title(&self, title: &str)
	{
		let title = CString::new(title.as_bytes()).unwrap();
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

	#[cfg(any(allegro_5_2_0, allegro_5_1_0))]
	fn destroy_shaders(&mut self)
	{
		for &mut (ref mut valid, shader) in &mut self.shaders
		{
			if valid.get()
			{
				unsafe
				{
					al_destroy_shader(shader);
				}
				valid.set(false);
			}
		}
	}

	#[cfg(not(any(allegro_5_2_0, allegro_5_1_0)))]
	fn destroy_shaders(&mut self)
	{
	}
}

impl Drop for Display
{
	fn drop(&mut self)
	{
		self.destroy_shaders();
		unsafe
		{
			al_destroy_display(self.allegro_display);
			if al_get_target_bitmap().is_null()
			{
				al_set_target_bitmap(dummy_target);
			}
		}
	}
}

#[cfg(any(allegro_5_2_0, allegro_5_1_0))]
pub fn register_shader(d: &mut Display, valid: Rc<Cell<bool>>, shader: *mut ALLEGRO_SHADER)
{
	d.shaders.push((valid, shader));
}
