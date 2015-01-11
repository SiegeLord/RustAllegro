// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name="allegro_primitives"]

#![crate_type = "lib"]
#![allow(unstable)]
#![feature(thread_local)]

extern crate allegro;
extern crate libc;
extern crate "allegro_primitives-sys" as allegro_primitives_sys;

use std::marker::NoSend;
use std::ptr;

use std::sync::{Arc, Mutex};

use allegro::{Bitmap, BitmapLike, Core, Color};
use allegro_primitives_sys::*;
use libc::*;

static mut initialized: bool = false;
#[thread_local]
static mut spawned_on_this_thread: bool = false;

#[repr(u32)]
#[derive(Copy)]
pub enum PrimType
{
	LineList = ALLEGRO_PRIM_LINE_LIST,
	LineStrip = ALLEGRO_PRIM_LINE_STRIP,
	LineLoop = ALLEGRO_PRIM_LINE_LOOP,
	TriangleList = ALLEGRO_PRIM_TRIANGLE_LIST,
	TriangleStrip = ALLEGRO_PRIM_TRIANGLE_STRIP,
	TriangleFan = ALLEGRO_PRIM_TRIANGLE_FAN,
	PointList = ALLEGRO_PRIM_POINT_LIST,
}

pub struct PrimitivesAddon
{
	no_send_marker: NoSend,
	core_mutex: Arc<Mutex<()>>,
}

impl PrimitivesAddon
{
	pub fn init(core: &Core) -> Result<PrimitivesAddon, String>
	{
		let mutex = core.get_core_mutex();
		let _guard = mutex.lock();
		unsafe
		{
			if initialized
			{
				if spawned_on_this_thread
				{
					Err("The primitives addon has already been created in this task.".to_string())
				}
				else
				{
					spawned_on_this_thread = true;
					Ok(PrimitivesAddon{ no_send_marker: NoSend, core_mutex: core.get_core_mutex() })
				}
			}
			else
			{
				if al_init_primitives_addon() != 0
				{
					initialized = true;
					spawned_on_this_thread = true;
					Ok(PrimitivesAddon{ no_send_marker: NoSend, core_mutex: core.get_core_mutex() })
				}
				else
				{
					Err("Could not initialize the primitives addon.".to_string())
				}
			}
		}
	}

	pub fn get_version() -> i32
	{
		unsafe
		{
			al_get_allegro_primitives_version() as i32
		}
	}

	pub fn get_core_mutex(&self) -> Arc<Mutex<()>>
	{
		self.core_mutex.clone()
	}

	pub fn draw_prim<T: VertexVector, B: BitmapLike = Bitmap>(&self, vtxs: &T, texture: Option<&B>, start: u32, end: u32, type_: PrimType) -> u32
	{
		let tex = texture.map_or(ptr::null_mut(), |bmp| bmp.get_allegro_bitmap());
		unsafe
		{
			al_draw_prim(vtxs.get_ptr() as *const _, vtxs.get_decl(), tex, start as c_int, end as c_int, type_ as c_int) as u32
		}
	}

	pub fn draw_indexed_prim<T: VertexVector, B: BitmapLike = Bitmap>(&self, vtxs: &T, texture: Option<&B>, indices: &[i32], num_vtx: u32, type_: PrimType) -> u32
	{
		let tex = texture.map_or(ptr::null_mut(), |bmp| bmp.get_allegro_bitmap());
		unsafe
		{
			al_draw_indexed_prim(vtxs.get_ptr() as *const _, vtxs.get_decl(), tex, indices.as_ptr(), num_vtx as c_int, type_ as c_int) as u32
		}
	}

	pub fn draw_line(&self, x1: f32, y1: f32, x2: f32, y2: f32, color: Color, thickness: f32)
	{
		unsafe
		{
			al_draw_line(x1 as c_float, y1 as c_float, x2 as c_float, y2 as c_float, color.0, thickness as c_float)
		}
	}

	pub fn draw_triangle(&self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, color: Color, thickness: f32)
	{
		unsafe
		{
			al_draw_triangle(x1 as c_float, y1 as c_float, x2 as c_float, y2 as c_float, x3 as c_float, y3 as c_float, color.0, thickness as c_float);
		}
	}

	pub fn draw_rectangle(&self, x1: f32, y1: f32, x2: f32, y2: f32, color: Color, thickness: f32)
	{
		unsafe
		{
			al_draw_rectangle(x1 as c_float, y1 as c_float, x2 as c_float, y2 as c_float, color.0, thickness as c_float);
		}
	}

	pub fn draw_rounded_rectangle(&self, x1: f32, y1: f32, x2: f32, y2: f32, rx: f32, ry: f32, color: Color, thickness: f32)
	{
		unsafe
		{
			al_draw_rounded_rectangle(x1 as c_float, y1 as c_float, x2 as c_float, y2 as c_float, rx as c_float, ry as c_float, color.0, thickness as c_float);
		}
	}

	pub fn draw_circle(&self, cx: f32, cy: f32, r: f32, color: Color, thickness: f32)
	{
		unsafe
		{
			al_draw_circle(cx as c_float, cy as c_float, r as c_float, color.0, thickness as c_float);
		}
	}

	pub fn draw_ellipse(&self, cx: f32, cy: f32, rx: f32, ry: f32, color: Color, thickness: f32)
	{
		unsafe
		{
			al_draw_ellipse(cx as c_float, cy as c_float, rx as c_float, ry as c_float, color.0, thickness as c_float);
		}
	}

	pub fn draw_arc(&self, cx: f32, cy: f32, r: f32, start_theta: f32, delta_theta: f32, color: Color, thickness: f32)
	{
		unsafe
		{
			al_draw_arc(cx as c_float, cy as c_float, r as c_float, start_theta as c_float, delta_theta as c_float, color.0, thickness as c_float);
		}
	}

	pub fn draw_elliptical_arc(&self, cx: f32, cy: f32, rx: f32, ry: f32, start_theta: f32, delta_theta: f32, color: Color, thickness: f32)
	{
		unsafe
		{
			al_draw_elliptical_arc(cx as c_float, cy as c_float, rx as c_float, ry as c_float, start_theta as c_float, delta_theta as c_float, color.0, thickness as c_float);
		}
	}

	pub fn draw_pieslice(&self, cx: f32, cy: f32, r: f32, start_theta: f32, delta_theta: f32, color: Color, thickness: f32)
	{
		unsafe
		{
			al_draw_pieslice(cx as c_float, cy as c_float, r as c_float, start_theta as c_float, delta_theta as c_float, color.0, thickness as c_float);
		}
	}

	pub fn draw_spline<T: Iterator<Item = (f32, f32)>>(&self, mut points: T, color: Color, thickness: f32) -> Result<(), ()>
	{
		let mut c_points: [c_float; 8] = [0.0; 8];
		let mut idx = 0;
		for (x, y) in points
		{
			if idx >= c_points.len()
			{
				return Err(())
			}
			c_points[idx + 0] = x as c_float;
			c_points[idx + 1] = y as c_float;
			idx += 2;
		}

		unsafe
		{
			al_draw_spline(c_points, color.0, thickness as c_float);
		}
		Ok(())
	}

	pub fn draw_filled_triangle(&self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, color: Color)
	{
		unsafe
		{
			al_draw_filled_triangle(x1 as c_float, y1 as c_float, x2 as c_float, y2 as c_float, x3 as c_float, y3 as c_float, color.0);
		}
	}

	pub fn draw_filled_rectangle(&self, x1: f32, y1: f32, x2: f32, y2: f32, color: Color)
	{
		unsafe
		{
			al_draw_filled_rectangle(x1 as c_float, y1 as c_float, x2 as c_float, y2 as c_float, color.0);
		}
	}

	pub fn draw_filled_ellipse(&self, cx: f32, cy: f32, rx: f32, ry: f32, color: Color)
	{
		unsafe
		{
			al_draw_filled_ellipse(cx as c_float, cy as c_float, rx as c_float, ry as c_float, color.0);
		}
	}

	pub fn draw_filled_circle(&self, cx: f32, cy: f32, r: f32, color: Color)
	{
		unsafe
		{
			al_draw_filled_circle(cx as c_float, cy as c_float, r as c_float, color.0);
		}
	}

	pub fn draw_filled_pieslice(&self, cx: f32, cy: f32, r: f32, start_theta: f32, delta_theta: f32, color: Color)
	{
		unsafe
		{
			al_draw_filled_pieslice(cx as c_float, cy as c_float, r as c_float, start_theta as c_float, delta_theta as c_float, color.0);
		}
	}

	pub fn draw_filled_rounded_rectangle(&self, x1: f32, y1: f32, x2: f32, y2: f32, rx: f32, ry: f32, color: Color)
	{
		unsafe
		{
			al_draw_filled_rounded_rectangle(x1 as c_float, y1 as c_float, x2 as c_float, y2 as c_float, rx as c_float, ry as c_float, color.0);
		}
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vertex
{
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub u: f32,
	pub v: f32,
	pub color: Color,
}

pub trait VertexVector
{
	fn get_ptr(&self) -> *const u8;
	fn get_decl(&self) -> *const ALLEGRO_VERTEX_DECL;
}

impl<'l> VertexVector for &'l [Vertex]
{
	fn get_ptr(&self) -> *const u8
	{
		self.as_ptr() as *const _
	}

	fn get_decl(&self) -> *const ALLEGRO_VERTEX_DECL
	{
		ptr::null()
	}
}
