// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use allegro_util::Flag;
use bitmap::Bitmap;
use bitmap_like::BitmapLike;
use color::PixelFormat;
use core::{check_display_targeted_elsewhere, update_thread_state, Core};
use events::EventSource;

use ffi::*;
use libc::*;
#[cfg(any(allegro_5_2_0, allegro_5_1_0))]
use shader::{Shader, ShaderPlatform};
use std::ffi::CString;
use std::mem;
use std::rc::{Rc, Weak};

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

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
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
#[derive(Copy, Clone, Debug)]
pub enum DisplayOptionImportance
{
	DontCare = ALLEGRO_DONTCARE,
	Require = ALLEGRO_REQUIRE,
	Suggest = ALLEGRO_SUGGEST,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
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
	#[cfg(any(allegro_5_2_0, allegro_5_1_0))]
	shaders: Vec<Rc<Shader>>,
}

impl Display
{
	pub fn new(_: &Core, w: i32, h: i32) -> Result<Display, ()>
	{
		unsafe {
			let d = al_create_display(w as c_int, h as c_int);
			if d.is_null()
			{
				Err(())
			}
			else
			{
				update_thread_state();
				let backbuffer = Bitmap::wrap(al_get_backbuffer(d), false);
				Ok(Display::new_impl(d, backbuffer))
			}
		}
	}

	#[cfg(not(any(allegro_5_2_0, allegro_5_1_0)))]
	fn new_impl(d: *mut ALLEGRO_DISPLAY, backbuffer: Bitmap) -> Display
	{
		Display {
			allegro_display: d,
			backbuffer: backbuffer,
		}
	}

	#[cfg(any(allegro_5_2_0, allegro_5_1_0))]
	fn new_impl(d: *mut ALLEGRO_DISPLAY, backbuffer: Bitmap) -> Display
	{
		Display {
			allegro_display: d,
			backbuffer: backbuffer,
			shaders: vec![],
		}
	}

	pub fn get_width(&self) -> i32
	{
		unsafe { al_get_display_width(self.allegro_display) as i32 }
	}

	pub fn get_height(&self) -> i32
	{
		unsafe { al_get_display_height(self.allegro_display) as i32 }
	}

	pub fn get_format(&self) -> PixelFormat
	{
		unsafe { mem::transmute(al_get_display_format(self.allegro_display) as u32) }
	}

	pub fn get_refresh_rate(&self) -> i32
	{
		unsafe { al_get_display_refresh_rate(self.allegro_display) as i32 }
	}

	pub fn get_flags(&self) -> DisplayFlags
	{
		unsafe { mem::transmute(al_get_display_flags(self.allegro_display)) }
	}

	pub fn set_flag(&self, flag: DisplayFlags, onoff: bool) -> bool
	{
		unsafe { al_set_display_flag(self.allegro_display, flag.get(), onoff as u8) != 0 }
	}

	pub fn get_backbuffer(&self) -> &Bitmap
	{
		&self.backbuffer
	}

	pub fn acknowledge_resize(&self) -> Result<(), ()>
	{
		unsafe {
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
		unsafe {
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
		unsafe {
			al_set_display_icon(self.allegro_display, icon.get_allegro_bitmap());
		}
	}

	pub fn set_icons<'l, U: Iterator<Item = &'l (BitmapLike + 'l)>>(&self, icons: U)
	{
		let mut c_icons: Vec<_> = icons.map(|b| b.get_allegro_bitmap()).collect();
		unsafe {
			al_set_display_icons(self.allegro_display, c_icons.len() as c_int, c_icons.as_mut_ptr());
		}
	}

	pub fn set_window_position(&self, x: i32, y: i32)
	{
		unsafe {
			al_set_window_position(self.allegro_display, x as c_int, y as c_int);
		}
	}

	pub fn get_window_position(&self) -> (i32, i32)
	{
		unsafe {
			let mut x = 0 as c_int;
			let mut y = 0 as c_int;
			al_get_window_position(self.allegro_display, &mut x, &mut y);
			(x as i32, y as i32)
		}
	}

	pub fn set_window_title(&self, title: &str)
	{
		let title = CString::new(title.as_bytes()).unwrap();
		unsafe {
			al_set_window_title(self.allegro_display, title.as_ptr());
		}
	}

	pub fn get_option(&self, option: DisplayOption) -> i32
	{
		unsafe { al_get_display_option(self.allegro_display, option as c_int) as i32 }
	}

	pub fn convert_bitmap<T: BitmapLike>(&self, bmp: &T) -> Result<Bitmap, ()>
	{
		unsafe { Bitmap::clone_and_wrap(bmp.get_allegro_bitmap()) }
	}

	pub fn get_event_source(&self) -> EventSource
	{
		unsafe { EventSource::wrap(al_get_display_event_source(self.allegro_display)) }
	}

	pub fn get_allegro_display(&self) -> *mut ALLEGRO_DISPLAY
	{
		self.allegro_display
	}

	/// Create a new shader associated with this display.
	///
	/// Note that display destruction will panic if a strong reference is held
	/// to a shader at that time.
	#[cfg(any(allegro_5_2_0, allegro_5_1_0))]
	pub fn create_shader(&mut self, platform: ShaderPlatform) -> Result<Weak<Shader>, ()>
	{
		let shader;
		unsafe {
			let old_target = al_get_target_bitmap();
			al_set_target_bitmap(self.get_backbuffer().get_allegro_bitmap());
			shader = al_create_shader(platform as ALLEGRO_SHADER_PLATFORM);
			al_set_target_bitmap(old_target);
		};
		if !shader.is_null()
		{
			let shader = unsafe { Rc::new(Shader::wrap(shader)) };
			let ret = Rc::downgrade(&shader);
			self.shaders.push(shader);
			Ok(ret)
		}
		else
		{
			Err(())
		}
	}

	#[cfg(any(allegro_5_2_0, allegro_5_1_0))]
	fn has_outstanding_shaders(&self) -> bool
	{
		for shader in &self.shaders
		{
			if Rc::strong_count(&shader) != 1
			{
				return true;
			}
		}
		false
	}

	#[cfg(not(any(allegro_5_2_0, allegro_5_1_0)))]
	fn has_outstanding_shaders(&self) -> bool
	{
		false
	}
}

impl Drop for Display
{
	fn drop(&mut self)
	{
		if self.has_outstanding_shaders()
		{
			panic!("Display has outstanding shaders!");
		}
		unsafe {
			check_display_targeted_elsewhere(self.allegro_display, "destroy");
			al_destroy_display(self.allegro_display);
			update_thread_state();
		}
	}
}
