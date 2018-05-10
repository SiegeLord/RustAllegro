// This file is released into Public Domain.
#[macro_use]
extern crate allegro;
extern crate allegro_image;
extern crate getopts;

use allegro::*;
use allegro_image::*;
use getopts::*;
use std::env;

#[cfg(any(allegro_5_2_0, allegro_5_1_0))]
allegro_main!
{
	let args = env::args().collect::<Vec<_>>();

	let mut opts = Options::new();
	opts.optflag("i", "init-only", "only initialize Allegro, don't do anything else");
	let matches = opts.parse(&args[1..]).unwrap();

	let init_only = matches.opt_present("i");

	let core = Core::init().unwrap();
	ImageAddon::init(&core).unwrap();

	if init_only
	{
		return;
	}

	core.set_new_display_flags(OPENGL | PROGRAMMABLE_PIPELINE);
	let mut disp = Display::new(&core, 800, 600).unwrap();
	disp.set_window_title("Shader example");

	let timer = Timer::new(&core, 1.0 / 60.0).unwrap();

	let q = EventQueue::new(&core).unwrap();
	q.register_event_source(disp.get_event_source());
	q.register_event_source(timer.get_event_source());

	let buffer = Bitmap::new(&core, 800, 600).unwrap();
	let bmp = Bitmap::load(&core, "data/mysha.pcx").unwrap();
	let black = Color::from_rgb_f(0.0, 0.0, 0.0);

	let shader = disp.create_shader(ShaderPlatform::GLSL).unwrap();

	shader.upgrade().unwrap().attach_shader_source(ShaderType::Vertex, Some(
	"
	attribute vec4 al_pos;
	attribute vec4 al_color;
	attribute vec2 al_texcoord;
	uniform mat4 al_projview_matrix;
	varying vec4 varying_color;
	varying vec2 varying_texcoord;
	void main()
	{
		varying_color = al_color;
		varying_texcoord = al_texcoord;
		gl_Position = al_projview_matrix * al_pos;
	}
	")).unwrap();
	shader.upgrade().unwrap().attach_shader_source(ShaderType::Pixel, Some(
	"
	#ifdef GL_ES
	precision mediump float;
	#endif
	uniform sampler2D al_tex;
	uniform vec3 tint;
	varying vec4 varying_color;
	varying vec2 varying_texcoord;
	void main()
	{
		vec4 tmp = varying_color * texture2D(al_tex, varying_texcoord);
		tmp.r *= tint.r;
		tmp.g *= tint.g;
		tmp.b *= tint.b;
		gl_FragColor = tmp;
	}
	")).unwrap();
	shader.upgrade().unwrap().build().unwrap();

	let tint = vec![[1.0f32, 0.0, 0.0]];
	let mut redraw = true;
	timer.start();
	'exit: loop
	{
		if redraw && q.is_empty()
		{
			core.set_target_bitmap(Some(&buffer));
			core.clear_to_color(black);
			core.use_shader(Some(&*shader.upgrade().unwrap())).unwrap();
			core.set_shader_uniform("tint", &tint[..]).unwrap();
			core.draw_bitmap(&bmp, 0.0, 0.0, Flag::zero());

			core.set_target_bitmap(Some(disp.get_backbuffer()));
			core.draw_bitmap(&buffer, 0.0, 0.0, Flag::zero());

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
