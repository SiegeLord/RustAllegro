#[link(name = "allegro", vers = "5.1.6", author = "SiegeLord")];
#[crate_type = "lib"];

pub use allegro5::altime::*;
pub use allegro5::base::*;
pub use allegro5::display::*;
pub use allegro5::system::*;

mod allegro5
{
	mod altime;
	mod base;
	mod display;
	mod system;
}

extern mod allegro {}
