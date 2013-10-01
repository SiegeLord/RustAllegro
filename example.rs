extern mod allegro5;

use std::num::Zero;
use std::f32::consts::pi;
use std::c_str::*;
use allegro5::*;

#[start]
fn start(argc: int, argv: **u8) -> int
{
    allegro5::run(argc, argv, main)
}

fn main()
{
	let core = Core::init().expect("Your Allegro version does not match this Rust binding");

	let disp = core.create_display(800, 600).unwrap();
	disp.set_window_title(&"Rust example".to_c_str());

	let q = core.create_event_queue().unwrap();
	q.register_event_source(disp.get_event_source());

	let bmp = core.create_bitmap(256, 256).unwrap();

	let mut info = MonitorInfo::new();
	core.get_monitor_info(0, &mut info);
	println!("{} {} {} {}", info.x1, info.y1, info.x2, info.y2);

	disp.clear_to_color(core.map_rgb_f(0.0, 0.0, 0.0));
	bmp.clear_to_color(core.map_rgb_f(0.0, 0.0, 1.0));

	let sub_bmp = bmp.create_sub_bitmap(64, 64, 64, 64).unwrap();
	sub_bmp.clear_to_color(core.map_rgb_f(0.0, 1.0, 1.0));

	disp.draw_rotated_bitmap(&bmp, 0.0, 0.0, disp.get_width() / 2.0, disp.get_height() / 2.0, pi / 4.0, Zero::zero());
	disp.flip();

	match q.wait_for_event()
	{
		DisplayClose{ source: src, _} =>
		{
			assert!(disp.get_event_source().get_event_source() == src)
			println!("Display close event...")
		},
		_ => println!("Some other event...")
	}
}
