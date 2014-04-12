// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;

use rust_util::c_bool;

#[cfg(windows)]
pub static ALLEGRO_NATIVE_PATH_SEP: char = '\\';
#[cfg(windows)]
pub static ALLEGRO_NATIVE_DRIVE_SEP: char = ':';

#[cfg(not(windows))]
pub static ALLEGRO_NATIVE_PATH_SEP: char = '/';
#[cfg(not(windows))]
pub static ALLEGRO_NATIVE_DRIVE_SEP: char = '\x00';

opaque!(ALLEGRO_PATH)

extern "C"
{
	pub fn al_create_path(str: *c_schar) -> *mut ALLEGRO_PATH;
	pub fn al_create_path_for_directory(str: *c_schar) -> *mut ALLEGRO_PATH;
	pub fn al_clone_path(path: *ALLEGRO_PATH) -> *mut ALLEGRO_PATH;

	pub fn al_get_path_num_components(path: *ALLEGRO_PATH) -> c_int;
	pub fn al_get_path_component(path: *ALLEGRO_PATH, i: c_int) -> *c_schar;
	pub fn al_replace_path_component(path: *mut ALLEGRO_PATH, i: c_int, s: *c_schar);
	pub fn al_remove_path_component(path: *mut ALLEGRO_PATH, i: c_int);
	pub fn al_insert_path_component(path: *mut ALLEGRO_PATH, i: c_int, s: *c_schar);
	pub fn al_get_path_tail(path: *ALLEGRO_PATH) -> *c_schar;
	pub fn al_drop_path_tail(path: *mut ALLEGRO_PATH);
	pub fn al_append_path_component(path: *mut ALLEGRO_PATH, s: *c_schar);
	pub fn al_join_paths(path: *ALLEGRO_PATH, tail: *ALLEGRO_PATH) -> c_bool;
	pub fn al_rebase_path(head: *ALLEGRO_PATH, tail: *ALLEGRO_PATH) -> c_bool;
	pub fn al_path_cstr(path: *ALLEGRO_PATH, delim: c_schar) -> *c_schar;
	pub fn al_destroy_path(path: *ALLEGRO_PATH);

	pub fn al_set_path_drive(path: *mut ALLEGRO_PATH, drive: *c_schar);
	pub fn al_get_path_drive(path: *ALLEGRO_PATH) -> *c_schar;

	pub fn al_set_path_filename(path: *mut ALLEGRO_PATH, filename: *c_schar);
	pub fn al_get_path_filename(path: *ALLEGRO_PATH) -> *c_schar;

	pub fn al_get_path_extension(path: *ALLEGRO_PATH) -> *c_schar;
	pub fn al_set_path_extension(path: *mut ALLEGRO_PATH, extension: *c_schar) ->	c_bool;
	pub fn al_get_path_basename(path: *ALLEGRO_PATH) -> *c_schar;

	pub fn al_make_path_canonical(path: *mut ALLEGRO_PATH) -> c_bool;
}
