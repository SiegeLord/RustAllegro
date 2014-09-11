// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;
use std::mem;

use ffi::*;

pub struct Transform(ALLEGRO_TRANSFORM);

pub mod external
{
	pub use super::Transform;
}

impl Transform
{
	pub fn identity() -> Transform
	{
		Transform(unsafe
		{
			let mut t = mem::uninitialized();
			al_identity_transform(&mut t);
			t
		})
	}

	pub fn build(x: f32, y: f32, sx: f32, sy: f32, theta: f32) -> Transform
	{
		Transform(unsafe
		{
			let mut t = mem::uninitialized();
			al_build_transform(&mut t, x as c_float, y as c_float, sx as c_float, sy as c_float, theta as c_float);
			t
		})
	}

	pub fn translate(&mut self, x: f32, y: f32)
	{
		unsafe
		{
			al_translate_transform(&mut **self, x as c_float, y as c_float);
		}
	}

	pub fn rotate(&mut self, theta: f32)
	{
		unsafe
		{
			al_rotate_transform(&mut **self, theta as c_float);
		}
	}

	pub fn scale(&mut self, sx: f32, sy: f32)
	{
		unsafe
		{
			al_scale_transform(&mut **self, sx as c_float, sy as c_float);
		}
	}

	pub fn transform_coordinates(&self, x: f32, y: f32) -> (f32, f32)
	{
		let mut x = x as c_float;
		let mut y = y as c_float;
		unsafe
		{
			al_transform_coordinates(&**self, &mut x, &mut y);
		}
		(x as f32, y as f32)
	}

	pub fn compose(&mut self, other: &Transform)
	{
		unsafe
		{
			al_compose_transform(&mut **self, &**other);
		}
	}

	pub fn invert(&mut self)
	{
		unsafe
		{
			al_invert_transform(&mut **self);
		}
	}

	pub fn check_inverse(&self, tol: f32) -> bool
	{
		unsafe
		{
			al_check_inverse(&**self, tol as c_float) != 0
		}
	}
}

impl Deref<ALLEGRO_TRANSFORM> for Transform
{
	fn deref<'l>(&'l self) -> &'l ALLEGRO_TRANSFORM
	{
		let Transform(ref t) = *self;
		t
	}
}


impl DerefMut<ALLEGRO_TRANSFORM> for Transform
{
	fn deref_mut<'l>(&'l mut self) -> &'l mut ALLEGRO_TRANSFORM
	{
		let Transform(ref mut t) = *self;
		t
	}
}

pub fn new_transform_wrap(trans: ALLEGRO_TRANSFORM) -> Transform
{
	Transform(trans)
}
