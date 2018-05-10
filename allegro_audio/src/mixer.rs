// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use addon::AudioAddon;
use allegro::c_bool;

use allegro_audio_sys::*;
use internal::AttachToMixerImpl;
use internal::Connection;
use internal::HasMixer;

use libc::*;
use properties::*;
use sample::{Sample, SampleInstance};
use std::mem;
use std::ptr;

macro_rules! set_impl
{
	($self_: ident, $c_func: ident, $($var: expr),+) =>
	{
		unsafe{ if $c_func($self_.get_mixer().allegro_mixer, $($var),+) != 0 { Ok(()) } else { Err(()) } }
	}
}

macro_rules! get_impl {
	($self_:ident, $c_func:ident, $dest_ty:ty) => {
		unsafe { $c_func($self_.get_mixer().allegro_mixer as *const _) as $dest_ty }
	};
}

macro_rules! get_conv_impl {
	($self_:ident, $c_func:ident, $conv:path) => {
		unsafe { $conv($c_func($self_.get_mixer().allegro_mixer as *const _)) }
	};
}

macro_rules! get_bool_impl {
	($self_:ident, $c_func:ident) => {
		unsafe { $c_func($self_.get_mixer().allegro_mixer as *const _) != 0 }
	};
}

pub trait AttachToMixer: AttachToMixerImpl
{
	fn detach(&mut self);

	fn attach<T: HasMixer>(&mut self, mixer: &mut T) -> Result<(), ()>
	{
		self.detach();

		let m = mixer.get_mixer_mut();
		self.create_connection(m.allegro_mixer).map(|conn| {
			m.children.push(conn);
		})
	}
}

struct CallbackHolder
{
	cb: Box<PostProcessCallback + Send>,
	sample_size: usize,
}

pub struct Mixer
{
	allegro_mixer: *mut ALLEGRO_MIXER,
	parent: Option<Connection>,
	children: Vec<Connection>,
	callback: Option<Box<CallbackHolder>>,
}

impl Mixer
{
	pub fn new(addon: &AudioAddon) -> Result<Mixer, ()>
	{
		Mixer::new_custom(addon, 44100, AudioDepth::F32, ChannelConf::Conf2)
	}

	pub fn new_custom(_: &AudioAddon, frequency: u32, depth: AudioDepth, chan_conf: ChannelConf) -> Result<Mixer, ()>
	{
		let mixer = unsafe { al_create_mixer(frequency as c_uint, depth.get(), chan_conf.get()) };
		if mixer.is_null()
		{
			Err(())
		}
		else
		{
			Ok(Mixer {
				allegro_mixer: mixer,
				parent: None,
				children: Vec::new(),
				callback: None,
			})
		}
	}

	pub fn get_allegro_mixer(&self) -> *mut ALLEGRO_MIXER
	{
		self.allegro_mixer
	}

	fn detach(allegro_mixer: *mut c_void)
	{
		unsafe {
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
		unsafe {
			al_destroy_mixer(self.allegro_mixer);
		}
	}
}

pub trait MixerLike: HasMixer
{
	fn get_allegro_mixer(&self) -> *mut ALLEGRO_MIXER
	{
		self.get_mixer().allegro_mixer
	}

	fn play_sample(&mut self, sample: &Sample, gain: f32, pan: Option<f32>, speed: f32, playmode: Playmode) -> Result<SampleInstance, ()>
	{
		let inst = sample.create_instance();
		inst.and_then(|mut inst| {
			let m = self.get_mixer_mut();
			if_ok!(inst.attach(m));
			if_ok!(inst.set_gain(gain));
			if_ok!(inst.set_pan(pan));
			if_ok!(inst.set_speed(speed));
			if_ok!(inst.set_playmode(playmode));
			if_ok!(inst.set_playing(true));
			Ok(inst)
		})
	}

	fn get_frequency(&self) -> u32
	{
		get_impl!(self, al_get_mixer_frequency, u32)
	}

	fn get_gain(&self) -> f32
	{
		get_impl!(self, al_get_mixer_gain, f32)
	}

	fn get_quality(&self) -> MixerQuality
	{
		get_conv_impl!(self, al_get_mixer_quality, MixerQuality::from_allegro)
	}

	fn get_channels(&self) -> ChannelConf
	{
		get_conv_impl!(self, al_get_mixer_channels, ChannelConf::from_allegro)
	}

	fn get_depth(&self) -> AudioDepth
	{
		get_conv_impl!(self, al_get_mixer_depth, AudioDepth::from_allegro)
	}

	fn get_playing(&self) -> bool
	{
		get_bool_impl!(self, al_get_mixer_playing)
	}

	fn get_attached(&self) -> bool
	{
		get_bool_impl!(self, al_get_mixer_attached)
	}

	fn set_playing(&self, playing: bool) -> Result<(), ()>
	{
		set_impl!(self, al_set_mixer_playing, playing as c_bool)
	}

	fn set_gain(&self, gain: f32) -> Result<(), ()>
	{
		set_impl!(self, al_set_mixer_gain, gain as c_float)
	}

	fn set_frequency(&self, freq: u32) -> Result<(), ()>
	{
		set_impl!(self, al_set_mixer_frequency, freq as u32)
	}

	fn set_quality(&self, quality: MixerQuality) -> Result<(), ()>
	{
		set_impl!(self, al_set_mixer_quality, quality.get())
	}

	fn set_postprocess_callback(&mut self, cb: Option<Box<PostProcessCallback + Send>>) -> Result<(), ()>
	{
		let allegro_mixer = self.get_mixer().allegro_mixer;

		match cb
		{
			Some(cb) =>
			{
				let mut cbh = Box::new(CallbackHolder {
					cb: cb,
					sample_size: self.get_channels().get_num_channels() * self.get_depth().get_byte_size(),
				});
				let ret = unsafe {
					al_set_mixer_postprocess_callback(
						allegro_mixer,
						Some(mixer_callback as extern "C" fn(*mut c_void, c_uint, *mut c_void)),
						&mut *cbh as *mut _ as *mut _,
					)
				};
				if ret == 0
				{
					return Err(());
				}
				self.get_mixer_mut().callback = Some(cbh);
			}
			None =>
			{
				let ret = unsafe { al_set_mixer_postprocess_callback(allegro_mixer, None, ptr::null_mut()) };
				if ret == 0
				{
					return Err(());
				}
				self.get_mixer_mut().callback = None;
			}
		}
		Ok(())
	}
}

extern "C" fn mixer_callback(data: *mut c_void, num_samples: c_uint, cb: *mut c_void)
{
	use std::slice::from_raw_parts_mut;
	unsafe {
		let cbh: &mut CallbackHolder = mem::transmute(cb);
		let buf = from_raw_parts_mut(data as *mut u8, num_samples as usize * cbh.sample_size);
		cbh.cb.process(buf, num_samples as u32);
	}
}

pub trait PostProcessCallback
{
	fn process(&mut self, data: &mut [u8], num_samples: u32);
}

impl AttachToMixerImpl for Mixer
{
	fn create_connection(&mut self, allegro_mixer: *mut ALLEGRO_MIXER) -> Result<Connection, ()>
	{
		if self.allegro_mixer == allegro_mixer
		{
			Err(())
		}
		else if unsafe { al_attach_mixer_to_mixer(self.allegro_mixer, allegro_mixer) == 0 }
		{
			Err(())
		}
		else
		{
			let (c1, c2) = Connection::new(unsafe { mem::transmute(self.allegro_mixer) }, Mixer::detach);
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
