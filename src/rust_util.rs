// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::c_char;
use std::ffi::CStr;

#[allow(non_camel_case_types)]
pub type c_bool = u8;

pub trait Flag
{
	fn zero() -> Self;
}

#[allow(dead_code)]
pub unsafe fn from_c_str(c_str: *const c_char) -> String
{
	String::from_utf8_lossy(CStr::from_ptr(c_str as *const _).to_bytes()).into_owned()
}
