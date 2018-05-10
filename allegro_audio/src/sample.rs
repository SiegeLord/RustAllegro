// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use addon::AudioAddon;
use allegro::c_bool;
use allegro_audio_sys::*;
use internal::{AttachToMixerImpl, Connection};

use libc::*;

use mixer::AttachToMixer;
use properties::*;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::slice::{from_raw_parts, from_raw_parts_mut};
use std::sync::Arc;
use std::sync::Mutex;

pub trait DataSample
where
	Self: Sized,
{
	fn get_depth(_: Option<Self>) -> AudioDepth;
}

macro_rules! data_sample_impl {
	($t:ty, $d:path) => {
		impl DataSample for $t
		{
			fn get_depth(_: Option<$t>) -> AudioDepth
			{
				$d
			}
		}
	};
}

data_sample_impl!(i8, AudioDepth::I8);
data_sample_impl!(i16, AudioDepth::I16);
data_sample_impl!(u8, AudioDepth::U8);
data_sample_impl!(u16, AudioDepth::U16);
data_sample_impl!(f32, AudioDepth::F32);

// TODO: ALLEGRO_SAMPLE and ALLEGRO_SAMPLE_INSTANCE can probably race on each other...
// consider adding mutexes (maybe Allegro's mutexes prevent everything bad already)

pub struct Sample
{
	allegro_sample: *mut ALLEGRO_SAMPLE,
	// This will inform sample instances that this sample got dropped
	sample_valid: Arc<Mutex<bool>>,
}

impl Sample
{
	pub fn load(_: &AudioAddon, filename: &str) -> Result<Sample, ()>
	{
		let filename = CString::new(filename.as_bytes()).unwrap();
		let samp = unsafe { al_load_sample(filename.as_ptr()) };
		if samp.is_null()
		{
			Err(())
		}
		else
		{
			Ok(Sample {
				allegro_sample: samp,
				sample_valid: Arc::new(Mutex::new(true)),
			})
		}
	}

	pub fn create_instance(&self) -> Result<SampleInstance, ()>
	{
		let inst = SampleInstance::new_raw();
		inst.and_then(|mut inst| {
			if_ok!(inst.set_sample(self));
			Ok(inst)
		})
	}

	pub fn get_frequency(&self) -> usize
	{
		unsafe { al_get_sample_frequency(self.allegro_sample as *const _) as usize }
	}

	pub fn get_length(&self) -> usize
	{
		unsafe { al_get_sample_length(self.allegro_sample as *const _) as usize }
	}

	pub fn get_byte_length(&self) -> usize
	{
		self.get_length() * self.get_channels().get_num_channels() * self.get_depth().get_byte_size()
	}

	pub fn get_depth(&self) -> AudioDepth
	{
		unsafe { AudioDepth::from_allegro(al_get_sample_depth(self.allegro_sample as *const _)) }
	}

	pub fn get_channels(&self) -> ChannelConf
	{
		unsafe { ChannelConf::from_allegro(al_get_sample_channels(self.allegro_sample as *const _)) }
	}

	pub fn get_raw_data<'l>(&'l self) -> &'l [u8]
	{
		let len = self.get_byte_length();
		unsafe { from_raw_parts(al_get_sample_data(self.allegro_sample as *const _) as *const _, len) }
	}

	pub fn get_data<'l, T: DataSample>(&'l self) -> Result<&'l [T], ()>
	{
		if self.get_depth() == DataSample::get_depth(None::<T>)
		{
			let len = self.get_byte_length() / mem::size_of::<T>();
			Ok(unsafe { from_raw_parts(al_get_sample_data(self.allegro_sample as *const _) as *const _, len) })
		}
		else
		{
			Err(())
		}
	}

	pub fn get_data_mut<'l, T: DataSample>(&'l mut self) -> Result<&'l mut [T], ()>
	{
		if self.get_depth() == DataSample::get_depth(None::<T>)
		{
			let len = self.get_byte_length() / mem::size_of::<T>();
			Ok(unsafe { from_raw_parts_mut(al_get_sample_data(self.allegro_sample as *const _) as *mut _, len) })
		}
		else
		{
			Err(())
		}
	}

	pub fn get_raw_data_mut<'l>(&'l mut self) -> &'l mut [u8]
	{
		let len = self.get_byte_length();
		unsafe { from_raw_parts_mut(al_get_sample_data(self.allegro_sample as *const _) as *mut _, len) }
	}

	pub fn get_allegro_sample(&self) -> *mut ALLEGRO_SAMPLE
	{
		self.allegro_sample
	}
}

impl Drop for Sample
{
	fn drop(&mut self)
	{
		let mut valid = self.sample_valid.lock().unwrap();
		*valid = false;
		unsafe {
			al_destroy_sample(self.allegro_sample);
		}
	}
}

pub struct SampleInstance
{
	parent: Option<Connection>,
	// I think when the sample is invalid, it is unsafe to resume it
	sample_valid: Arc<Mutex<bool>>,
	allegro_sample_instance: *mut ALLEGRO_SAMPLE_INSTANCE,
}

macro_rules! check_or_else {
	($self_:ident, $valid:expr, $invalid:expr) => {{
		let valid = $self_.sample_valid.lock().unwrap();
		if *valid
			{
			unsafe { $valid }
			}
		else
			{
			$invalid
			}
		}};
}

macro_rules! set_impl {
	($self_:ident, $c_func:ident, $var:expr) => {
		check_or_else!(
			$self_,
			if $c_func($self_.allegro_sample_instance, $var) != 0
				{
				Ok(())
				}
			else
				{
				Err(())
				},
			Err(())
			)
	};
}

macro_rules! get_opt_impl {
	($self_:ident, $c_func:ident, $dest_ty:ty) => {
		check_or_else!($self_, Ok($c_func($self_.allegro_sample_instance as *const _) as $dest_ty), Err(()))
	};
}

macro_rules! get_conv_impl {
	($self_:ident, $c_func:ident, $conv:path) => {
		check_or_else!($self_, Ok($conv($c_func($self_.allegro_sample_instance as *const _))), Err(()))
	};
}

macro_rules! get_bool_impl {
	($self_:ident, $c_func:ident) => {
		check_or_else!($self_, Ok($c_func($self_.allegro_sample_instance as *const _) != 0), Err(()))
	};
}

impl SampleInstance
{
	pub fn new(_: &AudioAddon) -> Result<SampleInstance, ()>
	{
		SampleInstance::new_raw()
	}

	fn new_raw() -> Result<SampleInstance, ()>
	{
		let inst = unsafe { al_create_sample_instance(ptr::null_mut()) };
		if inst.is_null()
		{
			Err(())
		}
		else
		{
			Ok(SampleInstance {
				parent: None,
				sample_valid: Arc::new(Mutex::new(false)),
				allegro_sample_instance: inst,
			})
		}
	}

	fn detach(allegro_sample_instance: *mut c_void)
	{
		unsafe {
			al_detach_sample_instance(mem::transmute(allegro_sample_instance));
		}
	}

	pub fn set_sample(&mut self, sample: &Sample) -> Result<(), ()>
	{
		if unsafe { al_set_sample(self.allegro_sample_instance, sample.allegro_sample) != 0 }
		{
			self.sample_valid = sample.sample_valid.clone();
			Ok(())
		}
		else
		{
			self.sample_valid = Arc::new(Mutex::new(false));
			// As per docs of al_set_sample
			self.parent = None;
			Err(())
		}
	}

	pub fn set_position(&self, position: u32) -> Result<(), ()>
	{
		set_impl!(self, al_set_sample_instance_position, position as c_uint)
	}

	pub fn set_length(&self, length: u32) -> Result<(), ()>
	{
		set_impl!(self, al_set_sample_instance_length, length as c_uint)
	}

	pub fn set_playing(&self, playing: bool) -> Result<(), ()>
	{
		set_impl!(self, al_set_sample_instance_playing, playing as c_bool)
	}

	pub fn set_gain(&self, gain: f32) -> Result<(), ()>
	{
		set_impl!(self, al_set_sample_instance_gain, gain as c_float)
	}

	pub fn set_pan(&self, pan: Option<f32>) -> Result<(), ()>
	{
		set_impl!(
			self,
			al_set_sample_instance_pan,
			match pan
			{
				Some(p) => p as c_float,
				None => ALLEGRO_AUDIO_PAN_NONE,
			}
		)
	}

	pub fn set_speed(&self, speed: f32) -> Result<(), ()>
	{
		set_impl!(self, al_set_sample_instance_speed, speed as c_float)
	}

	pub fn set_playmode(&self, playmode: Playmode) -> Result<(), ()>
	{
		set_impl!(self, al_set_sample_instance_playmode, playmode.get())
	}

	pub fn get_frequency(&self) -> Result<u32, ()>
	{
		get_opt_impl!(self, al_get_sample_instance_frequency, u32)
	}

	pub fn get_length(&self) -> Result<u32, ()>
	{
		get_opt_impl!(self, al_get_sample_instance_length, u32)
	}

	pub fn get_position(&self) -> Result<u32, ()>
	{
		get_opt_impl!(self, al_get_sample_instance_position, u32)
	}

	pub fn get_speed(&self) -> Result<f32, ()>
	{
		get_opt_impl!(self, al_get_sample_instance_speed, f32)
	}

	pub fn get_gain(&self) -> Result<f32, ()>
	{
		get_opt_impl!(self, al_get_sample_instance_gain, f32)
	}

	pub fn get_pan(&self) -> Result<f32, ()>
	{
		get_opt_impl!(self, al_get_sample_instance_pan, f32)
	}

	pub fn get_time(&self) -> Result<f32, ()>
	{
		get_opt_impl!(self, al_get_sample_instance_time, f32)
	}

	pub fn get_playmode(&self) -> Result<Playmode, ()>
	{
		get_conv_impl!(self, al_get_sample_instance_playmode, Playmode::from_allegro)
	}

	pub fn get_channels(&self) -> Result<ChannelConf, ()>
	{
		get_conv_impl!(self, al_get_sample_instance_channels, ChannelConf::from_allegro)
	}

	pub fn get_depth(&self) -> Result<AudioDepth, ()>
	{
		get_conv_impl!(self, al_get_sample_instance_depth, AudioDepth::from_allegro)
	}

	pub fn get_playing(&self) -> Result<bool, ()>
	{
		get_bool_impl!(self, al_get_sample_instance_playing)
	}

	pub fn get_attached(&self) -> Result<bool, ()>
	{
		get_bool_impl!(self, al_get_sample_instance_attached)
	}

	pub fn get_allegro_sample_instance(&self) -> *mut ALLEGRO_SAMPLE_INSTANCE
	{
		self.allegro_sample_instance
	}
}

impl Drop for SampleInstance
{
	fn drop(&mut self)
	{
		self.detach();
		unsafe {
			al_destroy_sample_instance(self.allegro_sample_instance);
		}
	}
}

impl AttachToMixerImpl for SampleInstance
{
	fn create_connection(&mut self, allegro_mixer: *mut ALLEGRO_MIXER) -> Result<Connection, ()>
	{
		if unsafe { al_attach_sample_instance_to_mixer(self.allegro_sample_instance, allegro_mixer) == 0 }
		{
			Err(())
		}
		else
		{
			let (c1, c2) = Connection::new(unsafe { mem::transmute(self.allegro_sample_instance) }, SampleInstance::detach);
			self.parent = Some(c1);
			Ok(c2)
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
