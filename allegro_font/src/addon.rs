// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![allow(non_upper_case_globals)]

use std::cell::RefCell;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

use allegro::Core;
use allegro_font_sys::*;

static mut initialized: bool = false;
thread_local!(static spawned_on_this_thread: RefCell<bool> = RefCell::new(false));

pub struct FontAddon
{
	core_mutex: Arc<Mutex<()>>,
	no_send_marker: PhantomData<*mut u8>,
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
				if spawned_on_this_thread.with(|x| *x.borrow())
				{
					Err("The font addon has already been created in this task.".to_string())
				}
				else
				{
					spawned_on_this_thread.with(|x| *x.borrow_mut() = true);
					Ok(FontAddon{ core_mutex: core.get_core_mutex(), no_send_marker: PhantomData })
				}
			}
			else
			{
				al_init_font_addon();
				initialized = true;
				spawned_on_this_thread.with(|x| *x.borrow_mut() = true);
				Ok(FontAddon{ core_mutex: core.get_core_mutex(), no_send_marker: PhantomData })
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
