// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use allegro::{BitmapLike, Color, Core, Display};
use allegro_primitives_sys::*;
use allegro_sys::*;
use allegro_util::Flag;
use libc::*;

use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ptr;
use std::sync::Arc;

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

#[cfg(any(allegro_5_2_0, allegro_5_1_0))]
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum LineJoinType
{
	None = ALLEGRO_LINE_JOIN_NONE,
	Bevel = ALLEGRO_LINE_JOIN_BEVEL,
	Round = ALLEGRO_LINE_JOIN_ROUND,
	Mitre = ALLEGRO_LINE_JOIN_MITRE,
}

#[cfg(any(allegro_5_2_0, allegro_5_1_0))]
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum LineCapType
{
	None = ALLEGRO_LINE_CAP_NONE,
	Squared = ALLEGRO_LINE_CAP_SQUARE,
	Round = ALLEGRO_LINE_CAP_ROUND,
	Triangle = ALLEGRO_LINE_CAP_TRIANGLE,
	Closed = ALLEGRO_LINE_CAP_CLOSED,
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
		static RUN_ONCE: Once = Once::new();

		let mut res = Err("The primitives addon already initialized.".into());
		RUN_ONCE.call_once(|| unsafe {
			res = if al_init_primitives_addon() != 0
			{
				Ok(PrimitivesAddon { _dummy: () })
			}
			else
			{
				Err("Could not initialize the primitives addon.".into())
			};
		});
		res
	}

	pub fn get_version() -> i32
	{
		unsafe { al_get_allegro_primitives_version() as i32 }
	}

	pub fn draw_prim<T: VertexType, B: BitmapLike>(
		&self, vtxs: &[T], texture: Option<&B>, start: u32, end: u32, type_: PrimType,
	) -> u32
	{
		check_valid_target_bitmap();
		let tex = texture.map_or(ptr::null_mut(), |bmp| bmp.get_allegro_bitmap());
		unsafe {
			al_draw_prim(
				vtxs.as_ptr() as *const _,
				T::get_decl(self).get_allegro_decl(),
				tex,
				start as c_int,
				end as c_int,
				type_ as c_int,
			) as u32
		}
	}

	pub fn draw_indexed_prim<T: VertexType, B: BitmapLike>(
		&self, vtxs: &[T], texture: Option<&B>, indices: &[i32], start: u32, end: u32,
		type_: PrimType,
	) -> u32
	{
		check_valid_target_bitmap();
		let tex = texture.map_or(ptr::null_mut(), |bmp| bmp.get_allegro_bitmap());
		unsafe {
			al_draw_indexed_prim(
				vtxs.as_ptr() as *const _,
				T::get_decl(self).get_allegro_decl(),
				tex,
				indices[start as usize..].as_ptr(),
				(end - start) as c_int,
				type_ as c_int,
			) as u32
		}
	}

	pub fn draw_vertex_buffer<T: VertexType, B: BitmapLike>(
		&self, vertex_buffer: &VertexBuffer<T>, texture: Option<&B>, start: u32, end: u32,
		type_: PrimType,
	) -> u32
	{
		check_valid_target_bitmap();
		let tex = texture.map_or(ptr::null_mut(), |bmp| bmp.get_allegro_bitmap());
		unsafe {
			al_draw_vertex_buffer(
				vertex_buffer.get_allegro_buffer(),
				tex,
				start as c_int,
				end as c_int,
				type_ as c_int,
			) as u32
		}
	}

	pub fn draw_indexed_buffer<V: VertexType, I: IndexType, B: BitmapLike>(
		&self, vertex_buffer: &VertexBuffer<V>, texture: Option<&B>, index_buffer: &IndexBuffer<I>,
		start: u32, end: u32, type_: PrimType,
	) -> u32
	{
		check_valid_target_bitmap();
		let tex = texture.map_or(ptr::null_mut(), |bmp| bmp.get_allegro_bitmap());
		unsafe {
			al_draw_indexed_buffer(
				vertex_buffer.get_allegro_buffer(),
				tex,
				index_buffer.get_allegro_buffer(),
				start as c_int,
				end as c_int,
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

	#[cfg(any(allegro_5_2_0, allegro_5_1_0))]
	pub fn draw_polyline(
		&self, vertices: &[(f32, f32)], join_style: LineJoinType, cap_style: LineCapType,
		color: Color, thickness: f32, miter_limit: f32,
	)
	{
		check_valid_target_bitmap();
		let mut c_vertices = Vec::with_capacity(2 * vertices.len());
		for &(x, y) in vertices
		{
			c_vertices.push(x);
			c_vertices.push(y);
		}

		unsafe {
			al_draw_polyline(
				c_vertices.as_ptr(),
				(std::mem::size_of::<c_float>() * 2) as c_int,
				vertices.len() as c_int,
				join_style as c_int,
				cap_style as c_int,
				color.get_allegro_color(),
				thickness as c_float,
				miter_limit as c_float,
			);
		}
	}

	#[cfg(any(allegro_5_2_0, allegro_5_1_0))]
	pub fn draw_polygon(
		&self, vertices: &[(f32, f32)], join_style: LineJoinType, color: Color, thickness: f32,
		miter_limit: f32,
	)
	{
		check_valid_target_bitmap();
		let mut c_vertices = Vec::with_capacity(2 * vertices.len());
		for &(x, y) in vertices
		{
			c_vertices.push(x);
			c_vertices.push(y);
		}

		unsafe {
			al_draw_polygon(
				c_vertices.as_ptr(),
				vertices.len() as c_int,
				join_style as c_int,
				color.get_allegro_color(),
				thickness as c_float,
				miter_limit as c_float,
			);
		}
	}

	#[cfg(any(allegro_5_2_0, allegro_5_1_0))]
	pub fn draw_filled_polygon(&self, vertices: &[(f32, f32)], color: Color)
	{
		check_valid_target_bitmap();
		let mut c_vertices = Vec::with_capacity(2 * vertices.len());
		for &(x, y) in vertices
		{
			c_vertices.push(x);
			c_vertices.push(y);
		}

		unsafe {
			al_draw_filled_polygon(
				c_vertices.as_ptr(),
				vertices.len() as c_int,
				color.get_allegro_color(),
			);
		}
	}

	#[cfg(any(allegro_5_2_0, allegro_5_1_0))]
	pub fn draw_filled_polygon_with_holes(
		&self, vertices: &[(f32, f32)], holes: &[&[(f32, f32)]], color: Color,
	)
	{
		let mut counts = Vec::with_capacity(2 + holes.len());
		counts.push(vertices.len() as c_int);
		for hole in holes
		{
			counts.push(hole.len() as c_int);
		}
		counts.push(0);

		check_valid_target_bitmap();
		let mut c_vertices = Vec::with_capacity(2 * counts.iter().sum::<c_int>() as usize);
		for &(x, y) in vertices
		{
			c_vertices.push(x);
			c_vertices.push(y);
		}
		for hole in holes
		{
			for &(x, y) in *hole
			{
				c_vertices.push(x);
				c_vertices.push(y);
			}
		}

		unsafe {
			al_draw_filled_polygon_with_holes(
				c_vertices.as_ptr(),
				counts.as_ptr(),
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

	pub fn user_attr(mut self, storage: VertexAttrStorage, offset: usize) -> Result<Self, ()>
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

allegro_util::flag_type! {
	BufferFlags
	{
		BUFFER_STREAM = ALLEGRO_PRIM_BUFFER_STREAM,
		BUFFER_STATIC = ALLEGRO_PRIM_BUFFER_STATIC,
		BUFFER_DYNAMIC = ALLEGRO_PRIM_BUFFER_DYNAMIC,
		BUFFER_READWRITE = ALLEGRO_PRIM_BUFFER_READWRITE
	}
}

pub struct VertexBufferLock<'l, T: VertexType, D>
{
	buffer: &'l mut VertexBuffer<T>,
	data: *mut [D],
}

impl<'l, T: VertexType, D> VertexBufferLock<'l, T, D>
{
	pub fn as_mut(&mut self) -> &mut [D]
	{
		unsafe { self.data.as_mut().unwrap() }
	}
}

impl<'l, T: VertexType + Copy> VertexBufferLock<'l, T, MaybeUninit<T>>
{
	pub fn copy_from_slice(&mut self, src: &[T])
	{
		for (d, s) in &mut self.as_mut().iter_mut().zip(src.iter())
		{
			d.write(*s);
		}
	}
}

impl<'l, T: VertexType, D> Drop for VertexBufferLock<'l, T, D>
{
	fn drop(&mut self)
	{
		unsafe {
			al_unlock_vertex_buffer(self.buffer.vertex_buffer);
		}
	}
}

pub struct VertexBuffer<T: VertexType>
{
	vertex_buffer: *mut ALLEGRO_VERTEX_BUFFER,
	#[allow(dead_code)]
	vertex_decl: VertexDecl, // Needs to be alive while the buffer is alive.
	vertex_type: PhantomData<T>,
	#[allow(dead_code)]
	token: Arc<String>,
}

impl<T: VertexType> VertexBuffer<T>
{
	pub fn new(
		display: &mut Display, prim: &PrimitivesAddon, init_data: Option<&[T]>, size: u32,
		flags: BufferFlags,
	) -> Result<Self, ()>
	{
		let token = Arc::new("VertexBuffer".to_string());
		display.add_dependency_token(token.clone());
		let vertex_decl = T::get_decl(prim);
		if let Some(init_data) = init_data
		{
			if init_data.len() as u32 != size
			{
				return Err(());
			}
		}
		let vertex_buffer = unsafe {
			al_create_vertex_buffer(
				vertex_decl.get_allegro_decl() as *mut _,
				init_data.map_or(ptr::null_mut(), |v| v.as_ptr() as *const _),
				size as c_int,
				flags.get() as c_int,
			)
		};

		if vertex_buffer.is_null()
		{
			Err(())
		}
		else
		{
			Ok(VertexBuffer {
				vertex_buffer: vertex_buffer,
				vertex_decl: vertex_decl,
				vertex_type: PhantomData,
				token: token,
			})
		}
	}

	/// Locks the buffer for reading and optionally writing.
	pub fn lock(&mut self, start: u32, len: u32, write: bool)
	-> Result<VertexBufferLock<T, T>, ()>
	{
		let data = unsafe {
			al_lock_vertex_buffer(
				self.vertex_buffer,
				start as c_int,
				len as c_int,
				if write
				{
					ALLEGRO_LOCK_READWRITE as c_int
				}
				else
				{
					ALLEGRO_LOCK_READONLY as c_int
				},
			)
		};
		if data.is_null()
		{
			Err(())
		}
		else
		{
			unsafe {
				Ok(VertexBufferLock {
					buffer: self,
					data: std::slice::from_raw_parts_mut(data as *mut T, len as usize),
				})
			}
		}
	}

	/// Locks the buffer for only writing.
	///
	/// This is unsafe because the contents of the lock are undefined.
	pub fn lock_write_only(
		&mut self, start: u32, len: u32,
	) -> Result<VertexBufferLock<T, MaybeUninit<T>>, ()>
	{
		let data = unsafe {
			al_lock_vertex_buffer(
				self.vertex_buffer,
				start as c_int,
				len as c_int,
				ALLEGRO_LOCK_WRITEONLY as c_int,
			)
		};
		if data.is_null()
		{
			Err(())
		}
		else
		{
			unsafe {
				Ok(VertexBufferLock {
					buffer: self,
					data: std::slice::from_raw_parts_mut(data as *mut MaybeUninit<T>, len as usize),
				})
			}
		}
	}

	pub fn len(&mut self) -> u32
	{
		unsafe { al_get_vertex_buffer_size(self.vertex_buffer) as u32 }
	}

	pub fn get_allegro_buffer(&self) -> *mut ALLEGRO_VERTEX_BUFFER
	{
		self.vertex_buffer
	}
}

impl<T: VertexType> Drop for VertexBuffer<T>
{
	fn drop(&mut self)
	{
		unsafe {
			al_destroy_vertex_buffer(self.vertex_buffer);
		}
	}
}

pub unsafe trait IndexType {}

unsafe impl IndexType for u16 {}
unsafe impl IndexType for u32 {}

pub struct IndexBufferLock<'l, T: IndexType, D>
{
	buffer: &'l mut IndexBuffer<T>,
	data: *mut [D],
}

impl<'l, T: IndexType, D> IndexBufferLock<'l, T, D>
{
	pub fn as_mut(&mut self) -> &mut [D]
	{
		unsafe { self.data.as_mut().unwrap() }
	}
}

impl<'l, T: IndexType + Copy> IndexBufferLock<'l, T, MaybeUninit<T>>
{
	pub fn copy_from_slice(&mut self, src: &[T])
	{
		for (d, s) in &mut self.as_mut().iter_mut().zip(src.iter())
		{
			d.write(*s);
		}
	}
}

impl<'l, T: IndexType, D> Drop for IndexBufferLock<'l, T, D>
{
	fn drop(&mut self)
	{
		unsafe {
			al_unlock_index_buffer(self.buffer.index_buffer);
		}
	}
}

pub struct IndexBuffer<T: IndexType>
{
	index_buffer: *mut ALLEGRO_INDEX_BUFFER,
	index_type: PhantomData<T>,
	#[allow(dead_code)]
	token: Arc<String>,
}

impl<T: IndexType> IndexBuffer<T>
{
	pub fn new(
		display: &mut Display, _: &PrimitivesAddon, init_data: Option<&[T]>, size: u32,
		flags: BufferFlags,
	) -> Result<Self, ()>
	{
		let token = Arc::new("IndexBuffer".to_string());
		display.add_dependency_token(token.clone());
		if let Some(init_data) = init_data
		{
			if init_data.len() as u32 != size
			{
				return Err(());
			}
		}
		let index_buffer = unsafe {
			al_create_index_buffer(
				size_of::<T>() as c_int,
				init_data.map_or(ptr::null_mut(), |v| v.as_ptr() as *const _),
				size as c_int,
				flags.get() as c_int,
			)
		};

		if index_buffer.is_null()
		{
			Err(())
		}
		else
		{
			Ok(IndexBuffer {
				index_buffer: index_buffer,
				index_type: PhantomData,
				token: token,
			})
		}
	}

	/// Locks the buffer for reading and optionally writing.
	pub fn lock(&mut self, start: u32, len: u32, write: bool) -> Result<IndexBufferLock<T, T>, ()>
	{
		let data = unsafe {
			al_lock_index_buffer(
				self.index_buffer,
				start as c_int,
				len as c_int,
				if write
				{
					ALLEGRO_LOCK_READWRITE as c_int
				}
				else
				{
					ALLEGRO_LOCK_READONLY as c_int
				},
			)
		};
		if data.is_null()
		{
			Err(())
		}
		else
		{
			unsafe {
				Ok(IndexBufferLock {
					buffer: self,
					data: std::slice::from_raw_parts_mut(data as *mut T, len as usize),
				})
			}
		}
	}

	/// Locks the buffer for only writing.
	pub fn lock_write_only(
		&mut self, start: u32, len: u32,
	) -> Result<IndexBufferLock<T, MaybeUninit<T>>, ()>
	{
		let data = unsafe {
			al_lock_index_buffer(
				self.index_buffer,
				start as c_int,
				len as c_int,
				ALLEGRO_LOCK_WRITEONLY as c_int,
			)
		};
		if data.is_null()
		{
			Err(())
		}
		else
		{
			unsafe {
				Ok(IndexBufferLock {
					buffer: self,
					data: std::slice::from_raw_parts_mut(data as *mut MaybeUninit<T>, len as usize),
				})
			}
		}
	}

	pub fn len(&mut self) -> u32
	{
		unsafe { al_get_index_buffer_size(self.index_buffer) as u32 }
	}

	pub fn get_allegro_buffer(&self) -> *mut ALLEGRO_INDEX_BUFFER
	{
		self.index_buffer
	}
}

impl<T: IndexType> Drop for IndexBuffer<T>
{
	fn drop(&mut self)
	{
		unsafe {
			al_destroy_index_buffer(self.index_buffer);
		}
	}
}
