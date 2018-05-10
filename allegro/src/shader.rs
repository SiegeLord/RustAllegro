// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use ffi::*;

use libc::*;
use std::ffi::{CStr, CString};
use std::ptr;

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
			_ => panic!("Malformed platform: {}", platform),
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
}

impl Shader
{
	pub unsafe fn wrap(shader: *mut ALLEGRO_SHADER) -> Shader
	{
		Shader { allegro_shader: shader }
	}

	/// Return the wrapped Allegro shader pointer.
	pub fn get_allegro_shader(&self) -> *mut ALLEGRO_SHADER
	{
		self.allegro_shader
	}

	/// Attach a source to the shader. Passing None clears the source.
	///
	/// Returns the log if there was an error.
	pub fn attach_shader_source(&self, shader_type: ShaderType, source: Option<&str>) -> Result<(), String>
	{
		let shader_type = shader_type as ALLEGRO_SHADER_TYPE;
		let ret = unsafe {
			match source
			{
				Some(source) =>
				{
					let source = CString::new(source.as_bytes()).unwrap();
					al_attach_shader_source(self.allegro_shader, shader_type, source.as_ptr())
				}
				None => al_attach_shader_source(self.allegro_shader, shader_type, ptr::null()),
			}
		};
		if ret != 0
		{
			Ok(())
		}
		else
		{
			Err(self.get_log())
		}
	}

	/// Attach a source to the shader that is loaded from a file.
	///
	/// Returns the log if there was an error.
	pub fn attach_shader_source_file(&self, shader_type: ShaderType, filename: &str) -> Result<(), String>
	{
		let filename = CString::new(filename.as_bytes()).unwrap();
		let ret = unsafe { al_attach_shader_source_file(self.allegro_shader, shader_type as ALLEGRO_SHADER_TYPE, filename.as_ptr()) };
		if ret != 0
		{
			Ok(())
		}
		else
		{
			Err(self.get_log())
		}
	}

	/// Build the shader. Call this after attaching the sources.
	///
	/// Returns the log if there was an error.
	pub fn build(&self) -> Result<(), String>
	{
		let ret = unsafe { al_build_shader(self.allegro_shader) };
		if ret != 0
		{
			Ok(())
		}
		else
		{
			Err(self.get_log())
		}
	}

	/// Get the log from the shader. Call this function if any of the attach/build functions fail to determine what went wrong.
	pub fn get_log(&self) -> String
	{
		unsafe {
			let log = al_get_shader_log(self.allegro_shader);
			CStr::from_ptr(log).to_string_lossy().into_owned()
		}
	}

	/// Return the platform of this shader.
	pub fn get_platform(&self) -> ShaderPlatform
	{
		unsafe { ShaderPlatform::from_allegro(al_get_shader_platform(self.allegro_shader)) }
	}
}

impl Drop for Shader
{
	fn drop(&mut self)
	{
		unsafe {
			al_destroy_shader(self.allegro_shader);
		}
	}
}

/// Trait implemented by types that can be used to set uniforms in shaders.
pub trait ShaderUniform
{
	unsafe fn set_self_for_shader(&self, name: &str) -> Result<(), ()>;
}

macro_rules! impl_shader_vector {
	($rust_type:ty, $func:expr, $num_elems:expr, $c_type:ty) => {
		impl ShaderUniform for $rust_type
		{
			unsafe fn set_self_for_shader(&self, name: &str) -> Result<(), ()>
			{
				let c_name = CString::new(name.as_bytes()).unwrap();
				//~ println!("{} {} {} {}", name, $num_elems, self.len(), stringify!($func));
				let ret = $func(c_name.as_ptr(), $num_elems, self.as_ptr() as *mut $c_type, self.len() as i32);
				if ret != 0
				{
					Ok(())
				}
				else
				{
					Err(())
				}
			}
		}
	};
}

impl_shader_vector!([f32], al_set_shader_float_vector, 1, c_float);
impl_shader_vector!([[f32; 2]], al_set_shader_float_vector, 2, c_float);
impl_shader_vector!([[f32; 3]], al_set_shader_float_vector, 3, c_float);
impl_shader_vector!([[f32; 4]], al_set_shader_float_vector, 4, c_float);
impl_shader_vector!([i32], al_set_shader_int_vector, 1, c_int);
impl_shader_vector!([[i32; 2]], al_set_shader_int_vector, 1, c_int);
impl_shader_vector!([[i32; 3]], al_set_shader_int_vector, 3, c_int);
impl_shader_vector!([[i32; 4]], al_set_shader_int_vector, 4, c_int);
