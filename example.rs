extern mod allegro5;

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

	disp.clear_to_color(Color::map_rgb_f(0.0, 0.0, 0.0));
	bmp.clear_to_color(Color::map_rgb_f(0.0, 0.0, 1.0));

	disp.draw_bitmap(&bmp, 100.0, 100.0, 0);
	disp.flip();

	rest(2.0);
}
