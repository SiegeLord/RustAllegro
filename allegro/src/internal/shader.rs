// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;
use std::cell::Cell;
use std::ffi::{CStr, CString};
use std::ptr;
use std::rc::Rc;

use internal::bitmap_like::BitmapLike;
use internal::display::{Display, register_shader};

use ffi::*;

/// Shader platform.
#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum ShaderPlatform
{
	/// Automatically detect the platform, only appropriate for shader creation.
	Auto = ALLEGRO_SHADER_AUTO,
	GLSL = ALLEGRO_SHADER_GLSL,
	HLSL = ALLEGRO_SHADER_HLSL,
}

impl ShaderPlatform
{
	pub fn from_allegro(platform: ALLEGRO_SHADER_PLATFORM) -> ShaderPlatform
	{
		match platform
		{
			ALLEGRO_SHADER_AUTO => ShaderPlatform::Auto,
			ALLEGRO_SHADER_GLSL => ShaderPlatform::GLSL,
			ALLEGRO_SHADER_HLSL => ShaderPlatform::HLSL,
			_ => panic!("Malformed platform: {}", platform)
		}
	}
}

/// Shader type.
#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum ShaderType
{
	Pixel = ALLEGRO_PIXEL_SHADER,
	Vertex = ALLEGRO_VERTEX_SHADER,
}

/// A shader program comprising of a pixel (fragment) and a vertex shader.
/// Wraps ALLEGRO_SHADER.
pub struct Shader
{
	allegro_shader: *mut ALLEGRO_SHADER,
	valid: Rc<Cell<bool>>,
}

impl Shader
{
	/// Create a new shader for a particular platform.
	///
	/// A shader gets invalidated when the display is destroyed.
	pub fn new(display: &mut Display, platform: ShaderPlatform) -> Result<Shader, ()>
	{
		let shader;
		unsafe
		{
			let old_target = al_get_target_bitmap();
			al_set_target_bitmap(display.get_backbuffer().get_allegro_bitmap());
			shader = al_create_shader(platform as ALLEGRO_SHADER_PLATFORM);
			al_set_target_bitmap(old_target);
		};
		if !shader.is_null()
		{
			let valid = Rc::new(Cell::new(true));
			register_shader(display, valid.clone(), shader);
			Ok(Shader
			{
				allegro_shader: shader,
				valid: valid,
			})
		}
		else
		{
			Err(())
		}
	}

	/// Returns the wrapped Allegro shader pointer.
	pub fn get_allegro_shader(&self) -> *mut ALLEGRO_SHADER
	{
		if self.is_valid()
		{
			self.allegro_shader
		}
		else
		{
			ptr::null_mut()
		}
	}

	/// Attach a source to the shader. Passing None clears the source.
	pub fn attach_shader_source(&mut self, shader_type: ShaderType, source: Option<&str>) -> Result<(), ()>
	{
		if !self.is_valid()
		{
			return Err(());
		}
		let shader_type = shader_type as ALLEGRO_SHADER_TYPE;
		let ret = unsafe
		{
			match source
			{
				Some(source) =>
				{
					let source = CString::new(source.as_bytes()).unwrap();
					al_attach_shader_source(self.allegro_shader, shader_type, source.as_ptr())
				},
				None => al_attach_shader_source(self.allegro_shader, shader_type, ptr::null())
			}
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

	/// Attach a source to the shader that is loaded from a file.
	pub fn attach_shader_source_file(&mut self, shader_type: ShaderType, filename: &str) -> Result<(), ()>
	{
		if !self.is_valid()
		{
			return Err(())
		}
		let filename = CString::new(filename.as_bytes()).unwrap();
		let ret = unsafe
		{
			al_attach_shader_source_file(self.allegro_shader, shader_type as ALLEGRO_SHADER_TYPE, filename.as_ptr())
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

	/// Build the shader. Call this after attaching the sources.
	pub fn build(&mut self) -> Result<(), ()>
	{
		if !self.is_valid()
		{
			return Err(());
		}
		let ret = unsafe
		{
			al_build_shader(self.allegro_shader)
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

	/// Get the log from the shader. Call this function if any of the attach/build functions fail to determine what went wrong.
	pub fn get_log(&self) -> String
	{
		if !self.is_valid()
		{
			return "".to_string();
		}
		unsafe
		{
			let log = al_get_shader_log(self.allegro_shader);
			CStr::from_ptr(log).to_string_lossy().into_owned()
		}
	}

	/// Returns the platform of this shader.
	pub fn get_platform(&self) -> Result<ShaderPlatform, ()>
	{
		if self.is_valid()
		{
			unsafe
			{
				Ok(ShaderPlatform::from_allegro(al_get_shader_platform(self.allegro_shader)))
			}
		}
		else
		{
			return Err(())
		}
	}

	/// Sets a sampler for a particular uniform and unit. Different uniforms should be set to different units.
	/// Pass None to bmp to clear the sampler.
	pub fn set_sampler<T: BitmapLike>(&mut self, name: &str, bmp: &T, unit: i32) -> Result<(), ()>
	{
		if !self.is_valid()
		{
			return Err(())
		}
		let c_name = CString::new(name.as_bytes()).unwrap();
		let ret = unsafe
		{
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

	pub fn is_valid(&self) -> bool
	{
		self.valid.get()
	}
}

impl Drop for Shader
{
	fn drop(&mut self)
	{
		if self.is_valid()
		{
			unsafe
			{
				al_destroy_shader(self.allegro_shader);
			}
		}
	}
}
