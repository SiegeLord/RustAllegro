// This file is released into Public Domain.
use allegro::*;
use allegro_primitives::*;
use getopts::*;
use std::env;

#[cfg(any(allegro_5_2_0, allegro_5_1_0))]
allegro_main! {
	let args = env::args().collect::<Vec<_>>();

	let mut opts = Options::new();
	opts.optflag("i", "init-only", "only initialize Allegro, don't do anything else");
	let matches = opts.parse(&args[1..]).unwrap();

	let init_only = matches.opt_present("i");

	let core = Core::init().unwrap();
	let prim = PrimitivesAddon::init(&core).unwrap();

	if init_only
	{
		return;
	}

	let mut disp = Display::new(&core, 800, 600).unwrap();
	disp.set_window_title("Vertex Buffer example");

	let timer = Timer::new(&core, 1.0 / 60.0).unwrap();

	let q = EventQueue::new(&core).unwrap();
	q.register_event_source(disp.get_event_source());
	q.register_event_source(timer.get_event_source());

	let black = Color::from_rgb_f(0.0, 0.0, 0.0);

	let mut vertex_buffer = VertexBuffer::<Vertex>::new(&mut disp, &prim, None, 4, BUFFER_STREAM).unwrap();
	let mut index_buffer = IndexBuffer::<u32>::new(&mut disp, &prim, None, 6, BUFFER_STREAM).unwrap();

	{
		let mut lock = vertex_buffer.lock_write_only(0, 4).unwrap();
		lock.copy_from_slice(&[
			Vertex { x: 0.0, y: 0.0, z: 0.0, color: Color::from_rgb_f(1., 0., 0.), u: 0., v: 0., },
			Vertex { x: 500.0, y: 0.0, z: 0.0, color: Color::from_rgb_f(1., 1., 0.), u: 0., v: 0., },
			Vertex { x: 500.0, y: 500.0, z: 0.0, color: Color::from_rgb_f(0., 1., 1.), u: 0., v: 0., },
			Vertex { x: 0.0, y: 500.0, z: 0.0, color: Color::from_rgb_f(0., 0., 1.), u: 0., v: 0., },
		]);
	}
	{
		let mut lock = index_buffer.lock_write_only(0, 6).unwrap();
		lock.copy_from_slice(&[0, 1, 2, 0, 2, 3]);
	}

	let mut redraw = true;
	timer.start();
	'exit: loop
	{
		if redraw && q.is_empty()
		{
			core.clear_to_color(black);

			prim.draw_indexed_buffer(&vertex_buffer, Option::<&Bitmap>::None, &index_buffer, 0, 6, PrimType::TriangleList);

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

#[cfg(not(any(allegro_5_2_0, allegro_5_1_0)))]
fn main()
{
	panic!("This example needs at least Allegro 5.1");
}
