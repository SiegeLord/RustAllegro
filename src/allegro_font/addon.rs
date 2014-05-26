// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use std::kinds::marker::NoSend;

use sync::{Arc, Mutex};

use allegro::Core;
use ffi::allegro_font::*;

static mut initialized: bool = false;
//#[thread_local]
static mut spawned_on_this_thread: bool = false;

pub struct FontAddon
{
	no_send_marker: NoSend,
	core_mutex: Arc<Mutex<()>>,
}

impl FontAddon
{
	pub fn init(core: &Core) -> Option<FontAddon>
	{
		let mutex = core.get_core_mutex();
		let _guard = mutex.lock();
		unsafe
		{
			if initialized
			{
				if spawned_on_this_thread
				{
					None
				}
				else
				{
					spawned_on_this_thread = true;
					Some(FontAddon{ no_send_marker: NoSend, core_mutex: core.get_core_mutex() })
				}
			}
			else
			{
				al_init_font_addon();
				initialized = true;
				spawned_on_this_thread = true;
				Some(FontAddon{ no_send_marker: NoSend, core_mutex: core.get_core_mutex() })
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
