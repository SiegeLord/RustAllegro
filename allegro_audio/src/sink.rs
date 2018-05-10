// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use addon::AudioAddon;
use allegro::c_bool;
use allegro_audio_sys::*;
use internal::HasMixer;

use libc::*;
use mixer::{Mixer, MixerLike};
use properties::*;

macro_rules! set_impl
{
	($self_: ident, $c_func: ident, $($var: expr),+) =>
	{
		unsafe{ if $c_func($self_.allegro_voice, $($var),+) != 0 { Ok(()) } else { Err(()) } }
	}
}

macro_rules! get_impl {
	($self_:ident, $c_func:ident, $dest_ty:ty) => {
		unsafe { $c_func($self_.allegro_voice as *const _) as $dest_ty }
	};
}

macro_rules! get_conv_impl {
	($self_:ident, $c_func:ident, $conv:path) => {
		unsafe { $conv($c_func($self_.allegro_voice as *const _)) }
	};
}

macro_rules! get_bool_impl {
	($self_:ident, $c_func:ident) => {
		unsafe { $c_func($self_.allegro_voice as *const _) != 0 }
	};
}

pub struct Sink
{
	allegro_voice: *mut ALLEGRO_VOICE,
	mixer: Mixer,
}

impl Sink
{
	pub fn new(addon: &AudioAddon) -> Result<Sink, String>
	{
		Sink::new_custom(
			addon,
			44100,
			AudioDepth::I16,
			ChannelConf::Conf2,
			AudioDepth::F32,
			ChannelConf::Conf2,
		)
	}

	pub fn new_custom(
		addon: &AudioAddon, frequency: u32, voice_depth: AudioDepth, voice_chan_conf: ChannelConf, mixer_depth: AudioDepth,
		mixer_chan_conf: ChannelConf,
	) -> Result<Sink, String>
	{
		Mixer::new_custom(addon, frequency, mixer_depth, mixer_chan_conf)
			.map_err(|_| "Could not create the mixer.".to_string())
			.and_then(|mixer| Sink::new_with_mixer(frequency, voice_depth, voice_chan_conf, mixer))
	}

	pub fn new_with_mixer(frequency: u32, voice_depth: AudioDepth, voice_chan_conf: ChannelConf, mixer: Mixer) -> Result<Sink, String>
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
				Ok(Sink {
					allegro_voice: voice,
					mixer: mixer,
				})
			}
			else
			{
				unsafe {
					al_destroy_voice(voice);
				}
				Err("Could not attach mixer to voice".to_string())
			}
		}
	}

	pub fn get_allegro_voice(&self) -> *mut ALLEGRO_VOICE
	{
		self.allegro_voice
	}

	pub fn get_voice_frequency(&self) -> u32
	{
		get_impl!(self, al_get_voice_frequency, u32)
	}

	pub fn get_voice_position(&self) -> u32
	{
		get_impl!(self, al_get_voice_position, u32)
	}

	pub fn get_voice_channels(&self) -> ChannelConf
	{
		get_conv_impl!(self, al_get_voice_channels, ChannelConf::from_allegro)
	}

	pub fn get_voice_depth(&self) -> AudioDepth
	{
		get_conv_impl!(self, al_get_voice_depth, AudioDepth::from_allegro)
	}

	pub fn get_voice_playing(&self) -> bool
	{
		get_bool_impl!(self, al_get_voice_playing)
	}

	pub fn set_voice_playing(&self, playing: bool) -> Result<(), ()>
	{
		set_impl!(self, al_set_voice_playing, playing as c_bool)
	}

	pub fn set_voice_position(&self, pos: u32) -> Result<(), ()>
	{
		set_impl!(self, al_set_voice_position, pos as c_uint)
	}
}

impl Drop for Sink
{
	fn drop(&mut self)
	{
		unsafe {
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
