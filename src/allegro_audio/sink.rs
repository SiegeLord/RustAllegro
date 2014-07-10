// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;
use std::option::Some;
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
	       mixer: Mixer) -> Option<Sink>
	{
		let voice = unsafe { al_create_voice(frequency as c_uint, voice_depth.get(), voice_chan_conf.get()) };
		if voice.is_null()
		{
			unsafe
			{
				println!("Voice is null {}", al_is_audio_installed());
			}
			None
		}
		else
		{
			if unsafe { al_attach_mixer_to_voice(mixer.get_allegro_mixer(), voice) != 0 }
			{
				Some(Sink{ allegro_voice: voice, mixer: mixer })
			}
			else
			{
				unsafe { al_destroy_voice(voice); }
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
	pub fn create_sink(&self) -> Option<Sink>
	{
		self.create_custom_sink(44100, AudioDepthI16, ChannelConf2, AudioDepthF32, ChannelConf2)
	}

	pub fn create_custom_sink(&self, frequency: u32, voice_depth: AudioDepth, voice_chan_conf: ChannelConf,
	                       mixer_depth: AudioDepth, mixer_chan_conf: ChannelConf) -> Option<Sink>
	{
		match self.create_custom_mixer(frequency, mixer_depth, mixer_chan_conf)
		{
			Some(mixer) => Sink::new(frequency, voice_depth, voice_chan_conf, mixer),
			_ => None
		}
	}
}
