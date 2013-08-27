use std::libc::*;

externfn!(fn al_get_time() -> c_double)
externfn!(fn al_rest(seconds: c_double))
