// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;
use allegro_util::c_bool;

pub type ALLEGRO_USTR = __al_tagbstring;
pub type ALLEGRO_USTR_INFO = __al_tagbstring;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __al_tagbstring
{
    pub mlen: c_int,
    pub slen: c_int,
    pub data: *mut c_uchar,
}

extern "C"
{
    pub fn al_ustr_new(s: *const c_char) -> *mut ALLEGRO_USTR;
    pub fn al_ustr_new_from_buffer(s: *const c_char, size: size_t) -> *mut ALLEGRO_USTR;
    pub fn al_ustr_free(us: *mut ALLEGRO_USTR);
    pub fn al_cstr(us: *const ALLEGRO_USTR) -> *const c_char;
    pub fn al_ustr_to_buffer(us: *const ALLEGRO_USTR, buffer: *mut c_char, size: c_int);
    pub fn al_cstr_dup(us: *const ALLEGRO_USTR) -> *mut c_char;
    pub fn al_ustr_dup(us: *const ALLEGRO_USTR) -> *mut ALLEGRO_USTR;
    pub fn al_ustr_dup_substr(us: *const ALLEGRO_USTR, start_pos: c_int, end_pos: c_int) -> *mut ALLEGRO_USTR;
    pub fn al_ustr_empty_string() -> *const ALLEGRO_USTR;
    pub fn al_ref_cstr(info: *mut ALLEGRO_USTR_INFO, s: *const c_char) -> *const ALLEGRO_USTR;
    pub fn al_ref_buffer(info: *mut ALLEGRO_USTR_INFO, s: *const c_char, size: size_t) -> *const ALLEGRO_USTR;
    pub fn al_ref_ustr(info: *mut ALLEGRO_USTR_INFO, us: *const ALLEGRO_USTR, start_pos: c_int, end_pos: c_int) -> *const ALLEGRO_USTR;
    pub fn al_ustr_size(us: *const ALLEGRO_USTR) -> size_t;
    pub fn al_ustr_length(us: *const ALLEGRO_USTR) -> size_t;
    pub fn al_ustr_offset(us: *const ALLEGRO_USTR, index: c_int) -> c_int;
    pub fn al_ustr_next(us: *const ALLEGRO_USTR, pos: *mut c_int) -> c_bool;
    pub fn al_ustr_prev(us: *const ALLEGRO_USTR, pos: *mut c_int) -> c_bool;
    pub fn al_ustr_get(us: *const ALLEGRO_USTR, pos: c_int) -> int32_t;
    pub fn al_ustr_get_next(us: *const ALLEGRO_USTR, pos: *mut c_int) -> int32_t;
    pub fn al_ustr_prev_get(us: *const ALLEGRO_USTR, pos: *mut c_int) -> int32_t;
    pub fn al_ustr_insert(us1: *mut ALLEGRO_USTR, pos: c_int, us2: *const ALLEGRO_USTR) -> c_bool;
    pub fn al_ustr_insert_cstr(us: *mut ALLEGRO_USTR, pos: c_int, us2: *const c_char) -> c_bool;
    pub fn al_ustr_insert_chr(us: *mut ALLEGRO_USTR, pos: c_int, c: int32_t) -> size_t;
    pub fn al_ustr_append(us1: *mut ALLEGRO_USTR, us2: *const ALLEGRO_USTR) -> c_bool;
    pub fn al_ustr_append_cstr(us: *mut ALLEGRO_USTR, s: *const c_char) -> c_bool;
    pub fn al_ustr_append_chr(us: *mut ALLEGRO_USTR, c: int32_t) -> size_t;
    pub fn al_ustr_remove_chr(us: *mut ALLEGRO_USTR, pos: c_int) -> c_bool;
    pub fn al_ustr_remove_range(us: *mut ALLEGRO_USTR, start_pos: c_int, end_pos: c_int) -> c_bool;
    pub fn al_ustr_truncate(us: *mut ALLEGRO_USTR, start_pos: c_int) -> c_bool;
    pub fn al_ustr_ltrim_ws(us: *mut ALLEGRO_USTR) -> c_bool;
    pub fn al_ustr_rtrim_ws(us: *mut ALLEGRO_USTR) -> c_bool;
    pub fn al_ustr_trim_ws(us: *mut ALLEGRO_USTR) -> c_bool;
    pub fn al_ustr_assign(us1: *mut ALLEGRO_USTR, us2: *const ALLEGRO_USTR) -> c_bool;
    pub fn al_ustr_assign_substr(us1: *mut ALLEGRO_USTR, us2: *const ALLEGRO_USTR, start_pos: c_int, end_pos: c_int) -> c_bool;
    pub fn al_ustr_assign_cstr(us1: *mut ALLEGRO_USTR, s: *const c_char) -> c_bool;
    pub fn al_ustr_set_chr(us: *mut ALLEGRO_USTR, pos: c_int, c: int32_t) -> size_t;
    pub fn al_ustr_replace_range(us1: *mut ALLEGRO_USTR, start_pos1: c_int, end_pos1: c_int, us2: *const ALLEGRO_USTR) -> c_bool;
    pub fn al_ustr_find_chr(us: *const ALLEGRO_USTR, start_pos: c_int, c: int32_t) -> c_int;
    pub fn al_ustr_rfind_chr(us: *const ALLEGRO_USTR, start_pos: c_int, c: int32_t) -> c_int;
    pub fn al_ustr_find_set(us: *const ALLEGRO_USTR, start_pos: c_int, accept: *const ALLEGRO_USTR) -> c_int;
    pub fn al_ustr_find_set_cstr(us: *const ALLEGRO_USTR, start_pos: c_int, accept: *const c_char) -> c_int;
    pub fn al_ustr_find_cset(us: *const ALLEGRO_USTR, start_pos: c_int, reject: *const ALLEGRO_USTR) -> c_int;
    pub fn al_ustr_find_cset_cstr(us: *const ALLEGRO_USTR, start_pos: c_int, reject: *const c_char) -> c_int;
    pub fn al_ustr_find_str(haystack: *const ALLEGRO_USTR, start_pos: c_int, needle: *const ALLEGRO_USTR) -> c_int;
    pub fn al_ustr_find_cstr(haystack: *const ALLEGRO_USTR, start_pos: c_int, needle: *const c_char) -> c_int;
    pub fn al_ustr_rfind_str(haystack: *const ALLEGRO_USTR, start_pos: c_int, needle: *const ALLEGRO_USTR) -> c_int;
    pub fn al_ustr_rfind_cstr(haystack: *const ALLEGRO_USTR, start_pos: c_int, needle: *const c_char) -> c_int;
    pub fn al_ustr_find_replace(us: *mut ALLEGRO_USTR, start_pos: c_int, find: *const ALLEGRO_USTR, replace: *const ALLEGRO_USTR) -> c_bool;
    pub fn al_ustr_find_replace_cstr(us: *mut ALLEGRO_USTR, start_pos: c_int, find: *const c_char, replace: *const c_char) -> c_bool;
    pub fn al_ustr_equal(us1: *const ALLEGRO_USTR, us2: *const ALLEGRO_USTR) -> c_bool;
    pub fn al_ustr_compare(u: *const ALLEGRO_USTR, v: *const ALLEGRO_USTR) -> c_int;
    pub fn al_ustr_ncompare(us1: *const ALLEGRO_USTR, us2: *const ALLEGRO_USTR, n: c_int) -> c_int;
    pub fn al_ustr_has_prefix(u: *const ALLEGRO_USTR, v: *const ALLEGRO_USTR) -> c_bool;
    pub fn al_ustr_has_prefix_cstr(u: *const ALLEGRO_USTR, s: *const c_char) -> c_bool;
    pub fn al_ustr_has_suffix(u: *const ALLEGRO_USTR, v: *const ALLEGRO_USTR) -> c_bool;
    pub fn al_ustr_has_suffix_cstr(us1: *const ALLEGRO_USTR, s: *const c_char) -> c_bool;
    pub fn al_utf8_width(c: int32_t) -> size_t;
    pub fn al_utf8_encode(s: c_void, c: int32_t) -> size_t;
    pub fn al_ustr_new_from_utf16(s: *const uint16_t) -> *mut ALLEGRO_USTR;
    pub fn al_ustr_size_utf16(us: *const ALLEGRO_USTR) -> size_t;
    pub fn al_ustr_encode_utf16(us: *const ALLEGRO_USTR, s: *mut uint16_t, n: size_t) -> size_t;
    pub fn al_utf16_width(c: c_int) -> size_t;
    pub fn al_utf16_encode(s: c_void, c: int32_t) -> size_t;
}
