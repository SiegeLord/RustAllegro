// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;
use std::mem;
use std::kinds::marker::NoSend;

use ffi::*;
use rust_util::Flag;
use internal::core::Core;

flag_type!{
	StickFlags
	{
		DIGITAL = ALLEGRO_JOYFLAG_DIGITAL,
		ANALOGUE = ALLEGRO_JOYFLAG_ANALOGUE
	}
}

#[allow(missing_copy_implementations)]
pub struct Joystick
{
	allegro_joystick: *mut ALLEGRO_JOYSTICK,
	no_send_marker: NoSend,
}

impl Joystick
{
	pub fn new(core: &Core, idx: i32) -> Result<Joystick, ()>
	{
		assert!(core.is_joystick_installed(), "Joystick support not installed!");
		let ptr = unsafe
		{
			al_get_joystick(idx as i32)
		};
		if !ptr.is_null()
		{
			Ok(Joystick{ allegro_joystick: ptr, no_send_marker: NoSend })
		}
		else
		{
			Err(())
		}
	}

	pub fn is_active(&self) -> bool
	{
		unsafe
		{
			al_get_joystick_active(self.allegro_joystick) != 0
		}
	}

	pub fn get_name(&self) -> Result<String, ()>
	{
		unsafe
		{
			let ptr = al_get_joystick_name(self.allegro_joystick);
			if !ptr.is_null()
			{
				Ok(String::from_raw_buf(ptr as *const _))
			}
			else
			{
				Err(())
			}
		}
	}

	pub fn get_num_sticks(&self) -> i32
	{
		unsafe
		{
			al_get_joystick_num_sticks(self.allegro_joystick) as i32
		}
	}

	pub fn get_stick_flags(&self, stick: i32) -> StickFlags
	{
		unsafe
		{
			mem::transmute(al_get_joystick_stick_flags(self.allegro_joystick, stick as c_int))
		}
	}

	pub fn get_stick_name(&self, stick: i32) -> Result<String, ()>
	{
		unsafe
		{
			let ptr = al_get_joystick_stick_name(self.allegro_joystick, stick as c_int);
			if !ptr.is_null()
			{
				Ok(String::from_raw_buf(ptr as *const _))
			}
			else
			{
				Err(())
			}
		}
	}

	pub fn get_num_axes(&self, stick: i32) -> i32
	{
		unsafe
		{
			al_get_joystick_num_axes(self.allegro_joystick, stick as c_int) as i32
		}
	}

	pub fn get_axis_name(&self, stick: i32, axis: i32) -> Result<String, ()>
	{
		unsafe
		{
			let ptr = al_get_joystick_axis_name(self.allegro_joystick, stick as c_int, axis as c_int);
			if !ptr.is_null()
			{
				Ok(String::from_raw_buf(ptr as *const _))
			}
			else
			{
				Err(())
			}
		}
	}

	pub fn get_num_buttons(&self) -> i32
	{
		unsafe
		{
			al_get_joystick_num_buttons(self.allegro_joystick) as i32
		}
	}

	pub fn get_button_name(&self, button: i32) -> Result<String, ()>
	{
		unsafe
		{
			let ptr = al_get_joystick_button_name(self.allegro_joystick, button as c_int);
			if !ptr.is_null()
			{
				Ok(String::from_raw_buf(ptr as *const _))
			}
			else
			{
				Err(())
			}
		}
	}

	pub fn get_allegro_joystick(&self) -> *mut ALLEGRO_JOYSTICK
	{
		self.allegro_joystick
	}
}
