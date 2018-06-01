// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use std::env::var;
use std::fs::File;
use std::io::prelude::*;

// Parses lines of the form KEY VALUE, where VALUE may be a quote delimited string.
fn get_define_value<'l>(line: &'l str, key: &'l str) -> Option<&'l str>
{
	if !line.starts_with(key) || !line[key.len()..].starts_with(char::is_whitespace)
	{
		return None;
	}
	let s = &line[key.len()..].trim_left();
	let value_offset = if s.starts_with('"')
	{
		match s[1..].find('"')
		{
			// + 2 because we start with 1 offset, and we actually want the character after the "
			Some(v) => v + 2,
			None => panic!("Couldn't parse value from '{}'", line)
		}
	}
	else
	{
		match s.find(char::is_whitespace)
		{
			Some(v) => v,
			None => s.len()
		}
	};
	Some(&s[..value_offset])
}

fn main()
{
	let mut header_contents = "".to_string();
	let mut include_dirs = vec!["/usr/local/include".to_string(), "/usr/include".to_string(), "include".to_string()];
	if let Ok(include_dir) = var("ALLEGRO_INCLUDE_DIR")
	{
		include_dirs.insert(0, include_dir);
	}
	for include_dir in &include_dirs
	{
		let include_path = include_dir.clone() + "/allegro5/base.h";
		println!("Trying to open {}.", include_path);
		if let Ok(mut header_file) = File::open(&include_path)
		{
			let mut s = String::new();
			header_file.read_to_string(&mut s).unwrap();
			header_contents = s;
			println!("Found Allegro header at {}", include_path);
			break;
		}
	}

	if header_contents.is_empty()
	{
		panic!("Could not find Allegro headers!");
	}

	let mut allegro_version = -1;
	let mut allegro_sub_version = -1;
	let mut allegro_wip_version = -1;
	let mut allegro_release_number = -1;
	let mut allegro_version_str = "";
	let mut allegro_date_str = "";
	let mut allegro_date = -1;

	for line in header_contents.lines()
	{
		let line = line.trim_left();
		if !line.starts_with("#define")
		{
			continue;
		}
		let line = line["#define".len()..].trim_left();
		if let Some(val) = get_define_value(line, "ALLEGRO_VERSION")
		{
			allegro_version = val.parse().unwrap();
		}
		else if let Some(val) = get_define_value(line, "ALLEGRO_SUB_VERSION")
		{
			allegro_sub_version = val.parse().unwrap();
		}
		else if let Some(val) = get_define_value(line, "ALLEGRO_WIP_VERSION")
		{
			allegro_wip_version = val.parse().unwrap();
		}
		else if let Some(val) = get_define_value(line, "ALLEGRO_RELEASE_NUMBER")
		{
			allegro_release_number = val.parse().unwrap();
		}
		else if let Some(val) = get_define_value(line, "ALLEGRO_DATE")
		{
			allegro_date = val.parse().unwrap();
		}
		else if let Some(val) = get_define_value(line, "ALLEGRO_VERSION_STR")
		{
			allegro_version_str = val;
		}
		else if let Some(val) = get_define_value(line, "ALLEGRO_DATE_STR")
		{
			allegro_date_str = val;
		}
	}
	assert!(allegro_version != -1, "Did not find ALLEGRO_VERSION in the header file.");
	assert!(allegro_sub_version != -1, "Did not find ALLEGRO_SUB_VERSION in the header file.");
	assert!(allegro_wip_version != -1, "Did not find ALLEGRO_WIP_VERSION in the header file.");
	assert!(allegro_release_number != -1, "Did not find ALLEGRO_RELEASE_NUMBER in the header file.");
	assert!(allegro_date != -1, "Did not find ALLEGRO_DATE in the header file.");
	assert!(!allegro_version_str.is_empty(), "Did not find ALLEGRO_VERSION_STR in the header file.");
	assert!(!allegro_date_str.is_empty(), "Did not find ALLEGRO_DATE_STR in the header file.");

    let mut versions_file = File::create(var("OUT_DIR").unwrap() + "/versions.rs").unwrap();
    versions_file.write_all(format!("
pub const ALLEGRO_VERSION: u32          = {};
pub const ALLEGRO_SUB_VERSION: u32      = {};
pub const ALLEGRO_WIP_VERSION: u32      = {};
pub const ALLEGRO_RELEASE_NUMBER: u32   = {};

pub const ALLEGRO_VERSION_STR: &'static str = {};
pub const ALLEGRO_DATE_STR: &'static str    = {};
pub const ALLEGRO_DATE: u32                 = {};
    ", allegro_version, allegro_sub_version, allegro_wip_version, allegro_release_number, allegro_version_str, allegro_date_str, allegro_date).as_bytes()).unwrap();

	for v in 0..allegro_wip_version + 1
	{
		println!("cargo:rustc-cfg=allegro_5_{}_{}", allegro_sub_version, v)
	}
	println!("cargo:sub_version={}", allegro_sub_version);
	println!("cargo:wip_version={}", allegro_wip_version);

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
	println!("cargo:rustc-flags=-l allegro{}{}", debug, static_);
	if let Ok(link_dir) = var("ALLEGRO_LINK_DIR")
	{
		println!("cargo:rustc-flags=-L {}", link_dir);
	}
}
