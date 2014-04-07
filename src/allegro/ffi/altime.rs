use libc::*;

pub struct ALLEGRO_TIMEOUT
{
	__pad1__: uint64_t,
	__pad2__: uint64_t,
}

extern "C"
{
	pub fn al_get_time() -> c_double;
	pub fn al_rest(seconds: c_double);
	pub fn al_init_timeout(timeout: *mut ALLEGRO_TIMEOUT, seconds: c_double);
}
