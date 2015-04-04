// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;
use allegro_util::c_bool;

use events::ALLEGRO_EVENT_SOURCE;

opaque!(ALLEGRO_TIMER);

extern "C"
{
	pub fn al_create_timer(speed_secs: c_double) -> *mut ALLEGRO_TIMER;
	pub fn al_destroy_timer(timer: *mut ALLEGRO_TIMER);
	pub fn al_start_timer(timer: *mut ALLEGRO_TIMER);
	pub fn al_stop_timer(timer: *mut ALLEGRO_TIMER);
	pub fn al_get_timer_started(timer: *const ALLEGRO_TIMER) -> c_bool;
	pub fn al_get_timer_speed(timer: *const ALLEGRO_TIMER) -> c_double;
	pub fn al_set_timer_speed(timer: *mut ALLEGRO_TIMER, speed_secs: c_double);
	pub fn al_get_timer_count(timer: *const ALLEGRO_TIMER) -> int64_t;
	pub fn al_set_timer_count(timer: *mut ALLEGRO_TIMER, count: int64_t);
	pub fn al_add_timer_count(timer: *mut ALLEGRO_TIMER, diff: int64_t);
	pub fn al_get_timer_event_source(timer: *mut ALLEGRO_TIMER) -> *mut ALLEGRO_EVENT_SOURCE;
}
