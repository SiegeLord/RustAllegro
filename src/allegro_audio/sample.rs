// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use allegro::c_bool;

use libc::*;
use sync::Arc;
use std::cast;
use std::ptr;
use std::option::Some;
use std::sync::atomics::{AtomicBool, SeqCst};

use mixer::AttachToMixer;
use ffi::*;
use internal::{Connection, AttachToMixerImpl};
use properties::Playmode;

// TODO: ALLEGRO_SAMPLE and ALLEGRO_SAMPLE_INSTANCE can probably race on each other...
// consider adding mutexes (maybe Allegro's mutexes prevent everything bad already)

pub struct Sample
{
	allegro_sample: *mut ALLEGRO_SAMPLE,
	sample_valid: Arc<AtomicBool>,
}

impl Sample
{
	fn load(filename: &str) -> Option<Sample>
	{
		let samp = filename.with_c_str(|s|
		{
			unsafe
			{
				al_load_sample(s)
			}
		});
		if samp.is_null()
		{
			None
		}
		else
		{
			Some(Sample
			{
				allegro_sample: samp,
				sample_valid: Arc::new(AtomicBool::new(true))
			})
		}
	}

	pub fn create_instance(&self) -> Option<SampleInstance>
	{
		let mut inst = SampleInstance::new();
		inst.as_mut().map(|inst|
		{
			inst.set_sample(self);
		});
		inst
	}
}

impl Drop for Sample
{
	fn drop(&mut self)
	{
		self.sample_valid.swap(false, SeqCst);
		unsafe
		{
			al_destroy_sample(self.allegro_sample);
		}
	}
}

pub struct SampleInstance
{
	parent: Option<Connection>,
	// I think when the sample is invalid, it is unsafe to resume it
	sample_valid: Arc<AtomicBool>,
	allegro_sample_instance: *mut ALLEGRO_SAMPLE_INSTANCE,
}

impl SampleInstance
{
	fn new() -> Option<SampleInstance>
	{
		let inst = unsafe { al_create_sample_instance(ptr::mut_null()) };
		if inst.is_null()
		{
			None
		}
		else
		{
			Some(SampleInstance
			{
				parent: None,
				sample_valid: Arc::new(AtomicBool::new(false)),
				allegro_sample_instance: inst
			})
		}
	}

	fn detach(allegro_sample_instance: *mut c_void)
	{
		unsafe
		{
			al_detach_sample_instance(cast::transmute(allegro_sample_instance));
		}
	}

	pub fn set_sample(&mut self, sample: &Sample) -> bool
	{
		if unsafe { al_set_sample(self.allegro_sample_instance, sample.allegro_sample) != 0 }
		{
			self.sample_valid = sample.sample_valid.clone();
			true
		}
		else
		{
			self.sample_valid = Arc::new(AtomicBool::new(false));
			// As per docs of al_set_sample
			self.parent = None;
			false
		}
	}

	pub fn set_playing(&self, playing: bool) -> bool
	{
		if self.sample_valid.load(SeqCst)
		{
			unsafe
			{
				al_set_sample_instance_playing(self.allegro_sample_instance, playing as c_bool) != 0
			}
		}
		else
		{
			false
		}
	}

	pub fn set_gain(&self, gain: f32) -> bool
	{
		unsafe
		{
			al_set_sample_instance_gain(self.allegro_sample_instance, gain as c_float) != 0
		}
	}

	pub fn set_pan(&self, pan: Option<f32>) -> bool
	{
		unsafe
		{
			let pan = match pan
			{
				Some(p) => p as c_float,
				None => ALLEGRO_AUDIO_PAN_NONE
			};

			al_set_sample_instance_pan(self.allegro_sample_instance, pan) != 0
		}
	}

	pub fn set_speed(&self, speed: f32) -> bool
	{
		unsafe
		{
			al_set_sample_instance_speed(self.allegro_sample_instance, speed as c_float) != 0
		}
	}

	pub fn set_playmode(&self, playmode: Playmode) -> bool
	{
		unsafe
		{
			al_set_sample_instance_playmode(self.allegro_sample_instance, playmode.get()) != 0
		}
	}
}

impl Drop for SampleInstance
{
	fn drop(&mut self)
	{
		self.detach();
		unsafe
		{
			al_destroy_sample_instance(self.allegro_sample_instance);
		}
	}
}

impl AttachToMixerImpl for SampleInstance
{
	fn create_connection(&mut self, allegro_mixer: *mut ALLEGRO_MIXER) -> Option<Connection>
	{
		if unsafe{ al_attach_sample_instance_to_mixer(self.allegro_sample_instance, allegro_mixer) == 0 }
		{
			None
		}
		else
		{
			let (c1, c2) = Connection::new(unsafe{ cast::transmute(self.allegro_sample_instance) }, SampleInstance::detach);
			self.parent = Some(c1);
			Some(c2)
		}
	}
}

impl AttachToMixer for SampleInstance
{
	fn detach(&mut self)
	{
		self.parent = None;
	}
}

impl ::addon::AudioAddon
{
	pub fn create_sample_instance(&self) -> Option<SampleInstance>
	{
		SampleInstance::new()
	}

	pub fn load_sample(&self, filename: &str) -> Option<Sample>
	{
		Sample::load(filename)
	}
}
