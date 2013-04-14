extern mod allegro;

use allegro::*;

fn main()
{
	if(!al_init())
	{
		fail!(~"Failed to initialize Allegro");
	}

	io::println(fmt!("Allegro version is %?", ALLEGRO_VERSION_STR));

	let d = ~al_create_display(800, 600).expect("creating display");

	io::println(fmt!("Display dimensions: %?", (al_get_display_width(d), al_get_display_height(d))));

	let bmp = ~al_create_bitmap(300, 200).expect("creating bitmap");

	io::println(fmt!("Bitmap dimensions: %?", (al_get_bitmap_width(bmp), al_get_bitmap_height(bmp))));

	al_rest(2.0);
	io::println("Done!");
}
