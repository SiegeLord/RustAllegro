// This file is released into Public Domain.
#[macro_use]
extern crate allegro;
extern crate allegro_acodec;
extern crate allegro_audio;
extern crate allegro_font;
extern crate getopts;

use allegro::*;
use allegro_acodec::*;
use allegro_audio::*;
use allegro_font::*;
use getopts::*;
use std::env;

struct AudioCallback
{
	silence: bool,
}

impl PostProcessCallback for AudioCallback
{
	fn process(&mut self, data: &mut [u8], _: u32)
	{
		if self.silence
		{
			for u in data.iter_mut()
			{
				*u = 0;
			}
		}
	}
}

allegro_main!
{
	let args = env::args().collect::<Vec<_>>();

	let mut opts = Options::new();
	opts.optflag("i", "init-only", "only initialize Allegro, don't do anything else");
	opts.optflag("s", "silence", "use the post-process callback to silence the audio");
	let matches = opts.parse(&args[1..]).unwrap();

	let init_only = matches.opt_present("i");
	let on_travis = env::var("TRAVIS").is_ok();

	if init_only && on_travis
	{
		// No Audio on Travis
		return;
	}

	let core = Core::init().unwrap();
	let font_addon = FontAddon::init(&core).unwrap();
	let audio_addon = AudioAddon::init(&core).unwrap();
	AcodecAddon::init(&audio_addon).unwrap();

	if init_only
	{
		return;
	}

	let disp = Display::new(&core, 800, 600).unwrap();
	disp.set_window_title("Audio example");

	core.install_keyboard().unwrap();

	let timer = Timer::new(&core, 1.0 / 60.0).unwrap();

	let q = EventQueue::new(&core).unwrap();
	q.register_event_source(disp.get_event_source());
	q.register_event_source(core.get_keyboard_event_source().unwrap());
	q.register_event_source(timer.get_event_source());

	let callback = Box::new(AudioCallback{ silence: matches.opt_present("silence") });
	let mut sink = Sink::new(&audio_addon).unwrap();
	sink.set_postprocess_callback(Some(callback)).unwrap();
	let font = Font::new_builtin(&font_addon).unwrap();
	let mut _sample_instance = None;
	let sample = Sample::load(&audio_addon, "data/welcome.ogg").unwrap();
	let mut stream = AudioStream::load(&audio_addon, "data/music.ogg").unwrap();
	stream.attach(&mut sink).ok().expect("Could not attach to stream");
	stream.set_playmode(Playmode::Loop).unwrap();
	let white = Color::from_rgb_f(1.0, 1.0, 1.0);
	let black = Color::from_rgb_f(0.0, 0.0, 0.0);

	let mut redraw = true;
	timer.start();
	'exit: loop
	{
		if redraw && q.is_empty()
		{
			core.clear_to_color(black);
			core.draw_text(&font, white, (disp.get_width() / 2) as f32, 32.0, FontAlign::Centre, "Press SPACE to be welcomed!");
			core.flip_display();
			redraw = false;
		}

		match q.wait_for_event()
		{
			DisplayClose{..} =>
			{
				break 'exit;
			},
			KeyDown{keycode: k, ..} if k == KeyCode::Escape =>
			{
				break 'exit;
			},
			KeyDown{keycode: k, ..} if k == KeyCode::Space =>
			{
				_sample_instance = sink.play_sample(&sample, 1.0, Some(0.0), 1.0, Playmode::Once).ok();
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
