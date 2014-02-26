#[crate_id="allegro_image#5.0.10.1"];

#[comment = "Allegro 5 image addon Rust bindings"];
#[license = "zlib"];
#[crate_type = "lib"];
#[feature(globs)];
#[feature(macro_rules)];
#[feature(struct_variant)];

extern crate allegro = "allegro5#5.0.10.1";

use allegro::Core;
use ffi::allegro_image::*;

pub mod ffi
{
	pub use self::allegro_image::*;
	pub mod allegro_image
	{
		use std::libc::*;
		use allegro::c_bool;

		#[link(name = "allegro_image")]
		extern "C" {
			pub fn al_init_image_addon() -> c_bool;
			pub fn al_shutdown_image_addon();
			pub fn al_get_allegro_image_version() -> uint32_t;
		}
	}
}

#[macro_escape]
#[path = "../macros.rs"]
pub mod macros;

static mut initialized: bool = false;

pub struct ImageAddon
{
	priv dummy: ()
}

impl ImageAddon
{
	/* FIXME: Make this thread-safe */
	pub fn init(_: &Core) -> Option<ImageAddon>
	{
		unsafe
		{
			if initialized
			{
				None
			}
			else
			{
				if al_init_image_addon() != 0
				{
					initialized = true;
					Some(ImageAddon{ dummy: () })
				}
				else
				{
					None
				}
			}
		}
	}

	pub fn get_version() -> i32
	{
		unsafe
		{
			al_get_allegro_image_version() as i32
		}
	}
}
