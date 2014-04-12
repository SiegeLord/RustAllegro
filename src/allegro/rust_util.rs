// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#[allow(non_camel_case_types)]
pub type c_bool = u8;

pub trait Flag
{
	fn zero() -> Self;
}
