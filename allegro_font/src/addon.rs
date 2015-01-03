// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use std::kinds::marker::NoSend;

use std::sync::{Arc, Mutex};

use allegro::Core;
use allegro_font_sys::*;

static mut initialized: bool = false;
#[thread_local]
static mut spawned_on_this_thread: bool = false;

pub struct FontAddon
{
	no_send_marker: NoSend,
	core_mutex: Arc<Mutex<()>>,
}

impl FontAddon
{
	pub fn init(core: &Core) -> Result<FontAddon, String>
	{
		let mutex = core.get_core_mutex();
		let _guard = mutex.lock();
		unsafe
		{
			if initialized
			{
				if spawned_on_this_thread
				{
					Err("The font addon has already been created in this task.".to_string())
				}
				else
				{
					spawned_on_this_thread = true;
					Ok(FontAddon{ no_send_marker: NoSend, core_mutex: core.get_core_mutex() })
				}
			}
			else
			{
				al_init_font_addon();
				initialized = true;
				spawned_on_this_thread = true;
				Ok(FontAddon{ no_send_marker: NoSend, core_mutex: core.get_core_mutex() })
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

	pub fn get_core_mutex(&self) -> Arc<Mutex<()>>
	{
		self.core_mutex.clone()
	}
}
