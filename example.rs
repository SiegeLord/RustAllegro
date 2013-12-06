#[feature(globs)];
#[feature(struct_variant)];

extern mod allegro5;

use std::num::Zero;
use std::c_str::*;
use allegro5::*;

#[start]
fn start(argc: int, argv: **u8) -> int
{
    allegro5::run(argc, argv, main)
}

fn main()
{
	let mut core = Core::init().expect("Your Allegro version does not match this Rust binding");

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

	let mut info = MonitorInfo::new();
	core.get_monitor_info(0, &mut info);
	println!("{} {} {} {}", info.x1, info.y1, info.x2, info.y2);

	bmp.clear_to_color(core.map_rgb_f(0.0, 0.0, 1.0));

	let sub_bmp = bmp.create_sub_bitmap(64, 64, 64, 64).unwrap();
	sub_bmp.clear_to_color(core.map_rgb_f(0.0, 1.0, 1.0));

	let mut theta = 0.0f32;
	let mut redraw = true;
	timer.start();
	'exit: loop
	{
		if redraw && q.is_empty()
		{
			disp.clear_to_color(core.map_rgb_f(0.0, 0.0, 0.0));
			disp.draw_rotated_bitmap(&bmp, 0.0, 0.0, (disp.get_width() / 2) as f32, (disp.get_height() / 2) as f32, theta, Zero::zero());
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
