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
use std::io::Write;
use std::mem;

macro_rules! set_impl
{
	($self_: ident, $c_func: ident, $($var: expr),+) =>
	{
		unsafe{ if $c_func($self_.allegro_audio_stream, $($var),+) != 0 { Ok(()) } else { Err(()) } }
	}
}

macro_rules! get_impl {
	($self_:ident, $c_func:ident, $dest_ty:ty) => {
		unsafe { $c_func($self_.allegro_audio_stream as *const _) as $dest_ty }
	};
}

macro_rules! get_conv_impl {
	($self_:ident, $c_func:ident, $conv:path) => {
		unsafe { $conv($c_func($self_.allegro_audio_stream as *const _)) }
	};
}

macro_rules! get_bool_impl {
	($self_:ident, $c_func:ident) => {
		unsafe { $c_func($self_.allegro_audio_stream as *const _) != 0 }
	};
}

pub struct AudioStream
{
	parent: Option<Connection>,
	allegro_audio_stream: *mut ALLEGRO_AUDIO_STREAM,
	fragment_samples: usize,
	created_by_load: bool,
}

impl AudioStream
{
	pub fn load(addon: &AudioAddon, filename: &str) -> Result<AudioStream, ()>
	{
		AudioStream::load_custom(addon, filename, 4, 2048)
	}

	pub fn load_custom(_: &AudioAddon, filename: &str, buffer_count: usize, samples: u32) -> Result<AudioStream, ()>
	{
		let filename = CString::new(filename.as_bytes()).unwrap();
		let stream = unsafe { al_load_audio_stream(filename.as_ptr(), buffer_count as size_t, samples as c_uint) };
		if stream.is_null()
		{
			Err(())
		}
		else
		{
			Ok(AudioStream {
				parent: None,
				allegro_audio_stream: stream,
				fragment_samples: samples as usize,
				created_by_load: true,
			})
		}
	}

	pub fn new(
		_: &AudioAddon, buffer_count: usize, samples: u32, frequency: u32, depth: AudioDepth, chan_conf: ChannelConf,
	) -> Result<AudioStream, ()>
	{
		let stream = unsafe {
			al_create_audio_stream(
				buffer_count as size_t,
				samples as c_uint,
				frequency as c_uint,
				depth.get(),
				chan_conf.get(),
			)
		};
		if stream.is_null()
		{
			Err(())
		}
		else
		{
			Ok(AudioStream {
				parent: None,
				allegro_audio_stream: stream,
				fragment_samples: samples as usize,
				created_by_load: false,
			})
		}
	}

	fn detach(allegro_audio_stream: *mut c_void)
	{
		unsafe {
			al_detach_audio_stream(mem::transmute(allegro_audio_stream));
		}
	}

	pub fn set_loop_secs(&self, start: f64, end: f64) -> Result<(), ()>
	{
		set_impl!(self, al_set_audio_stream_loop_secs, start as c_double, end as c_double)
	}

	pub fn set_playing(&self, playing: bool) -> Result<(), ()>
	{
		set_impl!(self, al_set_audio_stream_playing, playing as c_bool)
	}

	pub fn set_gain(&self, gain: f32) -> Result<(), ()>
	{
		set_impl!(self, al_set_audio_stream_gain, gain as c_float)
	}

	pub fn set_pan(&self, pan: Option<f32>) -> Result<(), ()>
	{
		set_impl!(
			self,
			al_set_audio_stream_pan,
			match pan
			{
				Some(p) => p as c_float,
				None => ALLEGRO_AUDIO_PAN_NONE,
			}
		)
	}

	pub fn set_speed(&self, speed: f32) -> Result<(), ()>
	{
		set_impl!(self, al_set_audio_stream_speed, speed as c_float)
	}

	pub fn set_playmode(&self, playmode: Playmode) -> Result<(), ()>
	{
		set_impl!(self, al_set_audio_stream_playmode, playmode.get())
	}

	pub fn seek_secs(&self, time: f64) -> Result<(), ()>
	{
		set_impl!(self, al_seek_audio_stream_secs, time as c_double)
	}

	pub fn rewind(&self) -> Result<(), ()>
	{
		unsafe {
			match al_rewind_audio_stream(self.allegro_audio_stream) != 0
			{
				true => Ok(()),
				false => Err(()),
			}
		}
	}

	pub fn drain(&self)
	{
		unsafe { al_drain_audio_stream(self.allegro_audio_stream) }
	}

	pub fn get_frequency(&self) -> u32
	{
		get_impl!(self, al_get_audio_stream_frequency, u32)
	}

	pub fn get_num_fragments(&self) -> u32
	{
		get_impl!(self, al_get_audio_stream_fragments, u32)
	}

	pub fn get_num_available_fragments(&self) -> u32
	{
		get_impl!(self, al_get_available_audio_stream_fragments, u32)
	}

	pub fn get_length_secs(&self) -> Result<f64, ()>
	{
		if self.created_by_load
		{
			Ok(unsafe { al_get_audio_stream_length_secs(self.allegro_audio_stream) as f64 })
		}
		else
		{
			Err(())
		}
	}

	pub fn get_length(&self) -> u32
	{
		get_impl!(self, al_get_audio_stream_length, u32)
	}

	pub fn get_position_secs(&self) -> Result<f64, ()>
	{
		if self.created_by_load
		{
			Ok(unsafe { al_get_audio_stream_position_secs(self.allegro_audio_stream) as f64 })
		}
		else
		{
			Err(())
		}
	}

	pub fn get_speed(&self) -> f32
	{
		get_impl!(self, al_get_audio_stream_speed, f32)
	}

	pub fn get_gain(&self) -> f32
	{
		get_impl!(self, al_get_audio_stream_gain, f32)
	}

	pub fn get_pan(&self) -> f32
	{
		get_impl!(self, al_get_audio_stream_pan, f32)
	}

	pub fn get_playmode(&self) -> Playmode
	{
		get_conv_impl!(self, al_get_audio_stream_playmode, Playmode::from_allegro)
	}

	pub fn get_channels(&self) -> ChannelConf
	{
		get_conv_impl!(self, al_get_audio_stream_channels, ChannelConf::from_allegro)
	}

	pub fn get_depth(&self) -> AudioDepth
	{
		get_conv_impl!(self, al_get_audio_stream_depth, AudioDepth::from_allegro)
	}

	pub fn get_playing(&self) -> bool
	{
		get_bool_impl!(self, al_get_audio_stream_playing)
	}

	pub fn get_attached(&self) -> bool
	{
		get_bool_impl!(self, al_get_audio_stream_attached)
	}

	pub fn write_fragment(&self, write_cb: &mut FnMut(/*writer: */ &mut Write)) -> Result<bool, ()>
	{
		use std::slice::from_raw_parts_mut;
		let fragment = unsafe { al_get_audio_stream_fragment(self.allegro_audio_stream as *const _) };
		if fragment.is_null()
		{
			return Ok(false);
		}

		let frag_size = self.get_channels().get_num_channels() * self.get_depth().get_byte_size() * self.fragment_samples;
		unsafe {
			let mut buf = from_raw_parts_mut(fragment as *mut u8, frag_size);
			{
				write_cb(&mut buf);
			}
			// Fill the rest with silence
			while buf.write(&[0]).is_ok()
			{}

			if al_set_audio_stream_fragment(self.allegro_audio_stream, fragment) != 0
			{
				Ok(true)
			}
			else
			{
				Err(())
			}
		}
	}
}

impl Drop for AudioStream
{
	fn drop(&mut self)
	{
		self.detach();
		unsafe {
			al_destroy_audio_stream(self.allegro_audio_stream);
		}
	}
}

impl AttachToMixerImpl for AudioStream
{
	fn create_connection(&mut self, allegro_mixer: *mut ALLEGRO_MIXER) -> Result<Connection, ()>
	{
		if unsafe { al_attach_audio_stream_to_mixer(self.allegro_audio_stream, allegro_mixer) == 0 }
		{
			Err(())
		}
		else
		{
			let (c1, c2) = Connection::new(unsafe { mem::transmute(self.allegro_audio_stream) }, AudioStream::detach);
			self.parent = Some(c1);
			Ok(c2)
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
