// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use std::env::var;
use std::str::FromStr;

fn main()
{
	if let Ok(path) = var("RUSTALLEGRO_EXAMPLE_LINK_PATH")
	{
		println!("cargo:rustc-flags=-L {}", path);
	}

	let allegro_sub_version = u32::from_str(&var("DEP_ALLEGRO_SUB_VERSION").unwrap()).unwrap();
	let allegro_wip_version = u32::from_str(&var("DEP_ALLEGRO_WIP_VERSION").unwrap()).unwrap();

	for v in 0..allegro_wip_version + 1
	{
		println!("cargo:rustc-cfg=allegro_5_{}_{}", allegro_sub_version, v)
	}
}
