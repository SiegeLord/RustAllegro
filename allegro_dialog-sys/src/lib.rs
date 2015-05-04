// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name="allegro_dialog_sys"]
#![crate_type = "lib"]

extern crate allegro_sys;
#[macro_use]
extern crate allegro_util;
extern crate libc;

pub use allegro_dialog::*;

pub mod allegro_dialog
{
	#![allow(non_camel_case_types)]

	use libc::*;
	use allegro_util::c_bool;
	use allegro_sys::{ALLEGRO_DISPLAY, ALLEGRO_EVENT_SOURCE};

	opaque!(ALLEGRO_FILECHOOSER);
	opaque!(ALLEGRO_TEXTLOG);

	pub const ALLEGRO_FILECHOOSER_FILE_MUST_EXIST: u32 = 1;
	pub const ALLEGRO_FILECHOOSER_SAVE: u32 = 2;
	pub const ALLEGRO_FILECHOOSER_FOLDER: u32 = 4;
	pub const ALLEGRO_FILECHOOSER_PICTURES: u32 = 8;
	pub const ALLEGRO_FILECHOOSER_SHOW_HIDDEN: u32 = 16;
	pub const ALLEGRO_FILECHOOSER_MULTIPLE: u32 = 32;

	pub const ALLEGRO_MESSAGEBOX_WARN: u32 = 1;
	pub const ALLEGRO_MESSAGEBOX_ERROR: u32 = 2;
	pub const ALLEGRO_MESSAGEBOX_OK_CANCEL: u32 = 4;
	pub const ALLEGRO_MESSAGEBOX_YES_NO: u32 = 8;
	pub const ALLEGRO_MESSAGEBOX_QUESTION: u32 = 16;

	pub const ALLEGRO_TEXTLOG_NO_CLOSE: u32 = 1;
	pub const ALLEGRO_TEXTLOG_MONOSPACE: u32 = 2;

	pub const ALLEGRO_EVENT_NATIVE_DIALOG_CLOSE: c_uint = 600;

	extern "C"
	{
		pub fn al_init_native_dialog_addon() -> c_bool;
		pub fn al_shutdown_native_dialog_addon();
		pub fn al_create_native_file_dialog(initial_path: *const c_char, title: *const c_char, patterns: *const c_char, mode: c_int) -> *mut ALLEGRO_FILECHOOSER;
		pub fn al_show_native_file_dialog(display: *mut ALLEGRO_DISPLAY, dialog: *mut ALLEGRO_FILECHOOSER) -> c_bool;
		pub fn al_get_native_file_dialog_count(dialog: *const ALLEGRO_FILECHOOSER) -> c_int;
		pub fn al_get_native_file_dialog_path(dialog: *const ALLEGRO_FILECHOOSER, index: size_t) -> *const c_char;
		pub fn al_destroy_native_file_dialog(dialog: *mut ALLEGRO_FILECHOOSER);
		pub fn al_show_native_message_box(display: *mut ALLEGRO_DISPLAY, title: *const c_char, heading: *const c_char, text: *const c_char, buttons: *const c_char, flags: c_int) -> c_int;
		pub fn al_open_native_text_log(title: *const c_char, flags: c_int) -> *mut ALLEGRO_TEXTLOG;
		pub fn al_close_native_text_log(textlog: *mut ALLEGRO_TEXTLOG);
		pub fn al_append_native_text_log(textlog: *mut ALLEGRO_TEXTLOG, format: *const c_char, ...);
		pub fn al_get_native_text_log_event_source(textlog: *mut ALLEGRO_TEXTLOG) -> *mut ALLEGRO_EVENT_SOURCE;
		pub fn al_get_allegro_native_dialog_version() -> uint32_t;
	}
}
