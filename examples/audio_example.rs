// This file is released into Public Domain.
#![feature(globs)]
#![feature(struct_variant)]
#![feature(phase)]

#[phase(plugin, link)]
extern crate allegro5;
extern crate allegro_font;
extern crate allegro_audio;
extern crate allegro_acodec;
extern crate getopts;

use getopts::*;
use std::os;
use std::c_str::*;
use std::os::getenv;
use allegro5::*;
use allegro_font::*;
use allegro_audio::*;
use allegro_acodec::*;

allegro_main!
{
	let args = os::args();

	let opts = vec![
		optflag("i", "init-only", "only initialize Allegro, don't do anything else")
	];

	let matches = match getopts(args.tail(), opts.as_slice())
	{
		Ok(m) => { m }
		Err(f) => { fail!(f.to_err_msg()) }
	};

	let init_only = matches.opt_present("i");
	let on_travis = getenv("TRAVIS").is_some();

	if init_only && on_travis
	{
		// No Audio on Travis
		return;
	}

	let mut core = Core::init().unwrap();
	let font_addon = FontAddon::init(&core).expect("Failed to initialize the font addon");
	let audio_addon = AudioAddon::init(&core).expect("Failed to initialize the audio addon");
	AcodecAddon::init(&audio_addon).expect("Failed to initialize the acodec addon");

	if init_only
	{
		return;
	}

	let disp = core.create_display(800, 600).unwrap();
	disp.set_window_title(&"Audio example".to_c_str());

	core.install_keyboard();

	let timer = core.create_timer(1.0 / 60.0).unwrap();

	let q = core.create_event_queue().unwrap();
	q.register_event_source(disp.get_event_source());
	q.register_event_source(core.get_keyboard_event_source().unwrap());
	q.register_event_source(timer.get_event_source());

	let mut sink = audio_addon.create_sink().expect("Couldn't create sink");
	let font = font_addon.create_builtin_font().unwrap();
	let mut _sample_instance = None;
	let sample = audio_addon.load_sample("data/welcome.ogg").unwrap();
	let mut stream = audio_addon.load_audio_stream("data/music.ogg").unwrap();
	stream.attach(&mut sink);
	let white = core.map_rgb_f(1.0, 1.0, 1.0);
	let black = core.map_rgb_f(0.0, 0.0, 0.0);

	let mut redraw = true;
	timer.start();
	'exit: loop
	{
		if redraw && q.is_empty()
		{
			core.clear_to_color(black);
			core.draw_text(&font, white, (disp.get_width() / 2) as f32, 32.0, AlignCentre, "Press SPACE to be welcomed!");
			disp.flip();
			redraw = false;
		}

		match q.wait_for_event()
		{
			DisplayClose{..} =>
			{
				break 'exit;
			},
			KeyDown{keycode: k, ..} if k == key::Escape =>
			{
				break 'exit;
			},
			KeyDown{keycode: k, ..} if k == key::Space =>
			{
				_sample_instance = sink.play_sample(&sample, 1.0, Some(0.0), 1.0, PlaymodeOnce);
				println!("Welcome to Allegro!");
			},
			TimerTick{..} =>
			{
				redraw = true;
			},
			_ => ()
		}
	}
}
