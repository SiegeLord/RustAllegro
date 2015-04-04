// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;

use bitmap::*;
use allegro_util::c_bool;

pub type ALLEGRO_IIO_LOADER_FUNCTION =  extern "C" fn(arg1: *const c_char) -> *mut ALLEGRO_BITMAP;
//~ pub type ALLEGRO_IIO_FS_LOADER_FUNCTION = extern "C" fn(arg1: *mut ALLEGRO_FILE) -> *mut ALLEGRO_BITMAP;
pub type ALLEGRO_IIO_SAVER_FUNCTION = extern "C" fn(arg1: *const c_char, arg2: *mut ALLEGRO_BITMAP) -> c_bool;
//~ pub type ALLEGRO_IIO_FS_SAVER_FUNCTION = extern "C" fn(arg1: *mut ALLEGRO_FILE, arg2: *mut ALLEGRO_BITMAP) -> c_bool;

extern "C" {
    pub fn al_register_bitmap_loader(ext: *const c_char, loader: ALLEGRO_IIO_LOADER_FUNCTION) -> c_bool;
    pub fn al_register_bitmap_saver(ext: *const c_char, saver: ALLEGRO_IIO_SAVER_FUNCTION) -> c_bool;
    //~ pub fn al_register_bitmap_loader_f(ext: *const c_char, fs_loader: ALLEGRO_IIO_FS_LOADER_FUNCTION) -> c_bool;
    //~ pub fn al_register_bitmap_saver_f(ext: *const c_char, fs_saver: ALLEGRO_IIO_FS_SAVER_FUNCTION) -> c_bool;
    pub fn al_load_bitmap(filename: *const c_char) -> *mut ALLEGRO_BITMAP;
    //~ pub fn al_load_bitmap_f(fp: *mut ALLEGRO_FILE, ident: *const c_char) -> *mut ALLEGRO_BITMAP;
    pub fn al_save_bitmap(filename: *const c_char, bitmap: *mut ALLEGRO_BITMAP) -> c_bool;
    //~ pub fn al_save_bitmap_f(fp: *mut ALLEGRO_FILE, ident: *const c_char, bitmap: *mut ALLEGRO_BITMAP) -> c_bool;
}
