use std::libc::*;
use std::cast;
use std::str;

use ffi::*;

flag_type!(
	StickFlags
	{
		DIGITAL = ALLEGRO_JOYFLAG_DIGITAL,
		ANALOGUE = ALLEGRO_JOYFLAG_ANALOGUE
	}
)

pub struct Joystick
{
	priv allegro_joystick: *mut ALLEGRO_JOYSTICK
}

impl Joystick
{
	pub fn is_active(&self) -> bool
	{
		unsafe
		{
			al_get_joystick_active(self.allegro_joystick) != 0
		}
	}

	pub fn get_name(&self) -> Option<~str>
	{
		unsafe
		{
			let ptr = al_get_joystick_name(self.allegro_joystick);
			if !ptr.is_null()
			{
				Some(str::raw::from_c_str(ptr))
			}
			else
			{
				None
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
			cast::transmute(al_get_joystick_stick_flags(self.allegro_joystick, stick as c_int))
		}
	}

	pub fn get_stick_name(&self, stick: i32) -> Option<~str>
	{
		unsafe
		{
			let ptr = al_get_joystick_stick_name(self.allegro_joystick, stick as c_int);
			if !ptr.is_null()
			{
				Some(str::raw::from_c_str(ptr))
			}
			else
			{
				None
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

	pub fn get_axis_name(&self, stick: i32, axis: i32) -> Option<~str>
	{
		unsafe
		{
			let ptr = al_get_joystick_axis_name(self.allegro_joystick, stick as c_int, axis as c_int);
			if !ptr.is_null()
			{
				Some(str::raw::from_c_str(ptr))
			}
			else
			{
				None
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

	pub fn get_button_name(&self, button: i32) -> Option<~str>
	{
		unsafe
		{
			let ptr = al_get_joystick_button_name(self.allegro_joystick, button as c_int);
			if !ptr.is_null()
			{
				Some(str::raw::from_c_str(ptr))
			}
			else
			{
				None
			}
		}
	}

	pub fn get_allegro_joystick(&self) -> *mut ALLEGRO_JOYSTICK
	{
		self.allegro_joystick
	}
}

impl ::internal::core::Core
{
	pub fn get_joystick(&self, idx: i32) -> Option<Joystick>
	{
		if self.is_joystick_installed()
		{
			let ptr = unsafe
			{
				al_get_joystick(idx as i32)
			};
			if !ptr.is_null()
			{
				Some(Joystick{ allegro_joystick: ptr })
			}
			else
			{
				None
			}
		}
		else
		{
			None
		}
	}
}
