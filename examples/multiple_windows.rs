// This file is released into Public Domain.
#![feature(rustc_private)]
#![feature(collections)]

#[macro_use]
extern crate allegro;
extern crate allegro_font;
extern crate getopts;

use getopts::*;
use std::sync::mpsc;
use std::env;

use allegro::*;
use allegro_font::*;

fn other_window(mut core: Core, sender: mpsc::SyncSender<()>, init_only: bool)
{
	let font_addon = FontAddon::init(&core).unwrap();

	if init_only
	{
		return;
	}

	let disp = Display::new(&core, 800, 600).unwrap();
	disp.set_window_title("Secondary Window");

	let timer = Timer::new(&core, 1.0 / 60.0).unwrap();

	let q = EventQueue::new(&core).unwrap();
	q.register_event_source(disp.get_event_source());
	q.register_event_source(core.get_keyboard_event_source());
	q.register_event_source(timer.get_event_source());

	let font = Font::new_builtin(&font_addon).unwrap();

	let mut c = 0.0f32;
	let mut d = 0.01;
	let mut redraw = true;
	timer.start();
	'exit: loop
	{
		if redraw && q.is_empty()
		{
			core.clear_to_color(core.map_rgb_f(0.0, 0.0, c));
			core.draw_text(&font, core.map_rgb_f(1.0, 1.0, 1.0), (disp.get_width() / 2) as f32, (disp.get_height() / 2) as f32, FontAlign::Centre, "Whee... multiple windows!");
			disp.flip();
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
			TimerTick{..} =>
			{
				redraw = true;
				c += d;
				if c > 1.0
				{
					c = 1.0;
					d = -d;
				}
				if c < 0.0
				{
					c = 0.0;
					d = -d;
				}
			},
			_ => ()
		}
	}

	sender.send(()).ok();
}

allegro_main!
{
	let args = env::args().collect::<Vec<_>>();

	let opts = vec![
		optflag("i", "init-only", "only initialize Allegro, don't do anything else")
	];

	let matches = getopts(args.tail(), &opts[..]).unwrap();

	let init_only = matches.opt_present("i");

	let mut core = Core::init().unwrap();

	let font_addon = FontAddon::init(&core).unwrap();
	core.install_keyboard().unwrap();

	let (sender, receiver) = mpsc::sync_channel(0);

	core.spawn(move |core|
	{
		other_window(core, sender, init_only);
	});

	if init_only
	{
		return;
	}

	let disp = Display::new(&core, 800, 600).unwrap();
	disp.set_window_title("Main Window");

	let timer = Timer::new(&core, 1.0 / 60.0).unwrap();

	let q = EventQueue::new(&core).unwrap();
	q.register_event_source(disp.get_event_source());
	q.register_event_source(core.get_keyboard_event_source());
	q.register_event_source(timer.get_event_source());

	let font = Font::new_builtin(&font_addon).unwrap();

	let mut text = "Whee... multiple windows!";
	let mut c = 0.0f32;
	let mut d = 0.01;
	let mut redraw = true;
	timer.start();
	'exit: loop
	{
		if redraw && q.is_empty()
		{
			core.clear_to_color(core.map_rgb_f(c, 0.0, 0.0));
			core.draw_text(&font, core.map_rgb_f(1.0, 1.0, 1.0), (disp.get_width() / 2) as f32, (disp.get_height() / 2) as f32, FontAlign::Centre, text);
			disp.flip();
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
			TimerTick{..} =>
			{
				match receiver.try_recv()
				{
					Err(mpsc::TryRecvError::Disconnected) | Ok(..) => text = "You closed my buddy!",
					_ => ()
				}

				c += d;
				if c > 1.0
				{
					c = 1.0;
					d = -d;
				}
				if c < 0.0
				{
					c = 0.0;
					d = -d;
				}
				redraw = true;
			},
			_ => ()
		}
	}
}
