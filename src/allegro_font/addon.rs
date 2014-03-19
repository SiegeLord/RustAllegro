use allegro::Core;
use ffi::allegro_font::*;

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
