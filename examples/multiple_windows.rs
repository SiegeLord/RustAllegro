// This file is released into Public Domain.
#[macro_use]
extern crate allegro;
extern crate allegro_font;
extern crate getopts;

use allegro::*;
use allegro_font::*;
use getopts::*;
use std::env;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread::spawn;

fn other_window(core: Arc<Core>, font_addon: Arc<FontAddon>, sender: mpsc::SyncSender<()>, init_only: bool)
{
	if init_only
	{
		return;
	}

	let disp = Display::new(&core, 800, 600).unwrap();
	disp.set_window_title("Secondary Window");

	let timer = Timer::new(&core, 1.0 / 60.0).unwrap();

	let q = EventQueue::new(&core).unwrap();
	q.register_event_source(disp.get_event_source());
	q.register_event_source(core.get_keyboard_event_source().unwrap());
	q.register_event_source(timer.get_event_source());

	let font = Font::new_builtin(&*font_addon).unwrap();

	let mut c = 0.0f32;
	let mut d = 0.01;
	let mut redraw = true;
	timer.start();
	'exit: loop
	{
		if redraw && q.is_empty()
		{
			core.set_target_bitmap(Some(disp.get_backbuffer()));
			core.clear_to_color(Color::from_rgb_f(0.0, 0.0, c));
			core.draw_text(
				&font,
				Color::from_rgb_f(1.0, 1.0, 1.0),
				(disp.get_width() / 2) as f32,
				(disp.get_height() / 2) as f32,
				FontAlign::Centre,
				"Whee... multiple windows!",
			);
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
			}
			_ => (),
		}
	}

	sender.send(()).ok();
}

allegro_main!
{
	let args = env::args().collect::<Vec<_>>();

	let mut opts = Options::new();
	opts.optflag("i", "init-only", "only initialize Allegro, don't do anything else");
	let matches = opts.parse(&args[1..]).unwrap();

	let init_only = matches.opt_present("i");

	let core = Arc::new(Core::init().unwrap());
	let font_addon = Arc::new(FontAddon::init(&core).unwrap());
	core.install_keyboard().unwrap();

	let (sender, receiver) = mpsc::sync_channel(0);

	let core2 = core.clone();
	let font_addon2 = font_addon.clone();

	if init_only
	{
		return;
	}

	let disp = Display::new(&core, 800, 600).unwrap();
	disp.set_window_title("Main Window");

	spawn(move ||
	{
		other_window(core2, font_addon2, sender, init_only);
	});

	let timer = Timer::new(&core, 1.0 / 60.0).unwrap();

	let q = EventQueue::new(&core).unwrap();
	q.register_event_source(disp.get_event_source());
	q.register_event_source(core.get_keyboard_event_source().unwrap());
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
			core.clear_to_color(Color::from_rgb_f(c, 0.0, 0.0));
			core.draw_text(&font, Color::from_rgb_f(1.0, 1.0, 1.0), (disp.get_width() / 2) as f32,
				(disp.get_height() / 2) as f32, FontAlign::Centre, text);
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
			TimerTick{..} =>
			{
				match receiver.try_recv()
				{
					Err(mpsc::TryRecvError::Disconnected) => text = "You closed my buddy!",
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
