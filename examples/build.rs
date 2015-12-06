// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use std::env::var;

fn main()
{
	if let Ok(path) = var("RUSTALLEGRO_EXAMPLE_LINK_PATH")
	{
		println!("cargo:rustc-flags=-L {}", path);
	}
}
