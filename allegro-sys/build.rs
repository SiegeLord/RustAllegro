// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![feature(os)]

use std::os::getenv;

fn main()
{
	if getenv("CARGO_FEATURE_LINK_NONE").is_some()
	{
		return;
	}

	let debug = match getenv("CARGO_FEATURE_LINK_DEBUG")
	{
		None => "",
		Some(_) => "-debug"
	};

	let static_ = match getenv("CARGO_FEATURE_LINK_STATIC")
	{
		None => "",
		Some(_) => "-static"
	};

	let monolith = match getenv("CARGO_FEATURE_LINK_MONOLITH")
	{
		None => "",
		Some(_) => "-monolith"
	};

	println!("cargo:rustc-flags=-l allegro{}{}{}", monolith, static_, debug);
}
