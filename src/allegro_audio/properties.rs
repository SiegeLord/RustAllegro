// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use ffi::*;

pub enum AudioDepth
{
	AudioDepthI8,
	AudioDepthI16,
	AudioDepthI24,
	AudioDepthU8,
	AudioDepthU16,
	AudioDepthU24,
	AudioDepthF32,
}

impl AudioDepth
{
	pub fn get(&self) -> ALLEGRO_AUDIO_DEPTH
	{
		match *self
		{
			AudioDepthI8 => ALLEGRO_AUDIO_DEPTH_INT8,
			AudioDepthI16 => ALLEGRO_AUDIO_DEPTH_INT16,
			AudioDepthI24 => ALLEGRO_AUDIO_DEPTH_INT24,
			AudioDepthU8 => ALLEGRO_AUDIO_DEPTH_UNSIGNED,
			AudioDepthU16 => ALLEGRO_AUDIO_DEPTH_UINT16,
			AudioDepthU24 => ALLEGRO_AUDIO_DEPTH_UINT24,
			AudioDepthF32 => ALLEGRO_AUDIO_DEPTH_FLOAT32,
		}
	}
}

pub enum ChannelConf
{
	ChannelConf1,
	ChannelConf2,
	ChannelConf3,
	ChannelConf4,
	ChannelConf51,
	ChannelConf61,
	ChannelConf71,
}

impl ChannelConf
{
	pub fn get(&self) -> ALLEGRO_CHANNEL_CONF
	{
		match *self
		{
			ChannelConf1   => ALLEGRO_CHANNEL_CONF_1,
			ChannelConf2   => ALLEGRO_CHANNEL_CONF_2,
			ChannelConf3   => ALLEGRO_CHANNEL_CONF_3,
			ChannelConf4   => ALLEGRO_CHANNEL_CONF_4,
			ChannelConf51 => ALLEGRO_CHANNEL_CONF_5_1,
			ChannelConf61 => ALLEGRO_CHANNEL_CONF_6_1,
			ChannelConf71 => ALLEGRO_CHANNEL_CONF_7_1,
		}
	}
}

pub enum Playmode
{
	PlaymodeOnce,
	PlaymodeLoop,
	PlaymodeBiDir,
}

impl Playmode
{
	pub fn get(&self) -> ALLEGRO_PLAYMODE
	{
		match *self
		{
			PlaymodeOnce => ALLEGRO_PLAYMODE_ONCE,
			PlaymodeLoop => ALLEGRO_PLAYMODE_LOOP,
			PlaymodeBiDir => ALLEGRO_PLAYMODE_BIDIR,
		}
	}
}

pub enum MixerQuality
{
	MixerQualityPoint,
	MixerQualityLinear,
	MixerQualityCubic,
}

impl MixerQuality
{
	pub fn get(&self) -> ALLEGRO_MIXER_QUALITY
	{
		match *self
		{
			MixerQualityPoint => ALLEGRO_MIXER_QUALITY_POINT,
			MixerQualityLinear => ALLEGRO_MIXER_QUALITY_LINEAR,
			MixerQualityCubic => ALLEGRO_MIXER_QUALITY_CUBIC,
		}
	}
}
