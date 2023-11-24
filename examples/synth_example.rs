// This file is released into Public Domain.
extern crate allegro;
extern crate allegro_acodec;
extern crate allegro_audio;
extern crate allegro_font;
extern crate byteorder;
extern crate getopts;

use allegro::*;
use allegro_audio::*;
use byteorder::{ByteOrder, LittleEndian};
use getopts::*;
use std::env;
use std::io::Write;

allegro_main! {
	let args = env::args().collect::<Vec<_>>();

	let mut opts = Options::new();
	opts.optflag(
		"i",
		"init-only",
		"only initialize Allegro, don't do anything else",
	);
	let matches = opts.parse(&args[1..]).unwrap();

	let init_only = matches.opt_present("i");
	let on_travis = env::var("TRAVIS").is_ok();

	if init_only && on_travis
	{
		// No Audio on Travis
		return;
	}

	let core = Core::init().unwrap();
	let audio_addon = AudioAddon::init(&core).unwrap();

	if init_only
	{
		return;
	}

	let disp = Display::new(&core, 800, 600).unwrap();
	disp.set_window_title("Synth example");

	core.install_keyboard().unwrap();

	let timer = Timer::new(&core, 1.0 / 60.0).unwrap();

	let q = EventQueue::new(&core).unwrap();
	q.register_event_source(disp.get_event_source());
	q.register_event_source(core.get_keyboard_event_source().unwrap());
	q.register_event_source(timer.get_event_source());

	let mut sink = Sink::new(&audio_addon).unwrap();
	let num_samples = 2048;
	let freq = 44100;
	let mut stream = AudioStream::new(
		&audio_addon,
		4,
		num_samples,
		freq,
		AudioDepth::F32,
		ChannelConf::Conf2,
	)
	.unwrap();
	stream
		.attach(&mut sink)
		.ok()
		.expect("Could not attach to stream");
	let black = Color::from_rgb_f(0.0, 0.0, 0.0);

	let mut synth_time: f64 = 0.;
	let mut redraw = true;
	timer.start();
	'exit: loop
	{
		if redraw && q.is_empty()
		{
			core.clear_to_color(black);
			core.flip_display();
			redraw = false;
		}

		match q.wait_for_event()
		{
			DisplayClose { .. } =>
			{
				break 'exit;
			}
			KeyDown { keycode: k, .. } if k == KeyCode::Escape =>
			{
				break 'exit;
			}
			TimerTick { .. } =>
			{
				let mut cb = |buf: &mut dyn Write| {
					for _ in 0..num_samples
					{
						let two_pi = 2.0 * std::f32::consts::PI;
						let gain =
							0.1 + 0.9 * (0.5 + 0.5 * (synth_time as f32 * 0.5 * two_pi)).sin();
						let v1 = (synth_time as f32 * 440.0 * two_pi).sin();
						let v2 = (synth_time as f32 * 220.0 * two_pi).sin();
						// Left, right.
						let mut byte_buf = [0; 4 * 2];
						LittleEndian::write_f32(&mut byte_buf, gain * v1);
						LittleEndian::write_f32(&mut byte_buf[4..], 0.25 * gain * v2);
						buf.write_all(&byte_buf).ok();
						synth_time += 1.0 / freq as f64;
					}
				};
				stream.write_fragment(&mut cb).ok();
				redraw = true;
			}
			_ => (),
		}
	}
}
