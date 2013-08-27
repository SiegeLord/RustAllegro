use std::libc::*;

use ffi::color::*;

externfn!(fn al_clear_to_color(color: ALLEGRO_COLOR))
externfn!(fn al_draw_pixel(x: c_float, y: c_float, color: ALLEGRO_COLOR))
