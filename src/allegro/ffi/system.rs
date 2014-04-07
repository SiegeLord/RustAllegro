use libc::*;

use ffi::path::*;
use rust_util::c_bool;

opaque!(ALLEGRO_SYSTEM)

extern "C"
{
	pub fn al_install_system(version: c_int, atexit_ptr: Option<extern "C" fn(atexit_ptr: extern "C" fn()) -> c_int>) -> c_bool;
	pub fn al_uninstall_system();
	pub fn al_is_system_installed() -> c_bool;
	pub fn al_get_system_driver() -> *mut ALLEGRO_SYSTEM;
//~ 	pub fn al_get_system_config() -> *mut ALLEGRO_CONFIG;
}

pub static ALLEGRO_RESOURCES_PATH: i32 = 0;
pub static ALLEGRO_TEMP_PATH: i32 = 1;
pub static ALLEGRO_USER_DATA_PATH: i32 = 2;
pub static ALLEGRO_USER_HOME_PATH: i32 = 3;
pub static ALLEGRO_USER_SETTINGS_PATH: i32 = 4;
pub static ALLEGRO_USER_DOCUMENTS_PATH: i32 = 5;
pub static ALLEGRO_EXENAME_PATH: i32 = 6;
pub static ALLEGRO_LAST_PATH: i32 = 7;

extern "C"
{
	pub fn al_get_standard_path(id: c_int) -> *mut ALLEGRO_PATH;
	pub fn al_set_exe_name(path: *c_schar);

	pub fn al_set_org_name(org_name: *c_schar);
	pub fn al_set_app_name(app_name: *c_schar);
	pub fn al_get_org_name() -> *c_schar;
	pub fn al_get_app_name() -> *c_schar;

	pub fn al_inhibit_screensaver(inhibit: c_uchar) -> c_bool;
}
