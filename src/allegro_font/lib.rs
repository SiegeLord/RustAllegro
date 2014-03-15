#[crate_id="allegro_font#5.0.10.1"];

#[comment = "Allegro 5 font addon Rust bindings"];
#[license = "zlib"];
#[crate_type = "lib"];
#[feature(globs)];
#[feature(macro_rules)];
#[feature(struct_variant)];

extern crate allegro = "allegro5#5.0.10.1";

use allegro::Core;
use ffi::allegro_font::*;

#[link(name = "allegro_font")]
extern "C" {}

pub mod ffi;

#[macro_escape]
#[path = "../macros.rs"]
pub mod macros;

static mut initialized: bool = false;

pub struct FontAddon
{
	priv dummy: ()
}

impl FontAddon
{
	/* FIXME: Make this thread-safe */
	pub fn init(_: &Core) -> Option<FontAddon>
	{
		unsafe
		{
			if initialized
			{
				None
			}
			else
			{
				initialized = true;
				Some(FontAddon{ dummy: () })
			}
		}
	}

	pub fn get_version() -> i32
	{
		unsafe
		{
			al_get_allegro_font_version() as i32
		}
	}
}
