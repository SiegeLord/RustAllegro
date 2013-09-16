extern mod allegro5;

use std::num::Zero;
use std::float::consts::pi;
use std::c_str::*;
use allegro5::*;

#[start]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int
{
    allegro5::run(argc, argv, crate_map, main)
}

fn main()
{
	let disp = Display::new(800, 600).unwrap();
	disp.set_window_title(&"Rust example".to_c_str());

	let q = EventQueue::new().unwrap();
	q.register_event_source(disp.get_event_source());

	let bmp = Bitmap::new(256, 256).unwrap();

	let mut info = MonitorInfo::new();
	get_monitor_info(0, &mut info);
	println!("{} {} {} {}", info.x1, info.y1, info.x2, info.y2);

	disp.clear_to_color(Color::map_rgb_f(0.0, 0.0, 0.0));
	bmp.clear_to_color(Color::map_rgb_f(0.0, 0.0, 1.0));

	let sub_bmp = bmp.create_sub_bitmap(64, 64, 64, 64).unwrap();
	sub_bmp.clear_to_color(Color::map_rgb_f(0.0, 1.0, 1.0));

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
