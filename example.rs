extern mod allegro;

use allegro::*;

fn main()
{
	if(!al_init())
	{
		fail!(~"Failed to initialize Allegro");
	}

	io::println(fmt!("Allegro version is %?", ALLEGRO_VERSION_STR));

	let _a = ~al_create_display(800, 600).expect("creating display");

	al_rest(2.0);
	io::println("Done!");
}
