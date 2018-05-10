// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use allegro_audio_sys::ALLEGRO_MIXER;
use libc::c_void;
use mixer::Mixer;

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::Arc;

#[doc(hidden)]
pub trait AttachToMixerImpl
{
	fn create_connection(&mut self, allegro_mixer: *mut ALLEGRO_MIXER) -> Result<Connection, ()>;
}

#[doc(hidden)]
pub trait HasMixer
{
	fn get_mixer<'l>(&'l self) -> &'l Mixer;
	fn get_mixer_mut<'l>(&'l mut self) -> &'l mut Mixer;
}

// When a connection is broken, the callback is called on the payload
pub struct Connection
{
	active: Arc<AtomicBool>,
	payload: *mut c_void,
	callback: fn(*mut c_void),
}

impl Connection
{
	pub fn new(payload: *mut c_void, callback: fn(*mut c_void)) -> (Connection, Connection)
	{
		let active1 = Arc::new(AtomicBool::new(true));
		let active2 = active1.clone();
		(
			Connection {
				active: active1,
				payload: payload,
				callback: callback,
			},
			Connection {
				active: active2,
				payload: payload,
				callback: callback,
			},
		)
	}
}

impl Drop for Connection
{
	fn drop(&mut self)
	{
		if self.active.swap(false, SeqCst)
		{
			(self.callback)(self.payload);
		}
	}
}
