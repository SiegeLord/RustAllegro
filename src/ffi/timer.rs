use std::libc::*;
use rust_util::c_bool;

use ffi::events::ALLEGRO_EVENT_SOURCE;

pub struct ALLEGRO_TIMER;

externfn!(fn al_create_timer(speed_secs: c_double) -> *mut ALLEGRO_TIMER)
externfn!(fn al_destroy_timer(timer: *mut ALLEGRO_TIMER))
externfn!(fn al_start_timer(timer: *mut ALLEGRO_TIMER))
externfn!(fn al_stop_timer(timer: *mut ALLEGRO_TIMER))
externfn!(fn al_get_timer_started(timer: *ALLEGRO_TIMER) -> c_bool)
externfn!(fn al_get_timer_speed(timer: *ALLEGRO_TIMER) -> c_double)
externfn!(fn al_set_timer_speed(timer: *mut ALLEGRO_TIMER, speed_secs: c_double))
externfn!(fn al_get_timer_count(timer: *ALLEGRO_TIMER) -> int64_t)
externfn!(fn al_set_timer_count(timer: *mut ALLEGRO_TIMER, count: int64_t))
externfn!(fn al_add_timer_count(timer: *mut ALLEGRO_TIMER, diff: int64_t))
externfn!(fn al_get_timer_event_source(timer: *mut ALLEGRO_TIMER) -> *mut ALLEGRO_EVENT_SOURCE)
