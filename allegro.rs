#[link(name = "allegro",
       vers = "5.1.6",
       author = "SiegeLord",
       url = "https://github.com/SiegeLord/RustAllegro")];

#[comment = "Allegro 5 core library Rust bindings"];
#[license = "zlib"];
#[crate_type = "lib"];

pub use allegro5::altime::*;
pub use allegro5::base::*;
pub use allegro5::display::*;
pub use allegro5::system::*;

#[link_args = "-lallegro"]
extern "C" {}

mod allegro5
{
	mod altime;
	mod base;
	mod display;
	mod system;
}
