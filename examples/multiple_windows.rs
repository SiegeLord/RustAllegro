// This file is released into Public Domain.
#![feature(globs)]
#![feature(struct_variant)]
#![feature(phase)]

#[phase(syntax, link)]
extern crate allegro5;
extern crate allegro_font;
extern crate getopts;

use getopts::*;
use std::comm;
use std::os;
use std::c_str::*;
use allegro5::*;
use allegro_font::*;

fn other_window(mut core: Core, sender: comm::SyncSender<()>, init_only: bool)
{
	let font_addon = FontAddon::init(&core).expect("Failed to initialize the font addon");

	if init_only
	{
		return;
	}

	let disp = core.create_display(800, 600).unwrap();
	disp.set_window_title(&"Secondary Window".to_c_str());

	let timer = core.create_timer(1.0 / 60.0).unwrap();

	let q = core.create_event_queue().unwrap();
	q.register_event_source(disp.get_event_source());
	q.register_event_source(core.get_keyboard_event_source().unwrap());
	q.register_event_source(timer.get_event_source());

	let font = font_addon.create_builtin_font().unwrap();

	let mut c = 0.0f32;
	let mut d = 0.01;
	let mut redraw = true;
	timer.start();
	'exit: loop
	{
		if redraw && q.is_empty()
		{
			core.clear_to_color(core.map_rgb_f(0.0, 0.0, c));
			core.draw_text(&font, core.map_rgb_f(1.0, 1.0, 1.0), (disp.get_width() / 2) as f32, (disp.get_height() / 2) as f32, AlignCentre, "Whee... multiple windows!");
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

	sender.send(());
}

allegro_main!
{
	let args = os::args();

	let opts = ~[
		optflag("i", "init-only", "only initialize Allegro, don't do anything else")
	];

	let matches = match getopts(args.tail(), opts)
	{
		Ok(m) => { m }
		Err(f) => { fail!(f.to_err_msg()) }
	};

	let init_only = matches.opt_present("i");

	let mut core = match Core::init()
	{
		Ok(core) => core,
		Err(msg) => fail!(msg)
	};

	let font_addon = FontAddon::init(&core).expect("Failed to initialize the font addon");
	core.install_keyboard();

	let (sender, receiver) = comm::sync_channel(0);

	core.spawn(proc(core)
	{
		other_window(core, sender, init_only);
	});

	if init_only
	{
		return;
	}

	let disp = core.create_display(800, 600).unwrap();
	disp.set_window_title(&"Main Window".to_c_str());

	let timer = core.create_timer(1.0 / 60.0).unwrap();

	let q = core.create_event_queue().unwrap();
	q.register_event_source(disp.get_event_source());
	q.register_event_source(core.get_keyboard_event_source().unwrap());
	q.register_event_source(timer.get_event_source());

	let font = font_addon.create_builtin_font().unwrap();

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
			core.draw_text(&font, core.map_rgb_f(1.0, 1.0, 1.0), (disp.get_width() / 2) as f32, (disp.get_height() / 2) as f32, AlignCentre, text);
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
			TimerTick{..} =>
			{
				match receiver.try_recv()
				{
					Err(comm::Disconnected) | Ok(..) => text = "You closed my buddy!",
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
