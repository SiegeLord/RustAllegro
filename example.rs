extern mod allegro5;

use std::num::Zero;
use std::float::consts::pi;
use allegro5::*;

#[start]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int
{
    allegro5::run(argc, argv, crate_map, main)
}

fn main()
{
	let disp = Display::new(800, 600).unwrap();
	let bmp = Bitmap::new(256, 256).unwrap();

	let mut info = MonitorInfo::new();
	get_monitor_info(0, &mut info);
	println!("{} {} {} {}", info.x1, info.y1, info.x2, info.y2);

	disp.clear_to_color(Color::map_rgb_f(0.0, 0.0, 0.0));
	bmp.clear_to_color(Color::map_rgb_f(0.0, 0.0, 1.0));

	disp.draw_rotated_bitmap(&bmp, 0.0, 0.0, disp.get_width() / 2.0, disp.get_height() / 2.0, pi / 4.0, Zero::zero());
	disp.flip();

	rest(2.0);
}
