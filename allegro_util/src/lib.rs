// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

extern crate libc;

use libc::c_char;
use std::ffi::CStr;

#[allow(non_camel_case_types)]
pub type c_bool = u8;

pub trait Flag
{
	fn zero() -> Self;
}

pub unsafe fn from_c_str(c_str: *const c_char) -> String
{
	String::from_utf8_lossy(CStr::from_ptr(c_str as *const _).to_bytes()).into_owned()
}

#[macro_export]
macro_rules! if_ok
{
	($e: expr) =>
	{
		if ($e).is_err()
		{
			return Err(());
		}
	}
}

#[macro_export]
macro_rules! opaque
{
	($f: ident) =>
	{
		/* Mimicking c_void */
		#[allow(missing_copy_implementations)]
		pub enum $f
		{
		}
	}
}

#[macro_export]
macro_rules! derive_copy_clone
{
	($t: ty) =>
	{
		impl Copy for $t {}
		impl Clone for $t
		{
			fn clone(&self) -> Self
			{
				*self
			}
		}
	}
}

#[macro_export]
macro_rules! flag_type
{
	($f: ident { $($n: ident = $v: expr),*}) =>
	{
		#[derive(Copy, Clone)]
		pub struct $f
		{
			bits: u32
		}

		impl $f
		{
			#[inline]
			pub fn get(&self) -> u32
			{
				self.bits
			}
		}

		impl Flag for $f
		{
			fn zero() -> $f
			{
				$f{bits: 0}
			}
		}

		impl ::std::ops::BitOr for $f
		{
			type Output = $f;
			fn bitor(self, e: $f) -> $f
			{
				$f{bits: self.bits | e.bits}
			}
		}

		impl ::std::ops::BitAnd for $f
		{
			type Output = bool;
			fn bitand(self, e: $f) -> bool
			{
				self.bits & e.bits != 0
			}
		}

		$(
			pub const $n: $f = $f{bits: $v};
		)+
	}
}

#[macro_export]
macro_rules! cast_to_c
{
	($p:ident, f32) =>
	{
		$p as c_float
	};
	($p:ident, Color) =>
	{
		*$p
	}
}
