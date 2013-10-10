#[link(name = "allegro5",
       vers = "5.0.10.1",
       author = "SiegeLord",
       url = "https://github.com/SiegeLord/RustAllegro")];

#[comment = "Allegro 5 core library Rust bindings"];
#[license = "zlib"];
#[crate_type = "lib"];
#[feature(globs)];
#[feature(macro_rules)];
#[feature(struct_variant)];

pub use internal::bitmap::external::*;
pub use internal::bitmap_like::*;
pub use internal::color::*;
pub use internal::core::*;
pub use internal::core_drawing::*;
pub use internal::display::external::*;
pub use internal::events::external::*;
pub use internal::keycodes::*;
pub use internal::run::*;
pub use internal::timer::external::*;

pub mod ffi;
#[macro_escape]
pub mod rust_util;

mod internal
{
	pub mod bitmap;
	pub mod bitmap_like;
	pub mod color;
	pub mod core;
	pub mod core_drawing;
	pub mod display;
	pub mod events;
	pub mod keycodes;
	pub mod run;
	pub mod timer;
}
