// This file is released into Public Domain.
#[macro_use]
extern crate allegro;

use allegro::*;
use std::env;

allegro_main!
{
	let args = env::args().collect::<Vec<_>>();

	if args.len() < 2
	{
		println!("Usage: {} test_config.cfg", args[0]);
		return;
	}

	let cfg = Config::load(&args[1]).unwrap();
	for section in cfg.sections()
	{
		if !section.is_empty()
		{
			println!("[{}]", section);
		}
		for key in cfg.keys(&section)
		{
			let value = cfg.get_value(&section, &key).expect("<not found or invalid utf8>");
			println!("{} = {}", key, value);
		}
	}
}
