use std::libc::*;

pub static ALLEGRO_VERSION: u32          = 5;
pub static ALLEGRO_SUB_VERSION: u32      = 0;
pub static ALLEGRO_WIP_VERSION: u32      = 10;
pub static ALLEGRO_RELEASE_NUMBER: u32   = 0;

pub static ALLEGRO_VERSION_STR: &'static str = "5.0.10";
pub static ALLEGRO_DATE_STR: &'static str    = "2013";
pub static ALLEGRO_DATE: u32                 = 20130616;
pub static ALLEGRO_VERSION_INT: u32          = ((ALLEGRO_VERSION << 24) | (ALLEGRO_SUB_VERSION << 16) | (ALLEGRO_WIP_VERSION << 8) | ALLEGRO_RELEASE_NUMBER);

externfn!(fn al_get_allegro_version() -> uint32_t)
externfn!(fn al_run_main(argc: c_int, argv: **c_char, user_main: extern "C" fn(argc: c_int, argv: **c_char) -> c_int) -> c_int)

pub static ALLEGRO_PI: f64 = 3.14159265358979323846;

fn AL_ID(a: u8, b: u8, c: u8, d: u8) -> u32
{
	return (((a as u32)<<24) | ((b as u32)<<16) | ((c as u32)<<8) | (d as u32));
}