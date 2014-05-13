// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;
use std::mem;
use std::option::Some;

use mixer::AttachToMixer;
use ffi::*;
use internal::{Connection, AttachToMixerImpl};

pub struct AudioStream
{
	parent: Option<Connection>,
	allegro_audio_stream: *mut ALLEGRO_AUDIO_STREAM,
}

impl AudioStream
{
	fn load(filename: &str, buffer_count: uint, samples: u32) -> Option<AudioStream>
	{
		let stream = filename.with_c_str(|s|
		{
			unsafe
			{
				al_load_audio_stream(s, buffer_count as size_t, samples as c_uint)
			}
		});
		if stream.is_null()
		{
			None
		}
		else
		{
			Some(AudioStream
			{
				parent: None,
				allegro_audio_stream: stream,
			})
		}
	}

	fn detach(allegro_audio_stream: *mut c_void)
	{
		unsafe
		{
			al_detach_audio_stream(mem::transmute(allegro_audio_stream));
		}
	}
}

impl Drop for AudioStream
{
	fn drop(&mut self)
	{
		self.detach();
		unsafe
		{
			al_destroy_audio_stream(self.allegro_audio_stream);
		}
	}
}

impl AttachToMixerImpl for AudioStream
{
	fn create_connection(&mut self, allegro_mixer: *mut ALLEGRO_MIXER) -> Option<Connection>
	{
		if unsafe{ al_attach_audio_stream_to_mixer(self.allegro_audio_stream, allegro_mixer) == 0 }
		{
			None
		}
		else
		{
			let (c1, c2) = Connection::new(unsafe{ mem::transmute(self.allegro_audio_stream) }, AudioStream::detach);
			self.parent = Some(c1);
			Some(c2)
		}
	}
}

impl AttachToMixer for AudioStream
{
	fn detach(&mut self)
	{
		self.parent = None;
	}
}

impl ::addon::AudioAddon
{
	pub fn load_audio_stream(&self, filename: &str) -> Option<AudioStream>
	{
		self.load_custom_audio_stream(filename, 4, 2048)
	}

	pub fn load_custom_audio_stream(&self, filename: &str, buffer_count: uint, samples: u32) -> Option<AudioStream>
	{
		AudioStream::load(filename, buffer_count, samples)
	}
}
