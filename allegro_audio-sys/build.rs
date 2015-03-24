// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use std::env::var;

fn main()
{
	if var("CARGO_FEATURE_LINK_NONE").is_ok()
	{
		return;
	}

	let debug = match var("CARGO_FEATURE_LINK_DEBUG")
	{
		Err(_) => "",
		Ok(_) => "-debug"
	};

	let static_ = match var("CARGO_FEATURE_LINK_STATIC")
	{
		Err(_) => "",
		Ok(_) => "-static"
	};

	if cfg!(windows)
	{
        // Windows needs a little extra help.
        let home = var("ALLEGRO_HOME").ok().expect("ALLEGRO_HOME must be set to the root of your Allegro 5 installation.");
	    let version = var("ALLEGRO_VERSION").ok().expect("ALLEGRO_VERSION must be set to the version of Allegro 5 you have installed.");
	    println!("cargo:rustc-link-search={}\\bin", home);
	    println!("cargo:rustc-link-lib=allegro_audio-{}{}{}{}", version, static_, "-mt", debug);
    }
    else
    {
        println!("cargo:rustc-flags=-l allegro_audio{}{}", static_, debug);
    }
}
