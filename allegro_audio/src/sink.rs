// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;
use ffi::*;
use properties::*;
use mixer::{MixerLike, Mixer};
use internal::HasMixer;

pub struct Sink
{
	allegro_voice: *mut ALLEGRO_VOICE,
	mixer: Mixer
}

impl Sink
{
	fn new(frequency: u32, voice_depth: AudioDepth, voice_chan_conf: ChannelConf,
	       mixer: Mixer) -> Result<Sink, String>
	{
		let voice = unsafe { al_create_voice(frequency as c_uint, voice_depth.get(), voice_chan_conf.get()) };
		if voice.is_null()
		{
			Err("Could not create the voice".to_string())
		}
		else
		{
			if unsafe { al_attach_mixer_to_voice(mixer.get_allegro_mixer(), voice) != 0 }
			{
				Ok(Sink{ allegro_voice: voice, mixer: mixer })
			}
			else
			{
				unsafe { al_destroy_voice(voice); }
				Err("Could not attach mixer to voice".to_string())
			}
		}
	}
}

impl Drop for Sink
{
	fn drop(&mut self)
	{
		unsafe
		{
			al_detach_mixer(self.mixer.get_allegro_mixer());
			al_destroy_voice(self.allegro_voice);
		}
	}
}

impl HasMixer for Sink
{
	fn get_mixer<'l>(&'l self) -> &'l Mixer
	{
		&self.mixer
	}

	fn get_mixer_mut<'l>(&'l mut self) -> &'l mut Mixer
	{
		&mut self.mixer
	}
}

impl MixerLike for Sink {}

impl ::addon::AudioAddon
{
	pub fn create_sink(&self) -> Result<Sink, String>
	{
		self.create_custom_sink(44100, AudioDepthI16, ChannelConf2, AudioDepthF32, ChannelConf2)
	}

	pub fn create_custom_sink(&self, frequency: u32, voice_depth: AudioDepth, voice_chan_conf: ChannelConf,
	                       mixer_depth: AudioDepth, mixer_chan_conf: ChannelConf) -> Result<Sink, String>
	{
		self.create_custom_mixer(frequency, mixer_depth, mixer_chan_conf)
		.map_err(|_| "Could not create the mixer.".to_string())
		.and_then(|mixer|
			Sink::new(frequency, voice_depth, voice_chan_conf, mixer)
		)
	}
}
