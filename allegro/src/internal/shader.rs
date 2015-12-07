// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use std::ffi::{CStr, CString};
use std::ptr;

use internal::core::Core;

use ffi::*;

/// Shader platform.
#[cfg(allegro_5_1_0)]
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
#[cfg(allegro_5_1_0)]
#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum ShaderType
{
	Pixel = ALLEGRO_PIXEL_SHADER,
	Vertex = ALLEGRO_VERTEX_SHADER,
}

/// A shader program comprising of a pixel (fragment) and a vertex shader.
/// Wraps ALLEGRO_SHADER.
#[cfg(allegro_5_1_0)]
pub struct Shader
{
	allegro_shader: *mut ALLEGRO_SHADER,
}

#[cfg(allegro_5_1_0)]
impl Shader
{
	/// Create a new shader for a particular platform.
	pub fn new(_: &Core, platform: ShaderPlatform) -> Result<Shader, ()>
	{
		let shader = unsafe
		{
			al_create_shader(platform as ALLEGRO_SHADER_PLATFORM)
		};
		if !shader.is_null()
		{
			Ok(Shader
			{
				allegro_shader: shader,
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
		self.allegro_shader
	}

	/// Attach a source to the shader. Passing None clears the source.
	pub fn attach_shader_source(&mut self, shader_type: ShaderType, source: Option<&str>) -> Result<(), ()>
	{
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
		unsafe
		{
			let log = al_get_shader_log(self.allegro_shader);
			CStr::from_ptr(log).to_string_lossy().into_owned()
		}
	}

	/// Returns the platform of this shader.
	pub fn get_platform(&self) -> ShaderPlatform
	{
		unsafe
		{
			ShaderPlatform::from_allegro(al_get_shader_platform(self.allegro_shader))
		}
	}
}

impl Drop for Shader
{
	fn drop(&mut self)
	{
		unsafe
		{
			al_destroy_shader(self.allegro_shader);
		}
	}
}
