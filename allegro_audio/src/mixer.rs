// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;
use std::option::Some;
use std::mem;

use ffi::*;
use internal::Connection;
use internal::HasMixer;
use internal::AttachToMixerImpl;
use properties::*;
use sample::{Sample, SampleInstance};

pub trait AttachToMixer : AttachToMixerImpl
{
	fn detach(&mut self);

	fn attach<T: HasMixer>(&mut self, mixer: &mut T) -> Result<(), ()>
	{
		self.detach();

		let m = mixer.get_mixer_mut();
		self.create_connection(m.allegro_mixer).map(|conn|
		{
			m.children.push(conn);
		})
	}
}

pub struct Mixer
{
	allegro_mixer: *mut ALLEGRO_MIXER,
	parent: Option<Connection>,
	children: Vec<Connection>
}

impl Mixer
{
	fn new(frequency: u32, depth: AudioDepth, chan_conf: ChannelConf) -> Result<Mixer, ()>
	{
		let mixer = unsafe { al_create_mixer(frequency as c_uint, depth.get(), chan_conf.get()) };
		if mixer.is_null()
		{
			Err(())
		}
		else
		{
			Ok(Mixer
			{
				allegro_mixer: mixer,
				parent: None,
				children: Vec::new(),
			})
		}
	}

	pub fn get_allegro_mixer(&self) -> *mut ALLEGRO_MIXER
	{
		self.allegro_mixer
	}

	fn detach(allegro_mixer: *mut c_void)
	{
		unsafe
		{
			al_detach_mixer(mem::transmute(allegro_mixer));
		}
	}
}

impl Drop for Mixer
{
	fn drop(&mut self)
	{
		self.detach();
		self.children.clear();
		unsafe
		{
			al_destroy_mixer(self.allegro_mixer);
		}
	}
}

pub trait MixerLike : HasMixer
{
	fn play_sample(&mut self, sample: &Sample, gain: f32, pan: Option<f32>, speed: f32, playmode: Playmode) -> Result<SampleInstance, ()>
	{
		let inst = sample.create_instance();
		inst.and_then(|mut inst|
		{
			let m = self.get_mixer_mut();
			if_ok!(inst.attach(m))
			if_ok!(inst.set_gain(gain))
			if_ok!(inst.set_pan(pan))
			if_ok!(inst.set_speed(speed))
			if_ok!(inst.set_playmode(playmode))
			if_ok!(inst.set_playing(true))
			Ok(inst)
		})
	}
}

impl AttachToMixerImpl for Mixer
{
	fn create_connection(&mut self, allegro_mixer: *mut ALLEGRO_MIXER) -> Result<Connection, ()>
	{
		if self.allegro_mixer == allegro_mixer
		{
			Err(())
		}
		else if unsafe{ al_attach_mixer_to_mixer(self.allegro_mixer, allegro_mixer) == 0 }
		{
			Err(())
		}
		else
		{
			let (c1, c2) = Connection::new(unsafe{ mem::transmute(self.allegro_mixer) }, Mixer::detach);
			self.parent = Some(c1);
			Ok(c2)
		}
	}
}

impl AttachToMixer for Mixer
{
	fn detach(&mut self)
	{
		self.parent = None;
	}
}

impl HasMixer for Mixer
{
	fn get_mixer<'l>(&'l self) -> &'l Mixer
	{
		self
	}

	fn get_mixer_mut<'l>(&'l mut self) -> &'l mut Mixer
	{
		self
	}
}

impl MixerLike for Mixer {}

impl ::addon::AudioAddon
{
	pub fn create_mixer(&self) -> Result<Mixer, ()>
	{
		self.create_custom_mixer(44100, AudioDepthF32, ChannelConf2)
	}

	pub fn create_custom_mixer(&self, frequency: u32, depth: AudioDepth, chan_conf: ChannelConf) -> Result<Mixer, ()>
	{
		Mixer::new(frequency, depth, chan_conf)
	}
}
