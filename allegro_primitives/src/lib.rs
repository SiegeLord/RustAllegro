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
		use std::sync::Once;
		static mut RUN_ONCE: Once = Once::new();

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

	pub fn draw_prim<T: VertexSource + ?Sized, B: BitmapLike>(
		&self, vtxs: &T, texture: Option<&B>, start: u32, end: u32, type_: PrimType,
	) -> u32
	{
		check_valid_target_bitmap();
		let tex = texture.map_or(ptr::null_mut(), |bmp| bmp.get_allegro_bitmap());
		unsafe {
			al_draw_prim(
				vtxs.get_ptr() as *const _,
				T::VertexType::get_decl(self).get_allegro_decl(),
				tex,
				start as c_int,
				end as c_int,
				type_ as c_int,
			) as u32
		}
	}

	pub fn draw_indexed_prim<T: VertexSource + ?Sized, B: BitmapLike>(
		&self, vtxs: &T, texture: Option<&B>, indices: &[i32], start: u32, end: u32,
		type_: PrimType,
	) -> u32
	{
		check_valid_target_bitmap();
		let tex = texture.map_or(ptr::null_mut(), |bmp| bmp.get_allegro_bitmap());
		unsafe {
			al_draw_indexed_prim(
				vtxs.get_ptr() as *const _,
				T::VertexType::get_decl(self).get_allegro_decl(),
				tex,
				indices[start as usize..].as_ptr(),
				(end - start) as c_int,
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

	pub fn draw_triangle(
		&self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, color: Color, thickness: f32,
	)
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

	pub fn draw_rounded_rectangle(
		&self, x1: f32, y1: f32, x2: f32, y2: f32, rx: f32, ry: f32, color: Color, thickness: f32,
	)
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

	pub fn draw_arc(
		&self, cx: f32, cy: f32, r: f32, start_theta: f32, delta_theta: f32, color: Color,
		thickness: f32,
	)
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

	pub fn draw_elliptical_arc(
		&self, cx: f32, cy: f32, rx: f32, ry: f32, start_theta: f32, delta_theta: f32,
		color: Color, thickness: f32,
	)
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

	pub fn draw_pieslice(
		&self, cx: f32, cy: f32, r: f32, start_theta: f32, delta_theta: f32, color: Color,
		thickness: f32,
	)
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

	pub fn draw_spline<T: Iterator<Item = (f32, f32)>>(
		&self, points: T, color: Color, thickness: f32,
	) -> Result<(), ()>
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
			al_draw_spline(&c_points, color.get_allegro_color(), thickness as c_float);
		}
		Ok(())
	}

	pub fn draw_filled_triangle(
		&self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, color: Color,
	)
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
			al_draw_filled_circle(
				cx as c_float,
				cy as c_float,
				r as c_float,
				color.get_allegro_color(),
			);
		}
	}

	pub fn draw_filled_pieslice(
		&self, cx: f32, cy: f32, r: f32, start_theta: f32, delta_theta: f32, color: Color,
	)
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

	pub fn draw_filled_rounded_rectangle(
		&self, x1: f32, y1: f32, x2: f32, y2: f32, rx: f32, ry: f32, color: Color,
	)
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
#[derive(Copy, Clone, Debug)]
pub struct Vertex
{
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub u: f32,
	pub v: f32,
	pub color: Color,
}

pub struct VertexDeclBuilder
{
	pos: Option<ALLEGRO_VERTEX_ELEMENT>,
	color: Option<ALLEGRO_VERTEX_ELEMENT>,
	uv: Option<ALLEGRO_VERTEX_ELEMENT>,
	uv_pixel: Option<ALLEGRO_VERTEX_ELEMENT>,
	user_attrs: Vec<ALLEGRO_VERTEX_ELEMENT>,
	stride: usize,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VertexAttrStorage
{
	F32_1 = ALLEGRO_PRIM_FLOAT_1,
	F32_2 = ALLEGRO_PRIM_FLOAT_2,
	F32_3 = ALLEGRO_PRIM_FLOAT_3,
	F32_4 = ALLEGRO_PRIM_FLOAT_4,
	I16_2 = ALLEGRO_PRIM_SHORT_2,
	U8_4 = ALLEGRO_PRIM_UBYTE_4,
	U16_4 = ALLEGRO_PRIM_SHORT_4,
	NormalizedU8_4 = ALLEGRO_PRIM_NORMALIZED_UBYTE_4,
	NormalizedI16_2 = ALLEGRO_PRIM_NORMALIZED_SHORT_2,
	NormalizedI16_4 = ALLEGRO_PRIM_NORMALIZED_SHORT_4,
	NormalizedU16_2 = ALLEGRO_PRIM_NORMALIZED_USHORT_2,
	NormalizedU16_4 = ALLEGRO_PRIM_NORMALIZED_USHORT_4,
	HalfF32_2 = ALLEGRO_PRIM_HALF_FLOAT_2,
	HalfF32_4 = ALLEGRO_PRIM_HALF_FLOAT_4,
}

impl VertexDeclBuilder
{
	pub fn new(stride: usize) -> Self
	{
		VertexDeclBuilder {
			pos: None,
			color: None,
			uv: None,
			uv_pixel: None,
			user_attrs: vec![],
			stride: stride,
		}
	}

	pub fn pos(mut self, storage: VertexAttrStorage, offset: usize) -> Result<Self, ()>
	{
		if storage != VertexAttrStorage::F32_2
			&& storage != VertexAttrStorage::F32_3
			&& storage != VertexAttrStorage::I16_2
		{
			return Err(());
		}
		self.pos = Some(ALLEGRO_VERTEX_ELEMENT {
			attribute: ALLEGRO_PRIM_POSITION,
			storage: storage as c_int,
			offset: offset as c_int,
		});
		Ok(self)
	}

	pub fn color(mut self, offset: usize) -> Result<Self, ()>
	{
		self.color = Some(ALLEGRO_VERTEX_ELEMENT {
			attribute: ALLEGRO_PRIM_COLOR_ATTR,
			storage: 0,
			offset: offset as c_int,
		});
		Ok(self)
	}

	pub fn uv(mut self, storage: VertexAttrStorage, offset: usize) -> Result<Self, ()>
	{
		if storage != VertexAttrStorage::F32_2 && storage != VertexAttrStorage::I16_2
		{
			return Err(());
		}
		self.uv = Some(ALLEGRO_VERTEX_ELEMENT {
			attribute: ALLEGRO_PRIM_TEX_COORD,
			storage: storage as c_int,
			offset: offset as c_int,
		});
		Ok(self)
	}

	pub fn uv_pixel(mut self, storage: VertexAttrStorage, offset: usize) -> Result<Self, ()>
	{
		if storage != VertexAttrStorage::F32_2 && storage != VertexAttrStorage::I16_2
		{
			return Err(());
		}
		self.uv_pixel = Some(ALLEGRO_VERTEX_ELEMENT {
			attribute: ALLEGRO_PRIM_TEX_COORD_PIXEL,
			storage: storage as c_int,
			offset: offset as c_int,
		});
		Ok(self)
	}

	pub fn user_attr(mut self, storage: VertexAttrStorage, offset: usize)
		-> Result<Self, ()>
	{
		if self.user_attrs.len() == ALLEGRO_PRIM_MAX_USER_ATTR as usize
		{
			return Err(());
		}
		self.user_attrs.push(ALLEGRO_VERTEX_ELEMENT {
			attribute: ALLEGRO_PRIM_USER_ATTR + self.user_attrs.len() as c_int,
			storage: storage as c_int,
			offset: offset as c_int,
		});
		Ok(self)
	}
}

pub struct VertexDecl
{
	decl: *mut ALLEGRO_VERTEX_DECL,
}

impl VertexDecl
{
	pub fn new() -> Self
	{
		VertexDecl {
			decl: ptr::null_mut(),
		}
	}

	pub fn from_builder(_: &PrimitivesAddon, builder: &VertexDeclBuilder) -> Self
	{
		let mut elements = vec![];
		if let Some(pos) = builder.pos
		{
			elements.push(pos);
		}
		if let Some(color) = builder.color
		{
			elements.push(color);
		}
		if let Some(uv) = builder.uv
		{
			elements.push(uv);
		}
		if let Some(uv_pixel) = builder.uv_pixel
		{
			elements.push(uv_pixel);
		}
		for &user_attr in &builder.user_attrs
		{
			elements.push(user_attr);
		}
		elements.push(ALLEGRO_VERTEX_ELEMENT {
			attribute: 0,
			storage: 0,
			offset: 0,
		});
		unsafe {
			VertexDecl {
				decl: al_create_vertex_decl(elements.as_ptr(), builder.stride as c_int),
			}
		}
	}

	fn get_allegro_decl(&self) -> *const ALLEGRO_VERTEX_DECL
	{
		self.decl
	}
}

impl Drop for VertexDecl
{
	fn drop(&mut self)
	{
		unsafe {
			if !self.decl.is_null()
			{
				al_destroy_vertex_decl(self.decl);
			}
		}
	}
}

pub trait VertexSource
{
	type VertexType: VertexType;
	fn get_ptr(&self) -> *const u8
	{
		ptr::null()
	}
}

pub unsafe trait VertexType
{
	fn get_decl(prim: &PrimitivesAddon) -> VertexDecl;
}

unsafe impl VertexType for Vertex
{
	fn get_decl(_: &PrimitivesAddon) -> VertexDecl
	{
		VertexDecl::new()
	}
}

impl<T: VertexType> VertexSource for [T]
{
	type VertexType = T;

	fn get_ptr(&self) -> *const u8
	{
		self.as_ptr() as *const _
	}
}
