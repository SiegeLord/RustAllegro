// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name="allegro_primitives_sys"]
#![crate_type = "lib"]

extern crate allegro_sys;
#[macro_use]
extern crate allegro_util;
extern crate libc;

pub use allegro_primitives::*;

pub mod allegro_primitives
{
	#![allow(non_camel_case_types)]

	use libc::*;
	use allegro_util::c_bool;
	use allegro_sys::{ALLEGRO_BITMAP, ALLEGRO_COLOR};

	pub const ALLEGRO_PRIM_LINE_LIST: c_uint = 0;
	pub const ALLEGRO_PRIM_LINE_STRIP: c_uint = 1;
	pub const ALLEGRO_PRIM_LINE_LOOP: c_uint = 2;
	pub const ALLEGRO_PRIM_TRIANGLE_LIST: c_uint = 3;
	pub const ALLEGRO_PRIM_TRIANGLE_STRIP: c_uint = 4;
	pub const ALLEGRO_PRIM_TRIANGLE_FAN: c_uint = 5;
	pub const ALLEGRO_PRIM_POINT_LIST: c_uint = 6;
	pub const ALLEGRO_PRIM_NUM_TYPES: c_uint = 7;

	pub const ALLEGRO_PRIM_POSITION: c_uint = 1;
	pub const ALLEGRO_PRIM_COLOR_ATTR: c_uint = 2;
	pub const ALLEGRO_PRIM_TEX_COORD: c_uint = 3;
	pub const ALLEGRO_PRIM_TEX_COORD_PIXEL: c_uint = 4;
	pub const ALLEGRO_PRIM_ATTR_NUM: c_uint = 5;

	pub const ALLEGRO_PRIM_FLOAT_2: c_uint = 0;
	pub const ALLEGRO_PRIM_FLOAT_3: c_uint = 1;
	pub const ALLEGRO_PRIM_SHORT_2: c_uint = 2;

	#[repr(C)]
	#[derive(Copy, Clone)]
	pub struct ALLEGRO_VERTEX_ELEMENT
	{
		pub attribute: c_int,
		pub storage: c_int,
		pub offset: c_int,
	}

	pub type Struct_ALLEGRO_VERTEX_DECL = c_void;

	opaque!(ALLEGRO_VERTEX_DECL);

	#[repr(C)]
	#[derive(Copy, Clone)]
	pub struct ALLEGRO_VERTEX
	{
		pub x: c_float,
		pub y: c_float,
		pub z: c_float,
		pub u: c_float,
		pub v: c_float,
		pub color: ALLEGRO_COLOR,
	}

	extern "C"
	{
		pub fn al_get_allegro_primitives_version() -> uint32_t;
		pub fn al_init_primitives_addon() -> c_bool;
		pub fn al_shutdown_primitives_addon();

		pub fn al_draw_prim(vtxs: *const c_void, decl: *const ALLEGRO_VERTEX_DECL, texture: *mut ALLEGRO_BITMAP, start: c_int, end: c_int, _type: c_int) -> c_int;
		pub fn al_draw_indexed_prim(vtxs: *const c_void, decl: *const ALLEGRO_VERTEX_DECL, texture: *mut ALLEGRO_BITMAP, indices: *const c_int, num_vtx: c_int, _type: c_int) -> c_int;

		pub fn al_create_vertex_decl(elements: *const ALLEGRO_VERTEX_ELEMENT, stride: c_int) -> *mut ALLEGRO_VERTEX_DECL;
		pub fn al_destroy_vertex_decl(decl: *mut ALLEGRO_VERTEX_DECL);

		pub fn al_draw_soft_triangle(v1: *mut ALLEGRO_VERTEX, v2: *mut ALLEGRO_VERTEX, v3: *mut ALLEGRO_VERTEX, state: uintptr_t, init: ::std::option::Option<extern "C" fn (arg1: uintptr_t, arg2: *mut ALLEGRO_VERTEX, arg3: *mut ALLEGRO_VERTEX, arg4: *mut ALLEGRO_VERTEX)>, first: ::std::option::Option<extern "C" fn (arg1: uintptr_t, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int)>, step: ::std::option::Option<extern "C" fn (arg1: uintptr_t, arg2: c_int)>, draw: ::std::option::Option<extern "C" fn (arg1: uintptr_t, arg2: c_int, arg3: c_int, arg4: c_int)>);
		pub fn al_draw_soft_line(v1: *mut ALLEGRO_VERTEX, v2: *mut ALLEGRO_VERTEX, state: uintptr_t, first: ::std::option::Option<extern "C" fn (arg1: uintptr_t, arg2: c_int, arg3: c_int, arg4: *mut ALLEGRO_VERTEX, arg5: *mut ALLEGRO_VERTEX)>, step: ::std::option::Option<extern "C" fn (arg1: uintptr_t, arg2: c_int)>, draw: ::std::option::Option<extern "C" fn (arg1: uintptr_t, arg2: c_int, arg3: c_int)>);

		pub fn al_draw_line(x1: c_float, y1: c_float, x2: c_float, y2: c_float, color: ALLEGRO_COLOR, thickness: c_float);
		pub fn al_draw_triangle(x1: c_float, y1: c_float, x2: c_float, y2: c_float, x3: c_float, y3: c_float, color: ALLEGRO_COLOR, thickness: c_float);
		pub fn al_draw_rectangle(x1: c_float, y1: c_float, x2: c_float, y2: c_float, color: ALLEGRO_COLOR, thickness: c_float);
		pub fn al_draw_rounded_rectangle(x1: c_float, y1: c_float, x2: c_float, y2: c_float, rx: c_float, ry: c_float, color: ALLEGRO_COLOR, thickness: c_float);
		pub fn al_calculate_arc(dest: *mut c_float, stride: c_int, cx: c_float, cy: c_float, rx: c_float, ry: c_float, start_theta: c_float, delta_theta: c_float, thickness: c_float, num_segments: c_int);
		pub fn al_draw_circle(cx: c_float, cy: c_float, r: c_float, color: ALLEGRO_COLOR, thickness: c_float);
		pub fn al_draw_ellipse(cx: c_float, cy: c_float, rx: c_float, ry: c_float, color: ALLEGRO_COLOR, thickness: c_float);
		pub fn al_draw_arc(cx: c_float, cy: c_float, r: c_float, start_theta: c_float, delta_theta: c_float, color: ALLEGRO_COLOR, thickness: c_float);
		pub fn al_draw_elliptical_arc(cx: c_float, cy: c_float, rx: c_float, ry: c_float, start_theta: c_float, delta_theta: c_float, color: ALLEGRO_COLOR, thickness: c_float);
		pub fn al_draw_pieslice(cx: c_float, cy: c_float, r: c_float, start_theta: c_float, delta_theta: c_float, color: ALLEGRO_COLOR, thickness: c_float);
		pub fn al_calculate_spline(dest: *mut c_float, stride: c_int, points: [c_float; 8], thickness: c_float, num_segments: c_int);
		pub fn al_draw_spline(points: [c_float; 8], color: ALLEGRO_COLOR, thickness: c_float);
		pub fn al_calculate_ribbon(dest: *mut c_float, dest_stride: c_int, points: *const c_float, points_stride: c_int, thickness: c_float, num_segments: c_int);
		pub fn al_draw_ribbon(points: *const c_float, points_stride: c_int, color: ALLEGRO_COLOR, thickness: c_float, num_segments: c_int);
		pub fn al_draw_filled_triangle(x1: c_float, y1: c_float, x2: c_float, y2: c_float, x3: c_float, y3: c_float, color: ALLEGRO_COLOR);
		pub fn al_draw_filled_rectangle(x1: c_float, y1: c_float, x2: c_float, y2: c_float, color: ALLEGRO_COLOR);
		pub fn al_draw_filled_ellipse(cx: c_float, cy: c_float, rx: c_float, ry: c_float, color: ALLEGRO_COLOR);
		pub fn al_draw_filled_circle(cx: c_float, cy: c_float, r: c_float, color: ALLEGRO_COLOR);
		pub fn al_draw_filled_pieslice(cx: c_float, cy: c_float, r: c_float, start_theta: c_float, delta_theta: c_float, color: ALLEGRO_COLOR);
		pub fn al_draw_filled_rounded_rectangle(x1: c_float, y1: c_float, x2: c_float, y2: c_float, rx: c_float, ry: c_float, color: ALLEGRO_COLOR);
	}
}
