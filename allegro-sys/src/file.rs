use crate::path::*;
use crate::utf8::*;

use libc::*;

use allegro_util::c_bool;

allegro_util::opaque!(ALLEGRO_FILE);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ALLEGRO_FILE_INTERFACE
{
	pub fi_fopen:
		Option<unsafe extern "C" fn(path: *const c_char, mode: *const c_char) -> *mut c_void>,
	pub fi_fclose: Option<unsafe extern "C" fn(handle: *mut ALLEGRO_FILE) -> c_bool>,
	pub fi_fread: Option<
		unsafe extern "C" fn(f: *mut ALLEGRO_FILE, ptr: *mut c_void, size: c_ulong) -> c_ulong,
	>,
	pub fi_fwrite: Option<
		unsafe extern "C" fn(f: *mut ALLEGRO_FILE, ptr: *const c_void, size: c_ulong) -> c_ulong,
	>,
	pub fi_fflush: Option<unsafe extern "C" fn(f: *mut ALLEGRO_FILE) -> c_bool>,
	pub fi_ftell: Option<unsafe extern "C" fn(f: *mut ALLEGRO_FILE) -> i64>,
	pub fi_fseek:
		Option<unsafe extern "C" fn(f: *mut ALLEGRO_FILE, offset: i64, whence: c_int) -> c_bool>,
	pub fi_feof: Option<unsafe extern "C" fn(f: *mut ALLEGRO_FILE) -> c_bool>,
	pub fi_ferror: Option<unsafe extern "C" fn(f: *mut ALLEGRO_FILE) -> c_int>,
	pub fi_ferrmsg: Option<unsafe extern "C" fn(f: *mut ALLEGRO_FILE) -> *const c_char>,
	pub fi_fclearerr: Option<unsafe extern "C" fn(f: *mut ALLEGRO_FILE)>,
	pub fi_fungetc: Option<unsafe extern "C" fn(f: *mut ALLEGRO_FILE, c: c_int) -> c_int>,
	pub fi_fsize: Option<unsafe extern "C" fn(f: *mut ALLEGRO_FILE) -> off_t>,
}

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum ALLEGRO_SEEK
{
	ALLEGRO_SEEK_SET = 0,
	ALLEGRO_SEEK_CUR = 1,
	ALLEGRO_SEEK_END = 2,
}

unsafe extern "C" {
	pub fn al_fopen(path: *const c_char, mode: *const c_char) -> *mut ALLEGRO_FILE;
	pub fn al_fopen_interface(
		vt: *const ALLEGRO_FILE_INTERFACE, path: *const c_char, mode: *const c_char,
	) -> *mut ALLEGRO_FILE;
	pub fn al_create_file_handle(
		vt: *const ALLEGRO_FILE_INTERFACE, userdata: *mut c_void,
	) -> *mut ALLEGRO_FILE;
	pub fn al_fclose(f: *mut ALLEGRO_FILE) -> c_bool;
	pub fn al_fread(f: *mut ALLEGRO_FILE, ptr: *mut c_void, size: c_ulong) -> c_ulong;
	pub fn al_fwrite(f: *mut ALLEGRO_FILE, ptr: *const c_void, size: c_ulong) -> c_ulong;
	pub fn al_fflush(f: *mut ALLEGRO_FILE) -> c_bool;
	pub fn al_ftell(f: *mut ALLEGRO_FILE) -> i64;
	pub fn al_fseek(f: *mut ALLEGRO_FILE, offset: i64, whence: c_int) -> c_bool;
	pub fn al_feof(f: *mut ALLEGRO_FILE) -> c_bool;
	pub fn al_ferror(f: *mut ALLEGRO_FILE) -> c_int;
	pub fn al_ferrmsg(f: *mut ALLEGRO_FILE) -> *const c_char;
	pub fn al_fclearerr(f: *mut ALLEGRO_FILE);
	pub fn al_fungetc(f: *mut ALLEGRO_FILE, c: c_int) -> c_int;
	pub fn al_fsize(f: *mut ALLEGRO_FILE) -> i64;
	pub fn al_fgetc(f: *mut ALLEGRO_FILE) -> c_int;
	pub fn al_fputc(f: *mut ALLEGRO_FILE, c: c_int) -> c_int;
	pub fn al_fread16le(f: *mut ALLEGRO_FILE) -> i16;
	pub fn al_fread16be(f: *mut ALLEGRO_FILE) -> i16;
	pub fn al_fwrite16le(f: *mut ALLEGRO_FILE, w: i16) -> c_ulong;
	pub fn al_fwrite16be(f: *mut ALLEGRO_FILE, w: i16) -> c_ulong;
	pub fn al_fread32le(f: *mut ALLEGRO_FILE) -> i32;
	pub fn al_fread32be(f: *mut ALLEGRO_FILE) -> i32;
	pub fn al_fwrite32le(f: *mut ALLEGRO_FILE, l: i32) -> c_ulong;
	pub fn al_fwrite32be(f: *mut ALLEGRO_FILE, l: i32) -> c_ulong;
	pub fn al_fgets(f: *mut ALLEGRO_FILE, p: *mut c_char, max: c_ulong) -> *mut c_char;
	pub fn al_fget_ustr(f: *mut ALLEGRO_FILE) -> *mut ALLEGRO_USTR;
	pub fn al_fputs(f: *mut ALLEGRO_FILE, p: *const c_char) -> c_int;
	pub fn al_fprintf(f: *mut ALLEGRO_FILE, format: *const c_char, ...) -> c_int;
	pub fn al_fopen_fd(fd: c_int, mode: *const c_char) -> *mut ALLEGRO_FILE;
	pub fn al_make_temp_file(
		tmpl: *const c_char, ret_path: *mut *mut ALLEGRO_PATH,
	) -> *mut ALLEGRO_FILE;
	pub fn al_fopen_slice(
		fp: *mut ALLEGRO_FILE, initial_size: c_ulong, mode: *const c_char,
	) -> *mut ALLEGRO_FILE;
	pub fn al_get_new_file_interface() -> *const ALLEGRO_FILE_INTERFACE;
	pub fn al_set_new_file_interface(file_interface: *const ALLEGRO_FILE_INTERFACE);
	pub fn al_set_standard_file_interface();
	pub fn al_get_file_userdata(f: *mut ALLEGRO_FILE) -> *mut c_void;
}
