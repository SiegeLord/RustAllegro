#[link(name = "allegro5",
       vers = "5.0.10.1",
       author = "SiegeLord",
       url = "https://github.com/SiegeLord/RustAllegro")];

#[comment = "Allegro 5 core library Rust bindings"];
#[license = "zlib"];
#[crate_type = "lib"];

pub use bitmap::*;
pub use bitmap_like::*;
pub use color::*;
pub use core::*;
pub use core_drawing::*;
pub use display::*;
pub use events::*;
pub use keycodes::*;
pub use run::*;
pub use timer::*;

pub mod ffi;
#[macro_escape]
pub mod rust_util;

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
