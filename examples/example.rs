#[feature(globs)];
#[feature(struct_variant)];

extern crate allegro5;
extern crate allegro_image;
extern crate getopts;

use getopts::*;
use std::os;
use std::c_str::*;
use allegro5::*;
use allegro_image::*;

#[start]
fn start(argc: int, argv: **u8) -> int
{
	allegro5::run(argc, argv, main)
}

fn main()
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

	let mut core = Core::init().expect("Your Allegro version does not match this Rust binding");
	ImageAddon::init(&core).expect("Failed to initialize the image addon");

	if init_only
	{
		return;
	}

	let disp = core.create_display(800, 600).unwrap();
	disp.set_window_title(&"Rust example".to_c_str());

	core.install_keyboard();
	core.install_mouse();

	let timer = core.create_timer(1.0 / 60.0).unwrap();

	let q = core.create_event_queue().unwrap();
	q.register_event_source(disp.get_event_source());
	q.register_event_source(core.get_keyboard_event_source().unwrap());
	q.register_event_source(core.get_mouse_event_source().unwrap());
	q.register_event_source(timer.get_event_source());

	let bmp = core.create_bitmap(256, 256).unwrap();

	let (mon_x1, mon_y1, mon_x2, mon_y2) = core.get_monitor_info(0).unwrap();
	println!("{} {} {} {}", mon_x1, mon_y1, mon_x2, mon_y2);

	bmp.clear_to_color(core.map_rgb_f(0.0, 0.0, 1.0));

	let sub_bmp = bmp.create_sub_bitmap(64, 64, 64, 64).unwrap();
	sub_bmp.clear_to_color(core.map_rgb_f(0.0, 1.0, 1.0));

	let bkg = core.load_bitmap("data/mysha.pcx").unwrap();

	let mut theta = 0.0f32;
	let mut redraw = true;
	timer.start();
	'exit: loop
	{
		if redraw && q.is_empty()
		{
			disp.clear_to_color(core.map_rgb_f(0.0, 0.0, 0.0));
			disp.draw_bitmap(&bkg, 0.0, 0.0, Flag::zero());
			disp.draw_rotated_bitmap(&bmp, 0.0, 0.0, (disp.get_width() / 2) as f32, (disp.get_height() / 2) as f32, theta, Flag::zero());
			disp.flip();
			redraw = false;
		}

		match q.wait_for_event()
		{
			DisplayClose{source: src, ..} =>
			{
				assert!(disp.get_event_source().get_event_source() == src)
				println!("Display close event...")
				break 'exit;
			},
			KeyDown{keycode: k, ..} if k == key::Escape =>
			{
				println!("Pressed Escape!");
				break 'exit;
			},
			KeyChar{unichar: c, ..} =>
			{
				println!("Entered a character: {}", c);
			},
			TimerTick{..} =>
			{
				redraw = true;
				theta = theta + 0.01;
			},
			MouseButtonDown{button: b, ..} =>
			{
				println!("Mouse button {} pressed", b);
			},
			_ => println!("Some other event...")
		}
	}
}
