// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;

use allegro_util::c_bool;

#[cfg(windows)]
pub const ALLEGRO_NATIVE_PATH_SEP: char = '\\';
#[cfg(windows)]
pub const ALLEGRO_NATIVE_DRIVE_SEP: char = ':';

#[cfg(not(windows))]
pub const ALLEGRO_NATIVE_PATH_SEP: char = '/';
#[cfg(not(windows))]
pub const ALLEGRO_NATIVE_DRIVE_SEP: char = '\x00';

opaque!(ALLEGRO_PATH);

extern "C"
{
	pub fn al_create_path(str: *const c_char) -> *mut ALLEGRO_PATH;
	pub fn al_create_path_for_directory(str: *const c_char) -> *mut ALLEGRO_PATH;
	pub fn al_clone_path(path: *const ALLEGRO_PATH) -> *mut ALLEGRO_PATH;

	pub fn al_get_path_num_components(path: *const ALLEGRO_PATH) -> c_int;
	pub fn al_get_path_component(path: *const ALLEGRO_PATH, i: c_int) -> *const c_char;
	pub fn al_replace_path_component(path: *mut ALLEGRO_PATH, i: c_int, s: *const c_char);
	pub fn al_remove_path_component(path: *mut ALLEGRO_PATH, i: c_int);
	pub fn al_insert_path_component(path: *mut ALLEGRO_PATH, i: c_int, s: *const c_char);
	pub fn al_get_path_tail(path: *const ALLEGRO_PATH) -> *const c_char;
	pub fn al_drop_path_tail(path: *mut ALLEGRO_PATH);
	pub fn al_append_path_component(path: *mut ALLEGRO_PATH, s: *const c_char);
	pub fn al_join_paths(path: *const ALLEGRO_PATH, tail: *const ALLEGRO_PATH) -> c_bool;
	pub fn al_rebase_path(head: *const ALLEGRO_PATH, tail: *const ALLEGRO_PATH) -> c_bool;
	pub fn al_path_cstr(path: *const ALLEGRO_PATH, delim: c_char) -> *const c_char;
	pub fn al_destroy_path(path: *const ALLEGRO_PATH);

	pub fn al_set_path_drive(path: *mut ALLEGRO_PATH, drive: *const c_char);
	pub fn al_get_path_drive(path: *const ALLEGRO_PATH) -> *const c_char;

	pub fn al_set_path_filename(path: *mut ALLEGRO_PATH, filename: *const c_char);
	pub fn al_get_path_filename(path: *const ALLEGRO_PATH) -> *const c_char;

	pub fn al_get_path_extension(path: *const ALLEGRO_PATH) -> *const c_char;
	pub fn al_set_path_extension(path: *mut ALLEGRO_PATH, extension: *const c_char) ->	c_bool;
	pub fn al_get_path_basename(path: *const ALLEGRO_PATH) -> *const c_char;

	pub fn al_make_path_canonical(path: *mut ALLEGRO_PATH) -> c_bool;
}
