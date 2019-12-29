// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;

use bitmap::*;
use transformations::*;
use allegro_util::c_bool;

opaque!(ALLEGRO_SHADER);

pub type ALLEGRO_SHADER_TYPE = c_uint;
pub const ALLEGRO_VERTEX_SHADER: c_uint = 1;
pub const ALLEGRO_PIXEL_SHADER: c_uint = 2;

pub type ALLEGRO_SHADER_PLATFORM = c_uint;
pub const ALLEGRO_SHADER_AUTO: c_uint = 0;
pub const ALLEGRO_SHADER_GLSL: c_uint = 1;
pub const ALLEGRO_SHADER_HLSL: c_uint = 2;

extern
{
	pub fn al_create_shader(platform: ALLEGRO_SHADER_PLATFORM) -> *mut ALLEGRO_SHADER;
	pub fn al_attach_shader_source(shader: *mut ALLEGRO_SHADER,
	                               _type: ALLEGRO_SHADER_TYPE,
	                               source: *const c_char)
	                               -> c_bool;
	pub fn al_attach_shader_source_file(shader: *mut ALLEGRO_SHADER,
	                                    _type: ALLEGRO_SHADER_TYPE,
	                                    filename: *const c_char)
	                                    -> c_bool;
	pub fn al_build_shader(shader: *mut ALLEGRO_SHADER) -> c_bool;
	pub fn al_get_shader_log(shader: *mut ALLEGRO_SHADER) -> *const c_char;
	pub fn al_get_shader_platform(shader: *mut ALLEGRO_SHADER) -> ALLEGRO_SHADER_PLATFORM;
	pub fn al_use_shader(shader: *mut ALLEGRO_SHADER) -> c_bool;
	pub fn al_destroy_shader(shader: *mut ALLEGRO_SHADER) -> ();
	pub fn al_set_shader_sampler(name: *const c_char,
	                             bitmap: *mut ALLEGRO_BITMAP,
	                             unit: c_int)
	                             -> c_bool;
	pub fn al_set_shader_matrix(name: *const c_char, matrix: *const ALLEGRO_TRANSFORM) -> c_bool;
	pub fn al_set_shader_int(name: *const c_char, i: c_int) -> c_bool;
	pub fn al_set_shader_float(name: *const c_char, f: c_float) -> c_bool;
	pub fn al_set_shader_int_vector(name: *const c_char,
	                                num_components: c_int,
	                                i: *mut c_int,
	                                num_elems: c_int)
	                                -> c_bool;
	pub fn al_set_shader_float_vector(name: *const c_char,
	                                  num_components: c_int,
	                                  f: *mut c_float,
	                                  num_elems: c_int)
	                                  -> c_bool;
	pub fn al_set_shader_bool(name: *const c_char, b: c_bool) -> c_bool;
	pub fn al_get_default_shader_source(platform: ALLEGRO_SHADER_PLATFORM,
	                                    _type: ALLEGRO_SHADER_TYPE)
	                                    -> *const c_char;
}
