// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![allow(non_camel_case_types)]

pub use self::allegro_audio::*;

pub mod allegro_audio
{
	use libc::*;
	use allegro::c_bool;
	use allegro::ffi::*;

	// These are enums instead of statics because functions expect these an arguments.
	// An API bug that cannot really be fixed.
	#[repr(C)]
	pub enum ALLEGRO_AUDIO_DEPTH
	{
		ALLEGRO_AUDIO_DEPTH_INT8 = 0,
		ALLEGRO_AUDIO_DEPTH_INT16 = 1,
		ALLEGRO_AUDIO_DEPTH_INT24 = 2,
		ALLEGRO_AUDIO_DEPTH_FLOAT32 = 3,
		ALLEGRO_AUDIO_DEPTH_UNSIGNED = 8,
		// Can't put this one here, but we don't really care for this anyway, as we wrap this differently.
		//~ ALLEGRO_AUDIO_DEPTH_UINT8 = 8,
		ALLEGRO_AUDIO_DEPTH_UINT16 = 9,
		ALLEGRO_AUDIO_DEPTH_UINT24 = 10,
	}
	pub static ALLEGRO_AUDIO_DEPTH_UINT8: u32 = 8;

	#[repr(C)]
	pub enum ALLEGRO_CHANNEL_CONF
	{
		ALLEGRO_CHANNEL_CONF_1 = 16,
		ALLEGRO_CHANNEL_CONF_2 = 32,
		ALLEGRO_CHANNEL_CONF_3 = 48,
		ALLEGRO_CHANNEL_CONF_4 = 64,
		ALLEGRO_CHANNEL_CONF_5_1 = 81,
		ALLEGRO_CHANNEL_CONF_6_1 = 97,
		ALLEGRO_CHANNEL_CONF_7_1 = 113,
	}

	#[repr(C)]
	pub enum ALLEGRO_PLAYMODE
	{
		ALLEGRO_PLAYMODE_ONCE = 256,
		ALLEGRO_PLAYMODE_LOOP = 257,
		ALLEGRO_PLAYMODE_BIDIR = 258,
		_ALLEGRO_PLAYMODE_STREAM_ONCE = 259,
		_ALLEGRO_PLAYMODE_STREAM_ONEDIR = 260,
	}

	#[repr(C)]
	pub enum ALLEGRO_MIXER_QUALITY
	{
		ALLEGRO_MIXER_QUALITY_POINT = 272,
		ALLEGRO_MIXER_QUALITY_LINEAR = 273,
		ALLEGRO_MIXER_QUALITY_CUBIC = 274,
	}

	opaque!(ALLEGRO_SAMPLE)

	pub struct ALLEGRO_SAMPLE_ID
	{
		pub _index: c_int,
		pub _id: c_int,
	}

	opaque!(ALLEGRO_SAMPLE_INSTANCE)
	opaque!(ALLEGRO_AUDIO_STREAM)
	opaque!(ALLEGRO_MIXER)
	opaque!(ALLEGRO_VOICE)

	extern "C"
	{
		pub fn al_create_sample(buf: *mut c_void, samples: c_uint, freq: c_uint, depth: ALLEGRO_AUDIO_DEPTH, chan_conf: ALLEGRO_CHANNEL_CONF, free_buf: c_bool) -> *mut ALLEGRO_SAMPLE;
		pub fn al_destroy_sample(spl: *mut ALLEGRO_SAMPLE);
		pub fn al_create_sample_instance(data: *mut ALLEGRO_SAMPLE) -> *mut ALLEGRO_SAMPLE_INSTANCE;
		pub fn al_destroy_sample_instance(spl: *mut ALLEGRO_SAMPLE_INSTANCE);
		pub fn al_get_sample_frequency(spl: *ALLEGRO_SAMPLE) -> c_uint;
		pub fn al_get_sample_length(spl: *ALLEGRO_SAMPLE) -> c_uint;
		pub fn al_get_sample_depth(spl: *ALLEGRO_SAMPLE) -> ALLEGRO_AUDIO_DEPTH;
		pub fn al_get_sample_channels(spl: *ALLEGRO_SAMPLE) -> ALLEGRO_CHANNEL_CONF;
		pub fn al_get_sample_data(spl: *ALLEGRO_SAMPLE) -> *mut c_void;

		pub fn al_get_sample_instance_frequency(spl: *ALLEGRO_SAMPLE_INSTANCE) -> c_uint;
		pub fn al_get_sample_instance_length(spl: *ALLEGRO_SAMPLE_INSTANCE) -> c_uint;
		pub fn al_get_sample_instance_position(spl: *ALLEGRO_SAMPLE_INSTANCE) -> c_uint;
		pub fn al_get_sample_instance_speed(spl: *ALLEGRO_SAMPLE_INSTANCE) -> c_float;
		pub fn al_get_sample_instance_gain(spl: *ALLEGRO_SAMPLE_INSTANCE) -> c_float;
		pub fn al_get_sample_instance_pan(spl: *ALLEGRO_SAMPLE_INSTANCE) -> c_float;
		pub fn al_get_sample_instance_time(spl: *ALLEGRO_SAMPLE_INSTANCE) -> c_float;
		pub fn al_get_sample_instance_depth(spl: *ALLEGRO_SAMPLE_INSTANCE) -> ALLEGRO_AUDIO_DEPTH;
		pub fn al_get_sample_instance_channels(spl: *ALLEGRO_SAMPLE_INSTANCE) -> ALLEGRO_CHANNEL_CONF;
		pub fn al_get_sample_instance_playmode(spl: *ALLEGRO_SAMPLE_INSTANCE) -> ALLEGRO_PLAYMODE;
		pub fn al_get_sample_instance_playing(spl: *ALLEGRO_SAMPLE_INSTANCE) -> c_bool;
		pub fn al_get_sample_instance_attached(spl: *ALLEGRO_SAMPLE_INSTANCE) -> c_bool;
		pub fn al_set_sample_instance_position(spl: *mut ALLEGRO_SAMPLE_INSTANCE, val: c_uint) -> c_bool;
		pub fn al_set_sample_instance_length(spl: *mut ALLEGRO_SAMPLE_INSTANCE, val: c_uint) -> c_bool;
		pub fn al_set_sample_instance_speed(spl: *mut ALLEGRO_SAMPLE_INSTANCE, val: c_float) -> c_bool;
		pub fn al_set_sample_instance_gain(spl: *mut ALLEGRO_SAMPLE_INSTANCE, val: c_float) -> c_bool;
		pub fn al_set_sample_instance_pan(spl: *mut ALLEGRO_SAMPLE_INSTANCE, val: c_float) -> c_bool;
		pub fn al_set_sample_instance_playmode(spl: *mut ALLEGRO_SAMPLE_INSTANCE, val: ALLEGRO_PLAYMODE) -> c_bool;
		pub fn al_set_sample_instance_playing(spl: *mut ALLEGRO_SAMPLE_INSTANCE, val: c_bool) -> c_bool;
		pub fn al_detach_sample_instance(spl: *mut ALLEGRO_SAMPLE_INSTANCE) -> c_bool;
		pub fn al_set_sample(spl: *mut ALLEGRO_SAMPLE_INSTANCE, data: *mut ALLEGRO_SAMPLE) -> c_bool;
		pub fn al_get_sample(spl: *mut ALLEGRO_SAMPLE_INSTANCE) -> *mut ALLEGRO_SAMPLE;
		pub fn al_play_sample_instance(spl: *mut ALLEGRO_SAMPLE_INSTANCE) -> c_bool;
		pub fn al_stop_sample_instance(spl: *mut ALLEGRO_SAMPLE_INSTANCE) -> c_bool;

		pub fn al_create_audio_stream(buffer_count: size_t, samples: c_uint, freq: c_uint, depth: ALLEGRO_AUDIO_DEPTH, chan_conf: ALLEGRO_CHANNEL_CONF) -> *mut ALLEGRO_AUDIO_STREAM;
		pub fn al_destroy_audio_stream(stream: *mut ALLEGRO_AUDIO_STREAM);
		pub fn al_drain_audio_stream(stream: *mut ALLEGRO_AUDIO_STREAM);
		pub fn al_get_audio_stream_frequency(stream: *ALLEGRO_AUDIO_STREAM) -> c_uint;
		pub fn al_get_audio_stream_length(stream: *ALLEGRO_AUDIO_STREAM) -> c_uint;
		pub fn al_get_audio_stream_fragments(stream: *ALLEGRO_AUDIO_STREAM) -> c_uint;
		pub fn al_get_available_audio_stream_fragments(stream: *ALLEGRO_AUDIO_STREAM) -> c_uint;
		pub fn al_get_audio_stream_speed(stream: *ALLEGRO_AUDIO_STREAM) -> c_float;
		pub fn al_get_audio_stream_gain(stream: *ALLEGRO_AUDIO_STREAM) -> c_float;
		pub fn al_get_audio_stream_pan(stream: *ALLEGRO_AUDIO_STREAM) -> c_float;
		pub fn al_get_audio_stream_channels(stream: *ALLEGRO_AUDIO_STREAM) -> ALLEGRO_CHANNEL_CONF;
		pub fn al_get_audio_stream_depth(stream: *ALLEGRO_AUDIO_STREAM) -> ALLEGRO_AUDIO_DEPTH;
		pub fn al_get_audio_stream_playmode(stream: *ALLEGRO_AUDIO_STREAM) -> ALLEGRO_PLAYMODE;
		pub fn al_get_audio_stream_playing(spl: *ALLEGRO_AUDIO_STREAM) -> c_bool;
		pub fn al_get_audio_stream_attached(spl: *ALLEGRO_AUDIO_STREAM) -> c_bool;
		pub fn al_get_audio_stream_fragment(stream: *ALLEGRO_AUDIO_STREAM) -> *mut c_void;
		pub fn al_set_audio_stream_speed(stream: *mut ALLEGRO_AUDIO_STREAM, val: c_float) -> c_bool;
		pub fn al_set_audio_stream_gain(stream: *mut ALLEGRO_AUDIO_STREAM, val: c_float) -> c_bool;
		pub fn al_set_audio_stream_pan(stream: *mut ALLEGRO_AUDIO_STREAM, val: c_float) -> c_bool;
		pub fn al_set_audio_stream_playmode(stream: *mut ALLEGRO_AUDIO_STREAM, val: ALLEGRO_PLAYMODE) -> c_bool;
		pub fn al_set_audio_stream_playing(stream: *mut ALLEGRO_AUDIO_STREAM, val: c_bool) -> c_bool;
		pub fn al_detach_audio_stream(stream: *mut ALLEGRO_AUDIO_STREAM) -> c_bool;
		pub fn al_set_audio_stream_fragment(stream: *mut ALLEGRO_AUDIO_STREAM, val: *mut c_void) -> c_bool;
		pub fn al_rewind_audio_stream(stream: *mut ALLEGRO_AUDIO_STREAM) -> c_bool;
		pub fn al_seek_audio_stream_secs(stream: *mut ALLEGRO_AUDIO_STREAM, time: c_double) -> c_bool;
		pub fn al_get_audio_stream_position_secs(stream: *mut ALLEGRO_AUDIO_STREAM) -> c_double;
		pub fn al_get_audio_stream_length_secs(stream: *mut ALLEGRO_AUDIO_STREAM) -> c_double;
		pub fn al_set_audio_stream_loop_secs(stream: *mut ALLEGRO_AUDIO_STREAM, start: c_double, end: c_double) -> c_bool;
		pub fn al_get_audio_stream_event_source(stream: *mut ALLEGRO_AUDIO_STREAM) -> *mut ALLEGRO_EVENT_SOURCE;

		pub fn al_create_mixer(freq: c_uint, depth: ALLEGRO_AUDIO_DEPTH, chan_conf: ALLEGRO_CHANNEL_CONF) -> *mut ALLEGRO_MIXER;
		pub fn al_destroy_mixer(mixer: *mut ALLEGRO_MIXER);
		pub fn al_attach_sample_instance_to_mixer(stream: *mut ALLEGRO_SAMPLE_INSTANCE, mixer: *mut ALLEGRO_MIXER) -> c_bool;
		pub fn al_attach_audio_stream_to_mixer(stream: *mut ALLEGRO_AUDIO_STREAM, mixer: *mut ALLEGRO_MIXER) -> c_bool;
		pub fn al_attach_mixer_to_mixer(stream: *mut ALLEGRO_MIXER, mixer: *mut ALLEGRO_MIXER) -> c_bool;
		pub fn al_set_mixer_postprocess_callback(mixer: *mut ALLEGRO_MIXER, cb: Option<extern "C" fn(arg1: *mut c_void, arg2: c_uint, arg3: *mut c_void)>, data: *mut c_void) -> c_bool;
		pub fn al_get_mixer_frequency(mixer: *ALLEGRO_MIXER) -> c_uint;
		pub fn al_get_mixer_channels(mixer: *ALLEGRO_MIXER) -> ALLEGRO_CHANNEL_CONF;
		pub fn al_get_mixer_depth(mixer: *ALLEGRO_MIXER) -> ALLEGRO_AUDIO_DEPTH;
		pub fn al_get_mixer_quality(mixer: *ALLEGRO_MIXER) -> ALLEGRO_MIXER_QUALITY;
		pub fn al_get_mixer_gain(mixer: *ALLEGRO_MIXER) -> c_float;
		pub fn al_get_mixer_playing(mixer: *ALLEGRO_MIXER) -> c_bool;
		pub fn al_get_mixer_attached(mixer: *ALLEGRO_MIXER) -> c_bool;
		pub fn al_set_mixer_frequency(mixer: *mut ALLEGRO_MIXER, val: c_uint) -> c_bool;
		pub fn al_set_mixer_quality(mixer: *mut ALLEGRO_MIXER, val: ALLEGRO_MIXER_QUALITY) -> c_bool;
		pub fn al_set_mixer_gain(mixer: *mut ALLEGRO_MIXER, gain: c_float) -> c_bool;
		pub fn al_set_mixer_playing(mixer: *mut ALLEGRO_MIXER, val: c_bool) -> c_bool;
		pub fn al_detach_mixer(mixer: *mut ALLEGRO_MIXER) -> c_bool;

		pub fn al_create_voice(freq: c_uint, depth: ALLEGRO_AUDIO_DEPTH, chan_conf: ALLEGRO_CHANNEL_CONF) -> *mut ALLEGRO_VOICE;
		pub fn al_destroy_voice(voice: *mut ALLEGRO_VOICE);
		pub fn al_attach_sample_instance_to_voice(stream: *mut ALLEGRO_SAMPLE_INSTANCE, voice: *mut ALLEGRO_VOICE) -> c_bool;
		pub fn al_attach_audio_stream_to_voice(stream: *mut ALLEGRO_AUDIO_STREAM, voice: *mut ALLEGRO_VOICE) -> c_bool;
		pub fn al_attach_mixer_to_voice(mixer: *mut ALLEGRO_MIXER, voice: *mut ALLEGRO_VOICE) -> c_bool;
		pub fn al_detach_voice(voice: *mut ALLEGRO_VOICE);
		pub fn al_get_voice_frequency(voice: *ALLEGRO_VOICE) -> c_uint;
		pub fn al_get_voice_position(voice: *ALLEGRO_VOICE) -> c_uint;
		pub fn al_get_voice_channels(voice: *ALLEGRO_VOICE) -> ALLEGRO_CHANNEL_CONF;
		pub fn al_get_voice_depth(voice: *ALLEGRO_VOICE) -> ALLEGRO_AUDIO_DEPTH;
		pub fn al_get_voice_playing(voice: *ALLEGRO_VOICE) -> c_bool;
		pub fn al_set_voice_position(voice: *mut ALLEGRO_VOICE, val: c_uint) -> c_bool;
		pub fn al_set_voice_playing(voice: *mut ALLEGRO_VOICE, val: c_bool) -> c_bool;

		pub fn al_install_audio() -> c_bool;
		pub fn al_uninstall_audio();
		pub fn al_is_audio_installed() -> c_bool;
		pub fn al_get_allegro_audio_version() -> uint32_t;

		pub fn al_get_channel_count(conf: ALLEGRO_CHANNEL_CONF) -> size_t;
		pub fn al_get_audio_depth_size(conf: ALLEGRO_AUDIO_DEPTH) -> size_t;

		pub fn al_reserve_samples(reserve_samples: c_int) -> c_bool;
		pub fn al_get_default_mixer() -> *mut ALLEGRO_MIXER;
		pub fn al_set_default_mixer(mixer: *mut ALLEGRO_MIXER) -> c_bool;
		pub fn al_restore_default_mixer() -> c_bool;

		pub fn al_play_sample(data: *mut ALLEGRO_SAMPLE, gain: c_float, pan: c_float, speed: c_float, _loop: ALLEGRO_PLAYMODE, ret_id: *mut ALLEGRO_SAMPLE_ID) -> c_bool;
		pub fn al_stop_sample(spl_id: *mut ALLEGRO_SAMPLE_ID);
		pub fn al_stop_samples();

		pub fn al_register_sample_loader(ext: *c_char, loader: Option<extern "C" fn(arg1: *c_char) -> *mut ALLEGRO_SAMPLE>) -> c_bool;
		pub fn al_register_sample_saver(ext: *c_char, saver: Option<extern "C" fn(arg1: *c_char, arg2: *mut ALLEGRO_SAMPLE) -> c_bool>) -> c_bool;
		pub fn al_register_audio_stream_loader(ext: *c_char, stream_loader: Option<extern "C" fn(arg1: *c_char, arg2: size_t, arg3: c_uint) -> *mut ALLEGRO_AUDIO_STREAM>) -> c_bool;
		//~ pub fn al_register_sample_loader_f(ext: *c_char, loader: Option<extern "C" fn(arg1: *mut ALLEGRO_FILE) -> *mut ALLEGRO_SAMPLE>) -> c_bool;
		//~ pub fn al_register_sample_saver_f(ext: *c_char, saver: Option<extern "C" fn(arg1: *mut ALLEGRO_FILE, arg2: *mut ALLEGRO_SAMPLE) -> c_bool>) -> c_bool;
		//~ pub fn al_register_audio_stream_loader_f(ext: *c_char, stream_loader: Option<extern "C" fn(arg1: *mut ALLEGRO_FILE, arg2: size_t, arg3: c_uint) -> *mut ALLEGRO_AUDIO_STREAM>) -> c_bool;
		pub fn al_load_sample(filename: *c_char) -> *mut ALLEGRO_SAMPLE;
		pub fn al_save_sample(filename: *c_char, spl: *mut ALLEGRO_SAMPLE) -> c_bool;
		pub fn al_load_audio_stream(filename: *c_char, buffer_count: size_t, samples: c_uint) -> *mut ALLEGRO_AUDIO_STREAM;
		//~ pub fn al_load_sample_f(fp: *mut ALLEGRO_FILE, ident: *c_char) -> *mut ALLEGRO_SAMPLE;
		//~ pub fn al_save_sample_f(fp: *mut ALLEGRO_FILE, ident: *c_char, spl: *mut ALLEGRO_SAMPLE) -> c_bool;
		//~ pub fn al_load_audio_stream_f(fp: *mut ALLEGRO_FILE, ident: *c_char, buffer_count: size_t, samples: c_uint) -> *mut ALLEGRO_AUDIO_STREAM;
	}
}
