// This file is released into Public Domain.
use allegro::*;

#[derive(Debug)]
struct Foo;

impl Drop for Foo
{
	fn drop(&mut self)
	{
		println!("Foo destructor");
	}
}

allegro_main! {
	let core = Core::init().unwrap();

	let mut source = UserEventSource::new(&core);
	let q = EventQueue::new(&core).unwrap();
	q.register_event_source(&mut source);
	source.emit(1i32);
	source.emit(2f32);
	source.emit(Foo);

	'exit: loop
	{
		let e = q.get_next_event();
		match e
		{
			User{timestamp, ref data, ..} =>
			{
				println!("{}", timestamp);
				if let Some(data) = data.downcast_ref()
				{
					println!("{:?}", data as &i32);
				}
				if let Some(data) = data.downcast_ref()
				{
					println!("{:?}", data as &f32);
				}
				if let Some(data) = data.downcast_ref()
				{
					println!("{:?}", data as &Foo);
				}
			},
			_ => break 'exit
		}
	}
}
