// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;
use std::option::Some;
use ffi::*;
use properties::*;

pub struct Sink
{
	allegro_voice: *mut ALLEGRO_VOICE,
	allegro_mixer: *mut ALLEGRO_MIXER,
}

impl Sink
{
	fn new(frequency: u32, voice_depth: AudioDepth, voice_chan_conf: ChannelConf,
	       mixer_depth: AudioDepth, mixer_chan_conf: ChannelConf) -> Option<Sink>
	{
		let (voice, mixer) = unsafe
		{
			(al_create_voice(frequency as c_uint, voice_depth.get(), voice_chan_conf.get()),
			 al_create_mixer(frequency as c_uint, mixer_depth.get(), mixer_chan_conf.get()))
		};
		if voice.is_null() || mixer.is_null()
		{
			unsafe
			{
				al_destroy_voice(voice);
				al_destroy_mixer(mixer);
			}
			None
		}
		else
		{
			if unsafe { al_attach_mixer_to_voice(mixer, voice) != 0 }
			{
				Some(Sink{ allegro_voice: voice, allegro_mixer: mixer })
			}
			else
			{
				unsafe
				{
					al_destroy_voice(voice);
					al_destroy_mixer(mixer);
				}
				None
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
			al_detach_mixer(self.allegro_mixer);
			al_destroy_voice(self.allegro_voice);
			al_destroy_mixer(self.allegro_mixer);
		}
	}
}

impl ::addon::AudioAddon
{
	pub fn new_sink(&self) -> Option<Sink>
	{
		self.new_custom_sink(44100, AudioDepthI16, ChannelConf2, AudioDepthF32, ChannelConf2)
	}

	pub fn new_custom_sink(&self, frequency: u32, voice_depth: AudioDepth, voice_chan_conf: ChannelConf,
	                       mixer_depth: AudioDepth, mixer_chan_conf: ChannelConf) -> Option<Sink>
	{
		Sink::new(frequency, voice_depth, voice_chan_conf, mixer_depth, mixer_chan_conf)
	}
}
