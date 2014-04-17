use libc::*;
use std::option::Some;
use std::cast;

use ffi::*;
use internal::Connection;
use internal::HasMixer;
use internal::AttachToMixerImpl;
use properties::*;

pub trait AttachToMixer : AttachToMixerImpl
{
	fn detach(&mut self);

	fn attach<T: HasMixer>(&mut self, mixer: &mut T) -> bool
	{
		let m = mixer.get_mixer_mut();
		let conn = self.create_connection(m.allegro_mixer);
		match conn
		{
			Some(conn) =>
			{
				m.children.push(conn);
				true
			},
			_ => false
		}
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
	fn new(frequency: u32, depth: AudioDepth, chan_conf: ChannelConf) -> Option<Mixer>
	{
		let mixer = unsafe { al_create_mixer(frequency as c_uint, depth.get(), chan_conf.get()) };
		if mixer.is_null()
		{
			None
		}
		else
		{
			Some(Mixer
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
			al_detach_mixer(cast::transmute(allegro_mixer));
		}
	}
}

pub trait MixerLike : HasMixer
{

}

impl AttachToMixerImpl for Mixer
{
	fn create_connection(&mut self, allegro_mixer: *mut ALLEGRO_MIXER) -> Option<Connection>
	{
		if self.allegro_mixer == allegro_mixer
		{
			None
		}
		else if unsafe{ al_attach_mixer_to_mixer(allegro_mixer, self.allegro_mixer) == 0 }
		{
			None
		}
		else
		{
			let (c1, c2) = Connection::new(unsafe{ cast::transmute(self.allegro_mixer) }, Mixer::detach);
			self.parent = Some(c1);
			Some(c2)
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
	pub fn new_mixer(&self) -> Option<Mixer>
	{
		self.new_custom_mixer(44100, AudioDepthF32, ChannelConf2)
	}

	pub fn new_custom_mixer(&self, frequency: u32, depth: AudioDepth, chan_conf: ChannelConf) -> Option<Mixer>
	{
		Mixer::new(frequency, depth, chan_conf)
	}
}
