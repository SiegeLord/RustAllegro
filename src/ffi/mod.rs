pub use ffi::altime::*;
pub use ffi::base::*;
pub use ffi::bitmap::*;
pub use ffi::bitmap_draw::*;
pub use ffi::color::*;
pub use ffi::display::*;
pub use ffi::drawing::*;
pub use ffi::events::*;
pub use ffi::system::*;

#[link_args = "-lallegro"]
extern "C" {}

pub mod altime;
pub mod base;
pub mod color;
pub mod bitmap;
pub mod bitmap_draw;
pub mod display;
pub mod drawing;
pub mod events;
pub mod path;
pub mod system;

pub mod rust_util;
