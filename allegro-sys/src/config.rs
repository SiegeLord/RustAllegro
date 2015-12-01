// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;

use allegro_util::c_bool;

opaque!(ALLEGRO_CONFIG_SECTION);
opaque!(ALLEGRO_CONFIG_ENTRY);
opaque!(ALLEGRO_CONFIG);

extern {
	pub fn al_create_config() -> *mut ALLEGRO_CONFIG;
	pub fn al_add_config_section(config: *mut ALLEGRO_CONFIG, name: *const c_char) -> ();
	pub fn al_set_config_value(config: *mut ALLEGRO_CONFIG,
	                           section: *const c_char,
	                           key: *const c_char,
	                           value: *const c_char)
	                           -> ();
	pub fn al_add_config_comment(config: *mut ALLEGRO_CONFIG,
	                             section: *const c_char,
	                             comment: *const c_char)
	                             -> ();
	pub fn al_get_config_value(config: *const ALLEGRO_CONFIG,
	                           section: *const c_char,
	                           key: *const c_char)
	                           -> *const c_char;
	pub fn al_load_config_file(filename: *const c_char) -> *mut ALLEGRO_CONFIG;
	//~ pub fn al_load_config_file_f(filename: *mut ALLEGRO_FILE) -> *mut ALLEGRO_CONFIG;
	pub fn al_save_config_file(filename: *const c_char, config: *const ALLEGRO_CONFIG) -> c_bool;
	//~ pub fn al_save_config_file_f(file: *mut ALLEGRO_FILE,
	                             //~ config: *const ALLEGRO_CONFIG)
	                             //~ -> c_bool;
	pub fn al_merge_config_into(master: *mut ALLEGRO_CONFIG, add: *const ALLEGRO_CONFIG) -> ();
	pub fn al_merge_config(cfg1: *const ALLEGRO_CONFIG,
	                       cfg2: *const ALLEGRO_CONFIG)
	                       -> *mut ALLEGRO_CONFIG;
	pub fn al_destroy_config(config: *mut ALLEGRO_CONFIG) -> ();
	pub fn al_remove_config_section(config: *mut ALLEGRO_CONFIG,
	                                section: *const c_char)
	                                -> c_bool;
	pub fn al_remove_config_key(config: *mut ALLEGRO_CONFIG,
	                            section: *const c_char,
	                            key: *const c_char)
	                            -> c_bool;
	pub fn al_get_first_config_section(config: *const ALLEGRO_CONFIG,
	                                   iterator: *mut *mut ALLEGRO_CONFIG_SECTION)
	                                   -> *const c_char;
	pub fn al_get_next_config_section(iterator: *mut *mut ALLEGRO_CONFIG_SECTION) -> *const c_char;
	pub fn al_get_first_config_entry(config: *const ALLEGRO_CONFIG,
	                                 section: *const c_char,
	                                 iterator: *mut *mut ALLEGRO_CONFIG_ENTRY)
	                                 -> *const c_char;
	pub fn al_get_next_config_entry(iterator: *mut *mut ALLEGRO_CONFIG_ENTRY) -> *const c_char;
}
