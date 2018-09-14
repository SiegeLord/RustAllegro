// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use ffi::*;
use libc::*;
use std::mem;

#[derive(Copy, Clone, Debug)]
pub struct Transform(ALLEGRO_TRANSFORM);

unsafe impl Send for Transform {}

impl Transform
{
	pub fn identity() -> Transform
	{
		Transform(unsafe {
			let mut t = mem::uninitialized();
			al_identity_transform(&mut t);
			t
		})
	}

	pub fn wrap(t: ALLEGRO_TRANSFORM) -> Transform
	{
		Transform(t)
	}

	pub fn build(x: f32, y: f32, sx: f32, sy: f32, theta: f32) -> Transform
	{
		Transform(unsafe {
			let mut t = mem::uninitialized();
			al_build_transform(&mut t, x as c_float, y as c_float, sx as c_float, sy as c_float, theta as c_float);
			t
		})
	}

	pub fn get_allegro_transform(&self) -> ALLEGRO_TRANSFORM
	{
		self.0
	}

	pub fn get_matrix(&self) -> &[[f32; 4]; 4]
	{
		&self.0.m
	}

	pub fn get_matrix_mut(&mut self) -> &mut [[f32; 4]; 4]
	{
		&mut self.0.m
	}

	pub fn translate(&mut self, x: f32, y: f32)
	{
		unsafe {
			al_translate_transform(&mut self.0, x as c_float, y as c_float);
		}
	}

	pub fn rotate(&mut self, theta: f32)
	{
		unsafe {
			al_rotate_transform(&mut self.0, theta as c_float);
		}
	}

	pub fn scale(&mut self, sx: f32, sy: f32)
	{
		unsafe {
			al_scale_transform(&mut self.0, sx as c_float, sy as c_float);
		}
	}

	pub fn transform_coordinates(&self, x: f32, y: f32) -> (f32, f32)
	{
		let mut x = x as c_float;
		let mut y = y as c_float;
		unsafe {
			al_transform_coordinates(&self.0, &mut x, &mut y);
		}
		(x as f32, y as f32)
	}

	pub fn compose(&mut self, other: &Transform)
	{
		unsafe {
			al_compose_transform(&mut self.0, &other.0);
		}
	}

	pub fn invert(&mut self)
	{
		unsafe {
			al_invert_transform(&mut self.0);
		}
	}

	pub fn check_inverse(&self, tol: f32) -> bool
	{
		unsafe { al_check_inverse(&self.0, tol as c_float) != 0 }
	}
}
