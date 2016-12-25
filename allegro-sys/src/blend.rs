use libc::*;

use color::ALLEGRO_COLOR;

pub const ALLEGRO_ZERO: u32 = 0;
pub const ALLEGRO_ONE: u32 = 1;
pub const ALLEGRO_ALPHA: u32 = 2;
pub const ALLEGRO_INVERSE_ALPHA: u32 = 3;
pub const ALLEGRO_SRC_COLOR: u32 = 4;
pub const ALLEGRO_DEST_COLOR: u32 = 5;
pub const ALLEGRO_INVERSE_SRC_COLOR: u32 = 6;
pub const ALLEGRO_INVERSE_DEST_COLOR: u32 = 7;
pub const ALLEGRO_CONST_COLOR: u32 = 8;
pub const ALLEGRO_INVERSE_CONST_COLOR: u32 = 9;
pub const ALLEGRO_NUM_BLEND_MODES: u32 = 10;

pub const ALLEGRO_ADD: u32 = 0;
pub const ALLEGRO_SRC_MINUS_DEST: u32 = 1;
pub const ALLEGRO_DEST_MINUS_SRC: u32 = 2;
pub const ALLEGRO_NUM_BLEND_OPERATIONS: u32 = 3;

extern "C" {
    pub fn al_set_blender(op: c_int,
                          source: c_int,
                          dest: c_int);
    pub fn al_set_blend_color(color: ALLEGRO_COLOR);
    pub fn al_get_blender(op: *mut c_int,
                          source: *mut c_int,
                          dest: *mut c_int);
    pub fn al_get_blend_color() -> ALLEGRO_COLOR;
    pub fn al_set_separate_blender(op: c_int,
                                   source: c_int,
                                   dest: c_int,
                                   alpha_op: c_int,
                                   alpha_source: c_int,
                                   alpha_dest: c_int);
    pub fn al_get_separate_blender(op: *mut c_int,
                                   source: *mut c_int,
                                   dest: *mut c_int,
                                   alpha_op: *mut c_int,
                                   alpha_src: *mut c_int,
                                   alpha_dest: *mut c_int);
}
