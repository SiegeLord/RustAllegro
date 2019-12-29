// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use allegro_util::{c_bool, from_c_str, Flag};
use bitmap_like::{BitmapFlags, BitmapLike};
use color::{Color, PixelFormat};
use config::Config;
use display::{Display, DisplayFlags, DisplayOption, DisplayOptionImportance};
use std::sync::Mutex;

use events::EventSource;

use ffi::*;
use keycodes::{KeyCode, KeyModifier};
use libc::*;
#[cfg(any(allegro_5_2_0, allegro_5_1_0))]
use shader::{Shader, ShaderPlatform, ShaderType, ShaderUniform};
use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;
use std::thread;
use transformations::Transform;

struct ThreadState
{
	id: thread::ThreadId,
	cur_bitmap: *mut ALLEGRO_BITMAP,
	cur_display: *mut ALLEGRO_DISPLAY,
}

unsafe impl Send for ThreadState {}

lazy_static! {
	static ref THREAD_STATE: Mutex<Vec<ThreadState>> = Mutex::new(vec![]);
}

unsafe fn get_real_bitmap(bmp: *mut ALLEGRO_BITMAP) -> *mut ALLEGRO_BITMAP
{
	if bmp.is_null() || al_is_sub_bitmap(bmp) == 0
	{
		bmp
	}
	else
	{
		al_get_parent_bitmap(bmp)
	}
}

pub(crate) unsafe fn update_thread_state()
{
	let cur_id = thread::current().id();
	let mut thread_state = THREAD_STATE.lock().unwrap();
	let pos = (*thread_state)
		.iter()
		.position(|s| s.id == cur_id)
		.unwrap_or(thread_state.len());
	if pos >= thread_state.len()
	{
		thread_state.push(ThreadState {
			id: cur_id,
			cur_bitmap: ptr::null_mut(),
			cur_display: ptr::null_mut(),
		});
	}
	thread_state[pos].cur_bitmap = get_real_bitmap(al_get_target_bitmap());
	thread_state[pos].cur_display = al_get_current_display();
}

pub(crate) unsafe fn check_bitmap_targeted_elsewhere(bmp: *mut ALLEGRO_BITMAP, action: &str)
{
	if bmp.is_null()
	{
		return;
	}
	let cur_id = thread::current().id();
	let thread_state = THREAD_STATE.lock().unwrap();
	let bmp = get_real_bitmap(bmp);
	if let Some(s) = thread_state
		.iter()
		.find(|s| s.cur_bitmap == bmp && s.id != cur_id)
	{
		panic!(
			"Attempting to {} bitmap {:?} in thread {:?}, but it is still targeted in thread {:?}",
			action, s.cur_bitmap, cur_id, s.id
		);
	}
}

pub(crate) unsafe fn check_display_targeted_elsewhere(disp: *mut ALLEGRO_DISPLAY, action: &str)
{
	if disp.is_null()
	{
		return;
	}
	let cur_id = thread::current().id();
	let thread_state = THREAD_STATE.lock().unwrap();
	if let Some(s) = thread_state
		.iter()
		.find(|s| s.cur_display == disp && s.id != cur_id)
	{
		panic!(
			"Attempting to {} display {:?} in thread {:?}, but it is still targeted in thread {:?}",
			action, s.cur_display, cur_id, s.id
		);
	}
}

fn check_valid_target_bitmap()
{
	unsafe {
		if al_get_target_bitmap().is_null()
		{
			panic!("Target bitmap is null!");
		}
	}
}

flag_type! {
	BitmapDrawingFlags
	{
		FLIP_NONE = 0x1,
		FLIP_HORIZONTAL = ALLEGRO_FLIP_HORIZONTAL << 1,
		FLIP_VERTICAL = ALLEGRO_FLIP_VERTICAL << 1
	}
}

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum BlendMode
{
	Zero = ALLEGRO_ZERO,
	One = ALLEGRO_ONE,
	Alpha = ALLEGRO_ALPHA,
	InverseAlpha = ALLEGRO_INVERSE_ALPHA,
	SrcColor = ALLEGRO_SRC_COLOR,
	DestColor = ALLEGRO_DEST_COLOR,
	InverseSrcColor = ALLEGRO_INVERSE_SRC_COLOR,
	InverseDestColor = ALLEGRO_INVERSE_DEST_COLOR,
	ConstColor = ALLEGRO_CONST_COLOR,
	InverseConstColor = ALLEGRO_INVERSE_CONST_COLOR,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum BlendOperation
{
	Add = ALLEGRO_ADD,
	SrcMinusDest = ALLEGRO_SRC_MINUS_DEST,
	DestMinusSrc = ALLEGRO_DEST_MINUS_SRC,
}

/// Extents of a monitor.
#[derive(Copy, Clone, Debug)]
pub struct MonitorInfo
{
	pub x1: i32,
	pub y1: i32,
	pub x2: i32,
	pub y2: i32,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum DepthFunction
{
	Never = ALLEGRO_RENDER_NEVER as u32,
	Always = ALLEGRO_RENDER_ALWAYS as u32,
	Less = ALLEGRO_RENDER_LESS as u32,
	Equal = ALLEGRO_RENDER_EQUAL as u32,
	LessEqual = ALLEGRO_RENDER_LESS_EQUAL as u32,
	Greater = ALLEGRO_RENDER_GREATER as u32,
	NotEqual = ALLEGRO_RENDER_NOT_EQUAL as u32,
	GreaterEqual = ALLEGRO_RENDER_GREATER_EQUAL as u32,
}

/// Type through which you'll be doing most your interaction with Allegro.
pub struct Core
{
	system_config: Config,
}

impl Core
{
	/// This must be called on the main thread.
	pub fn init() -> Result<Core, String>
	{
		Core::init_with_config(&Config::new())
	}

	/// Initializes Allegro while merging in the system_config into the system configuration.
	pub fn init_with_config(system_config: &Config) -> Result<Core, String>
	{
		use std::sync::Once;
		static mut RUN_ONCE: Once = Once::new();

		let mut res = Err("Already initialized.".to_string());
		unsafe {
			RUN_ONCE.call_once(|| {
				res = if al_install_system(ALLEGRO_VERSION_INT as c_int, None) != 0
				{
					al_merge_config_into(al_get_system_config(), system_config.get_allegro_config() as *const _);
					let config = Config::wrap(al_get_system_config(), false);
					Ok(Core { system_config: config })
				}
				else
				{
					let version = al_get_allegro_version();
					let major = version >> 24;
					let minor = (version >> 16) & 255;
					let revision = (version >> 8) & 255;
					let release = version & 255;

					Err(format!(
						"The system Allegro version ({}.{}.{}.{}) does not match the version of this binding ({}.{}.{}.{})",
						major, minor, revision, release, ALLEGRO_VERSION, ALLEGRO_SUB_VERSION, ALLEGRO_WIP_VERSION, ALLEGRO_RELEASE_NUMBER
					))
				};
			});
		}
		res
	}

	/// Returns the system config.
	pub fn get_system_config(&self) -> &Config
	{
		&self.system_config
	}

	/// Returns the system config.
	pub fn get_system_config_mut(&mut self) -> &mut Config
	{
		&mut self.system_config
	}

	pub fn get_num_video_adapters(&self) -> i32
	{
		unsafe { al_get_num_video_adapters() as i32 }
	}

	pub fn get_monitor_info(&self, adapter: i32) -> Result<MonitorInfo, ()>
	{
		unsafe {
			let mut c_info = ALLEGRO_MONITOR_INFO {
				x1: 0,
				y1: 0,
				x2: 0,
				y2: 0,
			};
			if al_get_monitor_info(adapter as c_int, &mut c_info as *mut _) != 0
			{
				Ok(MonitorInfo {
					x1: c_info.x1 as i32,
					y1: c_info.y1 as i32,
					x2: c_info.x2 as i32,
					y2: c_info.y2 as i32,
				})
			}
			else
			{
				Err(())
			}
		}
	}

	pub fn rest(&self, seconds: f64)
	{
		unsafe {
			al_rest(seconds as c_double);
		}
	}

	pub fn get_time(&self) -> f64
	{
		unsafe { al_get_time() as f64 }
	}

	pub fn install_keyboard(&self) -> Result<(), ()>
	{
		unsafe {
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
		unsafe { al_is_keyboard_installed() != 0 }
	}

	pub fn get_keyboard_event_source(&self) -> Option<EventSource>
	{
		if self.is_keyboard_installed()
		{
			unsafe { Some(EventSource::wrap(al_get_keyboard_event_source())) }
		}
		else
		{
			None
		}
	}

	pub fn set_keyboard_leds(&self, leds: KeyModifier) -> Result<(), ()>
	{
		assert!(self.is_keyboard_installed());
		unsafe {
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
		unsafe { from_c_str(al_keycode_to_name(k as c_int)) }
	}

	pub fn install_mouse(&self) -> Result<(), ()>
	{
		unsafe {
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
		unsafe { al_is_mouse_installed() != 0 }
	}

	pub fn get_mouse_event_source(&self) -> Option<EventSource>
	{
		if self.is_mouse_installed()
		{
			unsafe { Some(EventSource::wrap(al_get_mouse_event_source())) }
		}
		else
		{
			None
		}
	}

	pub fn install_joystick(&self) -> Result<(), ()>
	{
		unsafe {
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
		unsafe { al_is_joystick_installed() != 0 }
	}

	pub fn get_joystick_event_source(&self) -> Option<EventSource>
	{
		if self.is_joystick_installed()
		{
			unsafe { Some(EventSource::wrap(al_get_joystick_event_source())) }
		}
		else
		{
			None
		}
	}

	pub fn reconfigure_joysticks(&self) -> Result<(), ()>
	{
		assert!(self.is_joystick_installed());
		unsafe {
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
		unsafe { al_get_num_joysticks() as i32 }
	}

	pub fn get_mouse_num_buttons(&self) -> u32
	{
		assert!(self.is_mouse_installed());
		unsafe { al_get_mouse_num_buttons() as u32 }
	}

	pub fn get_mouse_num_axes(&self) -> u32
	{
		assert!(self.is_mouse_installed());
		unsafe { al_get_mouse_num_axes() as u32 }
	}

	pub fn set_mouse_xy(&self, display: &Display, x: i32, y: i32) -> Result<(), ()>
	{
		assert!(self.is_mouse_installed());
		unsafe {
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
		assert!(self.is_mouse_installed());
		unsafe {
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
		assert!(self.is_mouse_installed());
		unsafe {
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
		assert!(self.is_mouse_installed());
		unsafe {
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
		assert!(self.is_mouse_installed());
		unsafe {
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
		assert!(self.is_mouse_installed());
		unsafe {
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
		unsafe {
			al_set_new_bitmap_flags(flags.get() as c_int);
		}
	}

	pub fn get_new_bitmap_flags(&self) -> BitmapFlags
	{
		unsafe { mem::transmute(al_get_new_bitmap_flags() as u32) }
	}

	pub fn set_new_bitmap_format(&self, format: PixelFormat)
	{
		unsafe {
			al_set_new_bitmap_format(format as c_int);
		}
	}

	pub fn get_new_bitmap_format(&self) -> PixelFormat
	{
		unsafe { mem::transmute(al_get_new_bitmap_format() as u32) }
	}

	pub fn set_target_bitmap<T: BitmapLike>(&self, bmp: Option<&T>)
	{
		unsafe {
			al_set_target_bitmap(
				bmp.map(|b| b.get_allegro_bitmap())
					.unwrap_or(ptr::null_mut()),
			);
			check_bitmap_targeted_elsewhere(al_get_target_bitmap(), "target");
			check_display_targeted_elsewhere(al_get_current_display(), "target");
			update_thread_state();
		}
	}

	pub fn clear_to_color(&self, color: Color)
	{
		check_valid_target_bitmap();
		unsafe {
			al_clear_to_color(color.get_allegro_color());
		}
	}

	#[cfg(any(allegro_5_2_0, allegro_5_1_2))]
	pub fn clear_depth_buffer(&self, z: f32)
	{
		check_valid_target_bitmap();
		unsafe {
			al_clear_depth_buffer(z);
		}
	}

	pub fn draw_pixel(&self, x: f32, y: f32, color: Color)
	{
		check_valid_target_bitmap();
		unsafe {
			al_draw_pixel(x as c_float, y as c_float, color.get_allegro_color());
		}
	}

	pub fn put_pixel(&self, x: i32, y: i32, color: Color)
	{
		check_valid_target_bitmap();
		unsafe {
			al_put_pixel(x as c_int, y as c_int, color.get_allegro_color());
		}
	}

	pub fn put_blended_pixel(&self, x: i32, y: i32, color: Color)
	{
		check_valid_target_bitmap();
		unsafe {
			al_put_blended_pixel(x as c_int, y as c_int, color.get_allegro_color());
		}
	}

	pub fn draw_bitmap<T: BitmapLike>(
		&self, bitmap: &T, dx: f32, dy: f32, flags: BitmapDrawingFlags,
	)
	{
		check_valid_target_bitmap();
		unsafe {
			al_draw_bitmap(
				bitmap.get_allegro_bitmap(),
				dx as c_float,
				dy as c_float,
				(flags.get() >> 1) as c_int,
			);
		}
	}

	pub fn draw_bitmap_region<T: BitmapLike>(
		&self, bitmap: &T, sx: f32, sy: f32, sw: f32, sh: f32, dx: f32, dy: f32,
		flags: BitmapDrawingFlags,
	)
	{
		check_valid_target_bitmap();
		unsafe {
			al_draw_bitmap_region(
				bitmap.get_allegro_bitmap(),
				sx as c_float,
				sy as c_float,
				sw as c_float,
				sh as c_float,
				dx as c_float,
				dy as c_float,
				(flags.get() >> 1) as c_int,
			);
		}
	}

	pub fn draw_scaled_bitmap<T: BitmapLike>(
		&self, bitmap: &T, sx: f32, sy: f32, sw: f32, sh: f32, dx: f32, dy: f32, dw: f32, dh: f32,
		flags: BitmapDrawingFlags,
	)
	{
		check_valid_target_bitmap();
		unsafe {
			al_draw_scaled_bitmap(
				bitmap.get_allegro_bitmap(),
				sx as c_float,
				sy as c_float,
				sw as c_float,
				sh as c_float,
				dx as c_float,
				dy as c_float,
				dw as c_float,
				dh as c_float,
				(flags.get() >> 1) as c_int,
			);
		}
	}

	pub fn draw_rotated_bitmap<T: BitmapLike>(
		&self, bitmap: &T, cx: f32, cy: f32, dx: f32, dy: f32, angle: f32,
		flags: BitmapDrawingFlags,
	)
	{
		check_valid_target_bitmap();
		unsafe {
			al_draw_rotated_bitmap(
				bitmap.get_allegro_bitmap(),
				cx as c_float,
				cy as c_float,
				dx as c_float,
				dy as c_float,
				angle as c_float,
				(flags.get() >> 1) as c_int,
			);
		}
	}

	pub fn draw_scaled_rotated_bitmap<T: BitmapLike>(
		&self, bitmap: &T, cx: f32, cy: f32, dx: f32, dy: f32, xscale: f32, yscale: f32,
		angle: f32, flags: BitmapDrawingFlags,
	)
	{
		check_valid_target_bitmap();
		unsafe {
			al_draw_scaled_rotated_bitmap(
				bitmap.get_allegro_bitmap(),
				cx as c_float,
				cy as c_float,
				dx as c_float,
				dy as c_float,
				xscale as c_float,
				yscale as c_float,
				angle as c_float,
				(flags.get() >> 1) as c_int,
			);
		}
	}

	pub fn draw_tinted_bitmap<T: BitmapLike>(
		&self, bitmap: &T, tint: Color, dx: f32, dy: f32, flags: BitmapDrawingFlags,
	)
	{
		check_valid_target_bitmap();
		unsafe {
			al_draw_tinted_bitmap(
				bitmap.get_allegro_bitmap(),
				tint.get_allegro_color(),
				dx as c_float,
				dy as c_float,
				(flags.get() >> 1) as c_int,
			);
		}
	}

	pub fn draw_tinted_bitmap_region<T: BitmapLike>(
		&self, bitmap: &T, tint: Color, sx: f32, sy: f32, sw: f32, sh: f32, dx: f32, dy: f32,
		flags: BitmapDrawingFlags,
	)
	{
		check_valid_target_bitmap();
		unsafe {
			al_draw_tinted_bitmap_region(
				bitmap.get_allegro_bitmap(),
				tint.get_allegro_color(),
				sx as c_float,
				sy as c_float,
				sw as c_float,
				sh as c_float,
				dx as c_float,
				dy as c_float,
				(flags.get() >> 1) as c_int,
			);
		}
	}

	pub fn draw_tinted_scaled_bitmap<T: BitmapLike>(
		&self, bitmap: &T, tint: Color, sx: f32, sy: f32, sw: f32, sh: f32, dx: f32, dy: f32,
		dw: f32, dh: f32, flags: BitmapDrawingFlags,
	)
	{
		check_valid_target_bitmap();
		unsafe {
			al_draw_tinted_scaled_bitmap(
				bitmap.get_allegro_bitmap(),
				tint.get_allegro_color(),
				sx as c_float,
				sy as c_float,
				sw as c_float,
				sh as c_float,
				dx as c_float,
				dy as c_float,
				dw as c_float,
				dh as c_float,
				(flags.get() >> 1) as c_int,
			);
		}
	}

	pub fn draw_tinted_rotated_bitmap<T: BitmapLike>(
		&self, bitmap: &T, tint: Color, cx: f32, cy: f32, dx: f32, dy: f32, angle: f32,
		flags: BitmapDrawingFlags,
	)
	{
		check_valid_target_bitmap();
		unsafe {
			al_draw_tinted_rotated_bitmap(
				bitmap.get_allegro_bitmap(),
				tint.get_allegro_color(),
				cx as c_float,
				cy as c_float,
				dx as c_float,
				dy as c_float,
				angle as c_float,
				(flags.get() >> 1) as c_int,
			);
		}
	}

	pub fn draw_tinted_scaled_rotated_bitmap<T: BitmapLike>(
		&self, bitmap: &T, tint: Color, cx: f32, cy: f32, dx: f32, dy: f32, xscale: f32,
		yscale: f32, angle: f32, flags: BitmapDrawingFlags,
	)
	{
		check_valid_target_bitmap();
		unsafe {
			al_draw_tinted_scaled_rotated_bitmap(
				bitmap.get_allegro_bitmap(),
				tint.get_allegro_color(),
				cx as c_float,
				cy as c_float,
				dx as c_float,
				dy as c_float,
				xscale as c_float,
				yscale as c_float,
				angle as c_float,
				(flags.get() >> 1) as c_int,
			);
		}
	}

	pub fn draw_tinted_scaled_rotated_bitmap_region<T: BitmapLike>(
		&self, bitmap: &T, sx: f32, sy: f32, sw: f32, sh: f32, tint: Color, cx: f32, cy: f32,
		dx: f32, dy: f32, xscale: f32, yscale: f32, angle: f32, flags: BitmapDrawingFlags,
	)
	{
		check_valid_target_bitmap();
		unsafe {
			al_draw_tinted_scaled_rotated_bitmap_region(
				bitmap.get_allegro_bitmap(),
				sx as c_float,
				sy as c_float,
				sw as c_float,
				sh as c_float,
				tint.get_allegro_color(),
				cx as c_float,
				cy as c_float,
				dx as c_float,
				dy as c_float,
				xscale as c_float,
				yscale as c_float,
				angle as c_float,
				(flags.get() >> 1) as c_int,
			);
		}
	}

	pub fn set_clipping_rectangle(&self, x: i32, y: i32, width: i32, height: i32)
	{
		check_valid_target_bitmap();
		unsafe {
			al_set_clipping_rectangle(x as c_int, y as c_int, width as c_int, height as c_int);
		}
	}

	pub fn reset_clipping_rectangle(&self)
	{
		check_valid_target_bitmap();
		unsafe {
			al_reset_clipping_rectangle();
		}
	}

	pub fn get_clipping_rectangle(&self) -> (i32, i32, i32, i32)
	{
		check_valid_target_bitmap();
		unsafe {
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
		unsafe {
			al_set_new_display_flags(flags.get() as c_int);
		}
	}

	pub fn get_new_display_flags(&self) -> DisplayFlags
	{
		unsafe { mem::transmute(al_get_new_display_flags()) }
	}

	pub fn set_new_display_refresh_rate(&self, rate: i32)
	{
		unsafe {
			al_set_new_display_refresh_rate(rate as c_int);
		}
	}

	pub fn get_new_display_refresh_rate(&self) -> i32
	{
		unsafe { al_get_new_display_refresh_rate() as i32 }
	}

	pub fn set_new_display_adapter(&self, adapter: i32)
	{
		unsafe {
			al_set_new_display_adapter(adapter as c_int);
		}
	}

	pub fn get_new_display_adapter(&self) -> i32
	{
		unsafe { al_get_new_display_adapter() as i32 }
	}

	pub fn set_new_window_position(&self, x: i32, y: i32)
	{
		unsafe {
			al_set_new_window_position(x as c_int, y as c_int);
		}
	}

	pub fn get_new_window_position(&self) -> (i32, i32)
	{
		unsafe {
			use std::mem::MaybeUninit;

			let mut x = MaybeUninit::uninit();
			let mut y = MaybeUninit::uninit();
			al_get_new_window_position(x.as_mut_ptr(), y.as_mut_ptr());
			(x.assume_init() as i32, y.assume_init() as i32)
		}
	}

	pub fn reset_new_display_options(&self)
	{
		unsafe {
			al_reset_new_display_options();
		}
	}

	pub fn set_new_display_option(
		&self, option: DisplayOption, value: i32, importance: DisplayOptionImportance,
	)
	{
		unsafe {
			al_set_new_display_option(option as c_int, value as c_int, importance as c_int);
		}
	}

	pub fn get_new_display_option(&self, option: DisplayOption) -> (i32, DisplayOptionImportance)
	{
		unsafe {
			use std::mem::MaybeUninit;

			let mut imp = MaybeUninit::uninit();

			let val = al_get_new_display_option(option as c_int, imp.as_mut_ptr());
			(val as i32, mem::transmute(imp.assume_init()))
		}
	}

	pub fn get_current_transform(&self) -> Transform
	{
		check_valid_target_bitmap();
		let t = unsafe { al_get_current_transform() };
		unsafe { Transform::wrap(*t) }
	}

	pub fn use_transform(&self, trans: &Transform)
	{
		check_valid_target_bitmap();
		unsafe {
			al_use_transform(&trans.get_allegro_transform());
		}
	}

	#[cfg(any(allegro_5_2_0, allegro_5_1_9))]
	pub fn use_projection_transform(&self, trans: &Transform)
	{
		check_valid_target_bitmap();
		unsafe {
			al_use_projection_transform(&trans.get_allegro_transform());
		}
	}

	/// Set the shader as current for the current bitmap. Pass None to stop using this shader.
	/// Returns an error if the shader isn't compatible with the bitmap.
	#[cfg(any(allegro_5_2_0, allegro_5_1_6))]
	pub fn use_shader(&self, shader: Option<&Shader>) -> Result<(), ()>
	{
		check_valid_target_bitmap();
		match shader
		{
			Some(shader) =>
			{
				let ret = unsafe { al_use_shader(shader.get_allegro_shader()) };
				if ret != 0
				{
					Ok(())
				}
				else
				{
					Err(())
				}
			}
			None =>
			{
				unsafe {
					al_use_shader(ptr::null_mut());
				}
				Ok(())
			}
		}
	}

	/// Returns the source of the shader that Allegro uses by default.
	#[cfg(any(allegro_5_2_0, allegro_5_1_6))]
	pub fn get_default_shader_source(
		&self, platform: ShaderPlatform, shader_type: ShaderType,
	) -> Option<String>
	{
		unsafe {
			let src = al_get_default_shader_source(
				platform as ALLEGRO_SHADER_PLATFORM,
				shader_type as ALLEGRO_SHADER_TYPE,
			);
			if src.is_null()
			{
				None
			}
			else
			{
				Some(CStr::from_ptr(src).to_string_lossy().into_owned())
			}
		}
	}

	pub fn flip_display(&self)
	{
		unsafe {
			al_flip_display();
		}
	}

	pub fn update_display_region(&self, x: i32, y: i32, width: i32, height: i32)
	{
		unsafe {
			al_update_display_region(x as c_int, y as c_int, width as c_int, height as c_int);
		}
	}

	pub fn wait_for_vsync(&self) -> Result<(), ()>
	{
		unsafe {
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
		unsafe {
			al_hold_bitmap_drawing(hold as c_bool);
		}
	}

	pub fn is_bitmap_drawing_held(&self) -> bool
	{
		unsafe { al_is_bitmap_drawing_held() != 0 }
	}

	/// Set a sampler for a particular uniform and unit for the current shader.
	/// Different uniforms should be set to different units.
	/// Pass None to bmp to clear the sampler.
	#[cfg(any(allegro_5_2_0, allegro_5_1_0))]
	pub fn set_shader_sampler<T: BitmapLike>(
		&mut self, name: &str, bmp: &T, unit: i32,
	) -> Result<(), ()>
	{
		let c_name = CString::new(name.as_bytes()).unwrap();
		let ret = unsafe {
			al_set_shader_sampler(c_name.as_ptr(), bmp.get_allegro_bitmap(), unit as c_int) != 0
		};
		if ret
		{
			Ok(())
		}
		else
		{
			Err(())
		}
	}

	/// Sets a shader uniform to a value.
	#[cfg(any(allegro_5_2_0, allegro_5_1_0))]
	pub fn set_shader_uniform<T: ShaderUniform + ?Sized>(
		&self, name: &str, val: &T,
	) -> Result<(), ()>
	{
		unsafe { val.set_self_for_shader(name) }
	}

	/// Sets a shader uniform to a value.
	#[cfg(any(allegro_5_2_0, allegro_5_1_0))]
	pub fn set_shader_transform(&self, name: &str, val: &Transform) -> Result<(), ()>
	{
		let ret = unsafe {
			let name = CString::new(name.as_bytes()).unwrap();
			al_set_shader_matrix(name.as_ptr(), &val.get_allegro_transform())
		};
		if ret != 0
		{
			Ok(())
		}
		else
		{
			Err(())
		}
	}

	/// Set blender options.
	pub fn set_blender(&self, op: BlendOperation, source: BlendMode, dest: BlendMode)
	{
		unsafe {
			al_set_blender(op as c_int, source as c_int, dest as c_int);
		}
	}

	/// Set blender options, with alpha channel getting a separate setting.
	pub fn set_separate_blender(
		&self, op: BlendOperation, source: BlendMode, dest: BlendMode, alpha_op: BlendOperation,
		alpha_source: BlendMode, alpha_dest: BlendMode,
	)
	{
		unsafe {
			al_set_separate_blender(
				op as c_int,
				source as c_int,
				dest as c_int,
				alpha_op as c_int,
				alpha_source as c_int,
				alpha_dest as c_int,
			);
		}
	}

	pub fn set_depth_test(&self, function: Option<DepthFunction>)
	{
		if let Some(function) = function
		{
			unsafe {
				al_set_render_state(ALLEGRO_DEPTH_TEST, 1);
				al_set_render_state(ALLEGRO_DEPTH_FUNCTION, function as c_int);
			}
		}
		else
		{
			unsafe {
				al_set_render_state(ALLEGRO_DEPTH_TEST, 0);
			}
		}
	}
}
