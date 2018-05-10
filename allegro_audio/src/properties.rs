// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use allegro_audio_sys::*;

#[derive(PartialEq, Copy, Clone)]
pub enum AudioDepth
{
	I8,
	I16,
	I24,
	U8,
	U16,
	U24,
	F32,
}

impl AudioDepth
{
	pub fn from_allegro(val: ALLEGRO_AUDIO_DEPTH) -> AudioDepth
	{
		match val
		{
			ALLEGRO_AUDIO_DEPTH_INT8 => AudioDepth::I8,
			ALLEGRO_AUDIO_DEPTH_INT16 => AudioDepth::I16,
			ALLEGRO_AUDIO_DEPTH_INT24 => AudioDepth::I24,
			ALLEGRO_AUDIO_DEPTH_UNSIGNED => AudioDepth::U8,
			ALLEGRO_AUDIO_DEPTH_UINT16 => AudioDepth::U16,
			ALLEGRO_AUDIO_DEPTH_UINT24 => AudioDepth::U24,
			ALLEGRO_AUDIO_DEPTH_FLOAT32 => AudioDepth::F32,
		}
	}

	pub fn get(&self) -> ALLEGRO_AUDIO_DEPTH
	{
		match *self
		{
			AudioDepth::I8 => ALLEGRO_AUDIO_DEPTH_INT8,
			AudioDepth::I16 => ALLEGRO_AUDIO_DEPTH_INT16,
			AudioDepth::I24 => ALLEGRO_AUDIO_DEPTH_INT24,
			AudioDepth::U8 => ALLEGRO_AUDIO_DEPTH_UNSIGNED,
			AudioDepth::U16 => ALLEGRO_AUDIO_DEPTH_UINT16,
			AudioDepth::U24 => ALLEGRO_AUDIO_DEPTH_UINT24,
			AudioDepth::F32 => ALLEGRO_AUDIO_DEPTH_FLOAT32,
		}
	}

	pub fn get_byte_size(&self) -> usize
	{
		match *self
		{
			AudioDepth::I8 => 1,
			AudioDepth::I16 => 2,
			AudioDepth::I24 => 3,
			AudioDepth::U8 => 1,
			AudioDepth::U16 => 2,
			AudioDepth::U24 => 3,
			AudioDepth::F32 => 4,
		}
	}
}

#[derive(Copy, Clone)]
pub enum ChannelConf
{
	Conf1,
	Conf2,
	Conf3,
	Conf4,
	Conf51,
	Conf61,
	Conf71,
}

impl ChannelConf
{
	pub fn from_allegro(val: ALLEGRO_CHANNEL_CONF) -> ChannelConf
	{
		match val
		{
			ALLEGRO_CHANNEL_CONF_1 => ChannelConf::Conf1,
			ALLEGRO_CHANNEL_CONF_2 => ChannelConf::Conf2,
			ALLEGRO_CHANNEL_CONF_3 => ChannelConf::Conf3,
			ALLEGRO_CHANNEL_CONF_4 => ChannelConf::Conf4,
			ALLEGRO_CHANNEL_CONF_5_1 => ChannelConf::Conf51,
			ALLEGRO_CHANNEL_CONF_6_1 => ChannelConf::Conf61,
			ALLEGRO_CHANNEL_CONF_7_1 => ChannelConf::Conf71,
		}
	}

	pub fn get(&self) -> ALLEGRO_CHANNEL_CONF
	{
		match *self
		{
			ChannelConf::Conf1 => ALLEGRO_CHANNEL_CONF_1,
			ChannelConf::Conf2 => ALLEGRO_CHANNEL_CONF_2,
			ChannelConf::Conf3 => ALLEGRO_CHANNEL_CONF_3,
			ChannelConf::Conf4 => ALLEGRO_CHANNEL_CONF_4,
			ChannelConf::Conf51 => ALLEGRO_CHANNEL_CONF_5_1,
			ChannelConf::Conf61 => ALLEGRO_CHANNEL_CONF_6_1,
			ChannelConf::Conf71 => ALLEGRO_CHANNEL_CONF_7_1,
		}
	}

	pub fn get_num_channels(&self) -> usize
	{
		match *self
		{
			ChannelConf::Conf1 => 1,
			ChannelConf::Conf2 => 2,
			ChannelConf::Conf3 => 3,
			ChannelConf::Conf4 => 4,
			ChannelConf::Conf51 => 6,
			ChannelConf::Conf61 => 7,
			ChannelConf::Conf71 => 8,
		}
	}
}

#[derive(Copy, Clone)]
pub enum Playmode
{
	Once,
	Loop,
	BiDir,
}

impl Playmode
{
	pub fn from_allegro(val: ALLEGRO_PLAYMODE) -> Playmode
	{
		match val
		{
			ALLEGRO_PLAYMODE_ONCE => Playmode::Once,
			ALLEGRO_PLAYMODE_LOOP => Playmode::Loop,
			ALLEGRO_PLAYMODE_BIDIR => Playmode::BiDir,
			_ => unreachable!(),
		}
	}

	pub fn get(&self) -> ALLEGRO_PLAYMODE
	{
		match *self
		{
			Playmode::Once => ALLEGRO_PLAYMODE_ONCE,
			Playmode::Loop => ALLEGRO_PLAYMODE_LOOP,
			Playmode::BiDir => ALLEGRO_PLAYMODE_BIDIR,
		}
	}
}

#[derive(Copy, Clone)]
pub enum MixerQuality
{
	Point,
	Linear,
	Cubic,
}

impl MixerQuality
{
	pub fn from_allegro(val: ALLEGRO_MIXER_QUALITY) -> MixerQuality
	{
		match val
		{
			ALLEGRO_MIXER_QUALITY_POINT => MixerQuality::Point,
			ALLEGRO_MIXER_QUALITY_LINEAR => MixerQuality::Linear,
			ALLEGRO_MIXER_QUALITY_CUBIC => MixerQuality::Cubic,
		}
	}

	pub fn get(&self) -> ALLEGRO_MIXER_QUALITY
	{
		match *self
		{
			MixerQuality::Point => ALLEGRO_MIXER_QUALITY_POINT,
			MixerQuality::Linear => ALLEGRO_MIXER_QUALITY_LINEAR,
			MixerQuality::Cubic => ALLEGRO_MIXER_QUALITY_CUBIC,
		}
	}
}
