// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use crate::core::Core;

use allegro_sys::*;
use allegro_util::{Flag, from_c_str};
use libc::*;
use std::mem;

allegro_util::flag_type! {
	StickFlags
	{
		DIGITAL = ALLEGRO_JOYFLAG_DIGITAL as u32,
		ANALOGUE = ALLEGRO_JOYFLAG_ANALOGUE as u32
	}
}

#[cfg(all(feature = "unstable", allegro_5_2_11))]
#[repr(u32)]
#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug, PartialOrd, Ord)]
pub enum JoystickType
{
	Gamepad = ALLEGRO_JOYSTICK_TYPE_GAMEPAD as u32,
	Unknown = ALLEGRO_JOYSTICK_TYPE_UNKNOWN as u32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum JoystickStick
{
	#[cfg(all(feature = "unstable", allegro_5_2_11))]
	DPad,
	#[cfg(all(feature = "unstable", allegro_5_2_11))]
	LeftThumb,
	#[cfg(all(feature = "unstable", allegro_5_2_11))]
	RightThumb,
	#[cfg(all(feature = "unstable", allegro_5_2_11))]
	LeftTrigger,
	#[cfg(all(feature = "unstable", allegro_5_2_11))]
	RightTrigger,
	Generic(i32),
}

impl JoystickStick
{
	fn to_allegro(self) -> i32
	{
		match self
		{
			#[cfg(all(feature = "unstable", allegro_5_2_11))]
			JoystickStick::DPad => ALLEGRO_GAMEPAD_STICK_DPAD,
			#[cfg(all(feature = "unstable", allegro_5_2_11))]
			JoystickStick::LeftThumb => ALLEGRO_GAMEPAD_STICK_LEFT_THUMB,
			#[cfg(all(feature = "unstable", allegro_5_2_11))]
			JoystickStick::RightThumb => ALLEGRO_GAMEPAD_STICK_RIGHT_THUMB,
			#[cfg(all(feature = "unstable", allegro_5_2_11))]
			JoystickStick::LeftTrigger => ALLEGRO_GAMEPAD_STICK_LEFT_TRIGGER,
			#[cfg(all(feature = "unstable", allegro_5_2_11))]
			JoystickStick::RightTrigger => ALLEGRO_GAMEPAD_STICK_RIGHT_TRIGGER,
			JoystickStick::Generic(x) => x,
		}
	}

	pub(crate) fn from_allegro(joystick: &Joystick, stick: i32) -> JoystickStick
	{
		if joystick.is_gamepad()
		{
			match stick
			{
				#[cfg(all(feature = "unstable", allegro_5_2_11))]
				ALLEGRO_GAMEPAD_STICK_DPAD => JoystickStick::DPad,
				#[cfg(all(feature = "unstable", allegro_5_2_11))]
				ALLEGRO_GAMEPAD_STICK_LEFT_THUMB => JoystickStick::LeftThumb,
				#[cfg(all(feature = "unstable", allegro_5_2_11))]
				ALLEGRO_GAMEPAD_STICK_RIGHT_THUMB => JoystickStick::RightThumb,
				#[cfg(all(feature = "unstable", allegro_5_2_11))]
				ALLEGRO_GAMEPAD_STICK_LEFT_TRIGGER => JoystickStick::LeftTrigger,
				#[cfg(all(feature = "unstable", allegro_5_2_11))]
				ALLEGRO_GAMEPAD_STICK_RIGHT_TRIGGER => JoystickStick::RightTrigger,
				_ => JoystickStick::Generic(stick),
			}
		}
		else
		{
			JoystickStick::Generic(stick)
		}
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum JoystickButton
{
	#[cfg(all(feature = "unstable", allegro_5_2_11))]
	A,
	#[cfg(all(feature = "unstable", allegro_5_2_11))]
	B,
	#[cfg(all(feature = "unstable", allegro_5_2_11))]
	X,
	#[cfg(all(feature = "unstable", allegro_5_2_11))]
	Y,
	#[cfg(all(feature = "unstable", allegro_5_2_11))]
	LeftShoulder,
	#[cfg(all(feature = "unstable", allegro_5_2_11))]
	RightShoulder,
	#[cfg(all(feature = "unstable", allegro_5_2_11))]
	Back,
	#[cfg(all(feature = "unstable", allegro_5_2_11))]
	Start,
	#[cfg(all(feature = "unstable", allegro_5_2_11))]
	Guide,
	#[cfg(all(feature = "unstable", allegro_5_2_11))]
	LeftThumb,
	#[cfg(all(feature = "unstable", allegro_5_2_11))]
	RightThumb,
	Generic(i32),
}

impl JoystickButton
{
	fn to_allegro(self) -> i32
	{
		match self
		{
			#[cfg(all(feature = "unstable", allegro_5_2_11))]
			JoystickButton::A => ALLEGRO_GAMEPAD_BUTTON_A,
			#[cfg(all(feature = "unstable", allegro_5_2_11))]
			JoystickButton::B => ALLEGRO_GAMEPAD_BUTTON_B,
			#[cfg(all(feature = "unstable", allegro_5_2_11))]
			JoystickButton::X => ALLEGRO_GAMEPAD_BUTTON_X,
			#[cfg(all(feature = "unstable", allegro_5_2_11))]
			JoystickButton::Y => ALLEGRO_GAMEPAD_BUTTON_Y,
			#[cfg(all(feature = "unstable", allegro_5_2_11))]
			JoystickButton::LeftShoulder => ALLEGRO_GAMEPAD_BUTTON_LEFT_SHOULDER,
			#[cfg(all(feature = "unstable", allegro_5_2_11))]
			JoystickButton::RightShoulder => ALLEGRO_GAMEPAD_BUTTON_RIGHT_SHOULDER,
			#[cfg(all(feature = "unstable", allegro_5_2_11))]
			JoystickButton::Back => ALLEGRO_GAMEPAD_BUTTON_BACK,
			#[cfg(all(feature = "unstable", allegro_5_2_11))]
			JoystickButton::Start => ALLEGRO_GAMEPAD_BUTTON_START,
			#[cfg(all(feature = "unstable", allegro_5_2_11))]
			JoystickButton::Guide => ALLEGRO_GAMEPAD_BUTTON_GUIDE,
			#[cfg(all(feature = "unstable", allegro_5_2_11))]
			JoystickButton::LeftThumb => ALLEGRO_GAMEPAD_BUTTON_LEFT_THUMB,
			#[cfg(all(feature = "unstable", allegro_5_2_11))]
			JoystickButton::RightThumb => ALLEGRO_GAMEPAD_BUTTON_RIGHT_THUMB,
			JoystickButton::Generic(x) => x,
		}
	}

	pub(crate) fn from_allegro(joystick: &Joystick, button: i32) -> JoystickButton
	{
		if joystick.is_gamepad()
		{
			match button
			{
				#[cfg(all(feature = "unstable", allegro_5_2_11))]
				ALLEGRO_GAMEPAD_BUTTON_A => JoystickButton::A,
				#[cfg(all(feature = "unstable", allegro_5_2_11))]
				ALLEGRO_GAMEPAD_BUTTON_B => JoystickButton::B,
				#[cfg(all(feature = "unstable", allegro_5_2_11))]
				ALLEGRO_GAMEPAD_BUTTON_X => JoystickButton::X,
				#[cfg(all(feature = "unstable", allegro_5_2_11))]
				ALLEGRO_GAMEPAD_BUTTON_Y => JoystickButton::Y,
				#[cfg(all(feature = "unstable", allegro_5_2_11))]
				ALLEGRO_GAMEPAD_BUTTON_LEFT_SHOULDER => JoystickButton::LeftShoulder,
				#[cfg(all(feature = "unstable", allegro_5_2_11))]
				ALLEGRO_GAMEPAD_BUTTON_RIGHT_SHOULDER => JoystickButton::RightShoulder,
				#[cfg(all(feature = "unstable", allegro_5_2_11))]
				ALLEGRO_GAMEPAD_BUTTON_BACK => JoystickButton::Back,
				#[cfg(all(feature = "unstable", allegro_5_2_11))]
				ALLEGRO_GAMEPAD_BUTTON_START => JoystickButton::Start,
				#[cfg(all(feature = "unstable", allegro_5_2_11))]
				ALLEGRO_GAMEPAD_BUTTON_GUIDE => JoystickButton::Guide,
				#[cfg(all(feature = "unstable", allegro_5_2_11))]
				ALLEGRO_GAMEPAD_BUTTON_LEFT_THUMB => JoystickButton::LeftThumb,
				#[cfg(all(feature = "unstable", allegro_5_2_11))]
				ALLEGRO_GAMEPAD_BUTTON_RIGHT_THUMB => JoystickButton::RightThumb,
				_ => JoystickButton::Generic(button),
			}
		}
		else
		{
			JoystickButton::Generic(button)
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
#[allow(missing_copy_implementations)]
pub struct Joystick
{
	allegro_joystick: *mut ALLEGRO_JOYSTICK,
}

impl Joystick
{
	pub fn new(core: &Core, idx: i32) -> Result<Joystick, ()>
	{
		assert!(
			core.is_joystick_installed(),
			"Joystick support not installed!"
		);
		let ptr = unsafe { al_get_joystick(idx as i32) };
		if !ptr.is_null()
		{
			Ok(Joystick {
				allegro_joystick: ptr,
			})
		}
		else
		{
			Err(())
		}
	}

	pub unsafe fn wrap(joystick: *mut ALLEGRO_JOYSTICK) -> Joystick
	{
		Joystick {
			allegro_joystick: joystick,
		}
	}

	pub fn is_active(&self) -> bool
	{
		unsafe { al_get_joystick_active(self.allegro_joystick) != 0 }
	}

	pub fn get_name(&self) -> Result<String, ()>
	{
		unsafe {
			let ptr = al_get_joystick_name(self.allegro_joystick);
			if !ptr.is_null()
			{
				Ok(from_c_str(ptr))
			}
			else
			{
				Err(())
			}
		}
	}

	pub fn get_num_sticks(&self) -> i32
	{
		unsafe { al_get_joystick_num_sticks(self.allegro_joystick) as i32 }
	}

	pub fn get_stick_flags(&self, stick: JoystickStick) -> StickFlags
	{
		unsafe {
			mem::transmute(al_get_joystick_stick_flags(
				self.allegro_joystick,
				stick.to_allegro(),
			))
		}
	}

	pub fn get_stick_name(&self, stick: JoystickStick) -> Result<String, ()>
	{
		unsafe {
			let ptr = al_get_joystick_stick_name(self.allegro_joystick, stick.to_allegro());
			if !ptr.is_null()
			{
				Ok(from_c_str(ptr))
			}
			else
			{
				Err(())
			}
		}
	}

	pub fn get_num_axes(&self, stick: JoystickStick) -> i32
	{
		unsafe { al_get_joystick_num_axes(self.allegro_joystick, stick.to_allegro()) as i32 }
	}

	pub fn get_axis_name(&self, stick: JoystickStick, axis: i32) -> Result<String, ()>
	{
		unsafe {
			let ptr =
				al_get_joystick_axis_name(self.allegro_joystick, stick.to_allegro(), axis as c_int);
			if !ptr.is_null()
			{
				Ok(from_c_str(ptr))
			}
			else
			{
				Err(())
			}
		}
	}

	pub fn get_num_buttons(&self) -> i32
	{
		unsafe { al_get_joystick_num_buttons(self.allegro_joystick) as i32 }
	}

	pub fn get_button_name(&self, button: JoystickButton) -> Result<String, ()>
	{
		unsafe {
			let ptr = al_get_joystick_button_name(self.allegro_joystick, button.to_allegro());
			if !ptr.is_null()
			{
				Ok(from_c_str(ptr))
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

	#[cfg(all(feature = "unstable", allegro_5_2_11))]
	pub fn get_type(&self) -> JoystickType
	{
		unsafe { mem::transmute(al_get_joystick_type(self.allegro_joystick)) }
	}

	fn is_gamepad(&self) -> bool
	{
		#![allow(unused_assignments, unused_mut)]
		let mut is_gamepad = false;
		#[cfg(all(feature = "unstable", allegro_5_2_11))]
		{
			is_gamepad = self.get_type() == JoystickType::Gamepad;
		}
		is_gamepad
	}
}
