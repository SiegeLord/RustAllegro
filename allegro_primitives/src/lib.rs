// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name = "allegro_primitives"]
#![crate_type = "lib"]

extern crate allegro;
extern crate allegro_primitives_sys;
extern crate allegro_sys;
extern crate libc;

use allegro::{BitmapLike, Color, Core};
use allegro_primitives_sys::*;
use allegro_sys::*;
use libc::*;

use std::ptr;

#[repr(u32)]
#[derive(Copy, Clone)]
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
	_dummy: (),
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

impl PrimitivesAddon
{
	pub fn init(_: &Core) -> Result<PrimitivesAddon, String>
	{
		use std::sync::{Once, ONCE_INIT};
		static mut RUN_ONCE: Once = ONCE_INIT;

		let mut res = Err("The primitives addon already initialized.".into());
		unsafe {
			RUN_ONCE.call_once(|| {
				res = if al_init_primitives_addon() != 0
				{
					Ok(PrimitivesAddon { _dummy: () })
				}
				else
				{
					Err("Could not initialize the primitives addon.".into())
				};
			})
		}
		res
	}

	pub fn get_version() -> i32
	{
		unsafe { al_get_allegro_primitives_version() as i32 }
	}

	pub fn draw_prim<T: VertexVector, B: BitmapLike>(&self, vtxs: &T, texture: Option<&B>, start: u32, end: u32, type_: PrimType) -> u32
	{
		check_valid_target_bitmap();
		let tex = texture.map_or(ptr::null_mut(), |bmp| bmp.get_allegro_bitmap());
		unsafe {
			al_draw_prim(
				vtxs.get_ptr() as *const _,
				vtxs.get_decl(),
				tex,
				start as c_int,
				end as c_int,
				type_ as c_int,
			) as u32
		}
	}

	pub fn draw_indexed_prim<T: VertexVector, B: BitmapLike>(
		&self, vtxs: &T, texture: Option<&B>, indices: &[i32], num_vtx: u32, type_: PrimType,
	) -> u32
	{
		check_valid_target_bitmap();
		let tex = texture.map_or(ptr::null_mut(), |bmp| bmp.get_allegro_bitmap());
		unsafe {
			al_draw_indexed_prim(
				vtxs.get_ptr() as *const _,
				vtxs.get_decl(),
				tex,
				indices.as_ptr(),
				num_vtx as c_int,
				type_ as c_int,
			) as u32
		}
	}

	pub fn draw_line(&self, x1: f32, y1: f32, x2: f32, y2: f32, color: Color, thickness: f32)
	{
		check_valid_target_bitmap();
		unsafe {
			al_draw_line(
				x1 as c_float,
				y1 as c_float,
				x2 as c_float,
				y2 as c_float,
				color.get_allegro_color(),
				thickness as c_float,
			)
		}
	}

	pub fn draw_triangle(&self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, color: Color, thickness: f32)
	{
		check_valid_target_bitmap();
		unsafe {
			al_draw_triangle(
				x1 as c_float,
				y1 as c_float,
				x2 as c_float,
				y2 as c_float,
				x3 as c_float,
				y3 as c_float,
				color.get_allegro_color(),
				thickness as c_float,
			);
		}
	}

	pub fn draw_rectangle(&self, x1: f32, y1: f32, x2: f32, y2: f32, color: Color, thickness: f32)
	{
		check_valid_target_bitmap();
		unsafe {
			al_draw_rectangle(
				x1 as c_float,
				y1 as c_float,
				x2 as c_float,
				y2 as c_float,
				color.get_allegro_color(),
				thickness as c_float,
			);
		}
	}

	pub fn draw_rounded_rectangle(&self, x1: f32, y1: f32, x2: f32, y2: f32, rx: f32, ry: f32, color: Color, thickness: f32)
	{
		check_valid_target_bitmap();
		unsafe {
			al_draw_rounded_rectangle(
				x1 as c_float,
				y1 as c_float,
				x2 as c_float,
				y2 as c_float,
				rx as c_float,
				ry as c_float,
				color.get_allegro_color(),
				thickness as c_float,
			);
		}
	}

	pub fn draw_circle(&self, cx: f32, cy: f32, r: f32, color: Color, thickness: f32)
	{
		check_valid_target_bitmap();
		unsafe {
			al_draw_circle(
				cx as c_float,
				cy as c_float,
				r as c_float,
				color.get_allegro_color(),
				thickness as c_float,
			);
		}
	}

	pub fn draw_ellipse(&self, cx: f32, cy: f32, rx: f32, ry: f32, color: Color, thickness: f32)
	{
		check_valid_target_bitmap();
		unsafe {
			al_draw_ellipse(
				cx as c_float,
				cy as c_float,
				rx as c_float,
				ry as c_float,
				color.get_allegro_color(),
				thickness as c_float,
			);
		}
	}

	pub fn draw_arc(&self, cx: f32, cy: f32, r: f32, start_theta: f32, delta_theta: f32, color: Color, thickness: f32)
	{
		check_valid_target_bitmap();
		unsafe {
			al_draw_arc(
				cx as c_float,
				cy as c_float,
				r as c_float,
				start_theta as c_float,
				delta_theta as c_float,
				color.get_allegro_color(),
				thickness as c_float,
			);
		}
	}

	pub fn draw_elliptical_arc(&self, cx: f32, cy: f32, rx: f32, ry: f32, start_theta: f32, delta_theta: f32, color: Color, thickness: f32)
	{
		check_valid_target_bitmap();
		unsafe {
			al_draw_elliptical_arc(
				cx as c_float,
				cy as c_float,
				rx as c_float,
				ry as c_float,
				start_theta as c_float,
				delta_theta as c_float,
				color.get_allegro_color(),
				thickness as c_float,
			);
		}
	}

	pub fn draw_pieslice(&self, cx: f32, cy: f32, r: f32, start_theta: f32, delta_theta: f32, color: Color, thickness: f32)
	{
		check_valid_target_bitmap();
		unsafe {
			al_draw_pieslice(
				cx as c_float,
				cy as c_float,
				r as c_float,
				start_theta as c_float,
				delta_theta as c_float,
				color.get_allegro_color(),
				thickness as c_float,
			);
		}
	}

	pub fn draw_spline<T: Iterator<Item = (f32, f32)>>(&self, points: T, color: Color, thickness: f32) -> Result<(), ()>
	{
		check_valid_target_bitmap();
		let mut c_points: [c_float; 8] = [0.0; 8];
		let mut idx = 0;
		for (x, y) in points
		{
			if idx >= c_points.len()
			{
				return Err(());
			}
			c_points[idx + 0] = x as c_float;
			c_points[idx + 1] = y as c_float;
			idx += 2;
		}

		unsafe {
			al_draw_spline(c_points, color.get_allegro_color(), thickness as c_float);
		}
		Ok(())
	}

	pub fn draw_filled_triangle(&self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, color: Color)
	{
		check_valid_target_bitmap();
		unsafe {
			al_draw_filled_triangle(
				x1 as c_float,
				y1 as c_float,
				x2 as c_float,
				y2 as c_float,
				x3 as c_float,
				y3 as c_float,
				color.get_allegro_color(),
			);
		}
	}

	pub fn draw_filled_rectangle(&self, x1: f32, y1: f32, x2: f32, y2: f32, color: Color)
	{
		check_valid_target_bitmap();
		unsafe {
			al_draw_filled_rectangle(
				x1 as c_float,
				y1 as c_float,
				x2 as c_float,
				y2 as c_float,
				color.get_allegro_color(),
			);
		}
	}

	pub fn draw_filled_ellipse(&self, cx: f32, cy: f32, rx: f32, ry: f32, color: Color)
	{
		check_valid_target_bitmap();
		unsafe {
			al_draw_filled_ellipse(
				cx as c_float,
				cy as c_float,
				rx as c_float,
				ry as c_float,
				color.get_allegro_color(),
			);
		}
	}

	pub fn draw_filled_circle(&self, cx: f32, cy: f32, r: f32, color: Color)
	{
		check_valid_target_bitmap();
		unsafe {
			al_draw_filled_circle(cx as c_float, cy as c_float, r as c_float, color.get_allegro_color());
		}
	}

	pub fn draw_filled_pieslice(&self, cx: f32, cy: f32, r: f32, start_theta: f32, delta_theta: f32, color: Color)
	{
		check_valid_target_bitmap();
		unsafe {
			al_draw_filled_pieslice(
				cx as c_float,
				cy as c_float,
				r as c_float,
				start_theta as c_float,
				delta_theta as c_float,
				color.get_allegro_color(),
			);
		}
	}

	pub fn draw_filled_rounded_rectangle(&self, x1: f32, y1: f32, x2: f32, y2: f32, rx: f32, ry: f32, color: Color)
	{
		check_valid_target_bitmap();
		unsafe {
			al_draw_filled_rounded_rectangle(
				x1 as c_float,
				y1 as c_float,
				x2 as c_float,
				y2 as c_float,
				rx as c_float,
				ry as c_float,
				color.get_allegro_color(),
			);
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
