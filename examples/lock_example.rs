// This file is released into Public Domain.
use allegro::*;
use getopts::*;
use std::env;
use std::mem::MaybeUninit;

#[cfg(any(allegro_5_2_0, allegro_5_1_0))]
allegro_main! {
	let args = env::args().collect::<Vec<_>>();

	let mut opts = Options::new();
	opts.optflag("i", "init-only", "only initialize Allegro, don't do anything else");
	let matches = opts.parse(&args[1..]).unwrap();

	let init_only = matches.opt_present("i");

	let core = Core::init().unwrap();

	if init_only
	{
		return;
	}

	let mut disp = Display::new(&core, 800, 600).unwrap();
	disp.set_window_title("Locking example");

	let timer = Timer::new(&core, 1.0 / 60.0).unwrap();

	let q = EventQueue::new(&core).unwrap();
	q.register_event_source(disp.get_event_source());
	q.register_event_source(timer.get_event_source());

	let mut bmp = Bitmap::new(&core, 128, 128).unwrap();
	{
		let mut lock = bmp.lock_write_only(32, 64, 64, 64, PixelFormat::Rgba8888).unwrap();
		for i in 0..lock.height()
		{
			let row = lock.row_mut::<u32>(i).unwrap();
			for pixel in row
			{
				*pixel = MaybeUninit::new(0xffff00ff);
			}
		}
	}

	let mut redraw = true;
	timer.start();
	'exit: loop
	{
		if redraw && q.is_empty()
		{
			core.clear_to_color(Color::from_rgb(0, 0, 0));
			core.draw_bitmap(&bmp, 0.0, 0.0, Flag::zero());

			core.flip_display();
			redraw = false;
		}

		match q.wait_for_event()
		{
			DisplayClose{..} =>
			{
				break 'exit;
			},
			TimerTick{..} =>
			{
				redraw = true;
			},
			_ => ()
		}
	}
}
