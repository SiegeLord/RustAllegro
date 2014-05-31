// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use allegro::c_bool;

use libc::*;
use sync::Arc;
use std::mem;
use std::ptr;
use std::option::Some;
use std::sync::atomics::{AtomicBool, SeqCst};

use mixer::AttachToMixer;
use ffi::*;
use internal::{Connection, AttachToMixerImpl};
use properties::{AudioDepth, ChannelConf, Playmode};

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

macro_rules! check_or_else
{
	($valid: expr, $invalid: expr) =>
	{
		if self.sample_valid.load(SeqCst)
		{
			unsafe
			{
				$valid
			}
		}
		else
		{
			$invalid
		}
	}
}

macro_rules! set_impl
{
	($c_func: ident, $var: expr) =>
	{
		check_or_else!($c_func(self.allegro_sample_instance, $var) != 0, false)
	}
}

macro_rules! get_opt_impl
{
	($c_func: ident, $dest_ty: ty) =>
	{
		check_or_else!(Some($c_func(self.allegro_sample_instance as *ALLEGRO_SAMPLE_INSTANCE) as $dest_ty), None)
	}
}

macro_rules! get_conv_impl
{
	($c_func: ident, $conv: path) =>
	{
		check_or_else!(Some($conv($c_func(self.allegro_sample_instance as *ALLEGRO_SAMPLE_INSTANCE))), None)
	}
}

macro_rules! get_bool_impl
{
	($c_func: ident) =>
	{
		check_or_else!($c_func(self.allegro_sample_instance as *ALLEGRO_SAMPLE_INSTANCE) != 0, false)
	}
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
			al_detach_sample_instance(mem::transmute(allegro_sample_instance));
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

	pub fn set_position(&self, position: u32) -> bool
	{
		set_impl!(al_set_sample_instance_position, position as c_uint)
	}

	pub fn set_length(&self, length: u32) -> bool
	{
		set_impl!(al_set_sample_instance_length, length as c_uint)
	}

	pub fn set_playing(&self, playing: bool) -> bool
	{
		set_impl!(al_set_sample_instance_playing, playing as c_bool)
	}

	pub fn set_gain(&self, gain: f32) -> bool
	{
		set_impl!(al_set_sample_instance_gain, gain as c_float)
	}

	pub fn set_pan(&self, pan: Option<f32>) -> bool
	{
		set_impl!(al_set_sample_instance_pan,
		match pan
		{
			Some(p) => p as c_float,
			None => ALLEGRO_AUDIO_PAN_NONE
		})
	}

	pub fn set_speed(&self, speed: f32) -> bool
	{
		set_impl!(al_set_sample_instance_speed, speed as c_float)
	}

	pub fn set_playmode(&self, playmode: Playmode) -> bool
	{
		set_impl!(al_set_sample_instance_playmode, playmode.get())
	}

	pub fn get_frequency(&self) -> Option<u32>
	{
		get_opt_impl!(al_get_sample_instance_frequency, u32)
	}

	pub fn get_length(&self) -> Option<u32>
	{
		get_opt_impl!(al_get_sample_instance_length, u32)
	}

	pub fn get_position(&self) -> Option<u32>
	{
		get_opt_impl!(al_get_sample_instance_position, u32)
	}

	pub fn get_speed(&self) -> Option<f32>
	{
		get_opt_impl!(al_get_sample_instance_speed, f32)
	}

	pub fn get_gain(&self) -> Option<f32>
	{
		get_opt_impl!(al_get_sample_instance_gain, f32)
	}

	pub fn get_pan(&self) -> Option<f32>
	{
		get_opt_impl!(al_get_sample_instance_pan, f32)
	}

	pub fn get_time(&self) -> Option<f32>
	{
		get_opt_impl!(al_get_sample_instance_time, f32)
	}

	pub fn get_playmode(&self) -> Option<Playmode>
	{
		get_conv_impl!(al_get_sample_instance_playmode, Playmode::from_allegro)
	}

	pub fn get_channels(&self) -> Option<ChannelConf>
	{
		get_conv_impl!(al_get_sample_instance_channels, ChannelConf::from_allegro)
	}

	pub fn get_depth(&self) -> Option<AudioDepth>
	{
		get_conv_impl!(al_get_sample_instance_depth, AudioDepth::from_allegro)
	}

	pub fn get_playing(&self) -> bool
	{
		get_bool_impl!(al_get_sample_instance_playing)
	}

	pub fn get_attached(&self) -> bool
	{
		get_bool_impl!(al_get_sample_instance_attached)
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
			let (c1, c2) = Connection::new(unsafe{ mem::transmute(self.allegro_sample_instance) }, SampleInstance::detach);
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
