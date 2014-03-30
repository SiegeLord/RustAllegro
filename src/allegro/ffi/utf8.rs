use std::libc::*;
use rust_util::c_bool;

pub type ALLEGRO_USTR = __al_tagbstring;
pub type ALLEGRO_USTR_INFO = __al_tagbstring;

pub struct __al_tagbstring
{
    mlen: c_int,
    slen: c_int,
    data: *mut c_uchar,
}

extern "C"
{
    pub fn al_ustr_new(s: *c_schar) -> *mut ALLEGRO_USTR;
    pub fn al_ustr_new_from_buffer(s: *c_schar, size: size_t) -> *mut ALLEGRO_USTR;
    pub fn al_ustr_free(us: *mut ALLEGRO_USTR);
    pub fn al_cstr(us: *ALLEGRO_USTR) -> *c_schar;
    pub fn al_ustr_to_buffer(us: *ALLEGRO_USTR, buffer: *mut c_schar, size: c_int);
    pub fn al_cstr_dup(us: *ALLEGRO_USTR) -> *mut c_schar;
    pub fn al_ustr_dup(us: *ALLEGRO_USTR) -> *mut ALLEGRO_USTR;
    pub fn al_ustr_dup_substr(us: *ALLEGRO_USTR, start_pos: c_int, end_pos: c_int) -> *mut ALLEGRO_USTR;
    pub fn al_ustr_empty_string() -> *ALLEGRO_USTR;
    pub fn al_ref_cstr(info: *mut ALLEGRO_USTR_INFO, s: *c_schar) -> *ALLEGRO_USTR;
    pub fn al_ref_buffer(info: *mut ALLEGRO_USTR_INFO, s: *c_schar, size: size_t) -> *ALLEGRO_USTR;
    pub fn al_ref_ustr(info: *mut ALLEGRO_USTR_INFO, us: *ALLEGRO_USTR, start_pos: c_int, end_pos: c_int) -> *ALLEGRO_USTR;
    pub fn al_ustr_size(us: *ALLEGRO_USTR) -> size_t;
    pub fn al_ustr_length(us: *ALLEGRO_USTR) -> size_t;
    pub fn al_ustr_offset(us: *ALLEGRO_USTR, index: c_int) -> c_int;
    pub fn al_ustr_next(us: *ALLEGRO_USTR, pos: *mut c_int) -> c_bool;
    pub fn al_ustr_prev(us: *ALLEGRO_USTR, pos: *mut c_int) -> c_bool;
    pub fn al_ustr_get(us: *ALLEGRO_USTR, pos: c_int) -> int32_t;
    pub fn al_ustr_get_next(us: *ALLEGRO_USTR, pos: *mut c_int) -> int32_t;
    pub fn al_ustr_prev_get(us: *ALLEGRO_USTR, pos: *mut c_int) -> int32_t;
    pub fn al_ustr_insert(us1: *mut ALLEGRO_USTR, pos: c_int, us2: *ALLEGRO_USTR) -> c_bool;
    pub fn al_ustr_insert_cstr(us: *mut ALLEGRO_USTR, pos: c_int, us2: *c_schar) -> c_bool;
    pub fn al_ustr_insert_chr(us: *mut ALLEGRO_USTR, pos: c_int, c: int32_t) -> size_t;
    pub fn al_ustr_append(us1: *mut ALLEGRO_USTR, us2: *ALLEGRO_USTR) -> c_bool;
    pub fn al_ustr_append_cstr(us: *mut ALLEGRO_USTR, s: *c_schar) -> c_bool;
    pub fn al_ustr_append_chr(us: *mut ALLEGRO_USTR, c: int32_t) -> size_t;
    pub fn al_ustr_remove_chr(us: *mut ALLEGRO_USTR, pos: c_int) -> c_bool;
    pub fn al_ustr_remove_range(us: *mut ALLEGRO_USTR, start_pos: c_int, end_pos: c_int) -> c_bool;
    pub fn al_ustr_truncate(us: *mut ALLEGRO_USTR, start_pos: c_int) -> c_bool;
    pub fn al_ustr_ltrim_ws(us: *mut ALLEGRO_USTR) -> c_bool;
    pub fn al_ustr_rtrim_ws(us: *mut ALLEGRO_USTR) -> c_bool;
    pub fn al_ustr_trim_ws(us: *mut ALLEGRO_USTR) -> c_bool;
    pub fn al_ustr_assign(us1: *mut ALLEGRO_USTR, us2: *ALLEGRO_USTR) -> c_bool;
    pub fn al_ustr_assign_substr(us1: *mut ALLEGRO_USTR, us2: *ALLEGRO_USTR, start_pos: c_int, end_pos: c_int) -> c_bool;
    pub fn al_ustr_assign_cstr(us1: *mut ALLEGRO_USTR, s: *c_schar) -> c_bool;
    pub fn al_ustr_set_chr(us: *mut ALLEGRO_USTR, pos: c_int, c: int32_t) -> size_t;
    pub fn al_ustr_replace_range(us1: *mut ALLEGRO_USTR, start_pos1: c_int, end_pos1: c_int, us2: *ALLEGRO_USTR) -> c_bool;
    pub fn al_ustr_find_chr(us: *ALLEGRO_USTR, start_pos: c_int, c: int32_t) -> c_int;
    pub fn al_ustr_rfind_chr(us: *ALLEGRO_USTR, start_pos: c_int, c: int32_t) -> c_int;
    pub fn al_ustr_find_set(us: *ALLEGRO_USTR, start_pos: c_int, accept: *ALLEGRO_USTR) -> c_int;
    pub fn al_ustr_find_set_cstr(us: *ALLEGRO_USTR, start_pos: c_int, accept: *c_schar) -> c_int;
    pub fn al_ustr_find_cset(us: *ALLEGRO_USTR, start_pos: c_int, reject: *ALLEGRO_USTR) -> c_int;
    pub fn al_ustr_find_cset_cstr(us: *ALLEGRO_USTR, start_pos: c_int, reject: *c_schar) -> c_int;
    pub fn al_ustr_find_str(haystack: *ALLEGRO_USTR, start_pos: c_int, needle: *ALLEGRO_USTR) -> c_int;
    pub fn al_ustr_find_cstr(haystack: *ALLEGRO_USTR, start_pos: c_int, needle: *c_schar) -> c_int;
    pub fn al_ustr_rfind_str(haystack: *ALLEGRO_USTR, start_pos: c_int, needle: *ALLEGRO_USTR) -> c_int;
    pub fn al_ustr_rfind_cstr(haystack: *ALLEGRO_USTR, start_pos: c_int, needle: *c_schar) -> c_int;
    pub fn al_ustr_find_replace(us: *mut ALLEGRO_USTR, start_pos: c_int, find: *ALLEGRO_USTR, replace: *ALLEGRO_USTR) -> c_bool;
    pub fn al_ustr_find_replace_cstr(us: *mut ALLEGRO_USTR, start_pos: c_int, find: *c_schar, replace: *c_schar) -> c_bool;
    pub fn al_ustr_equal(us1: *ALLEGRO_USTR, us2: *ALLEGRO_USTR) -> c_bool;
    pub fn al_ustr_compare(u: *ALLEGRO_USTR, v: *ALLEGRO_USTR) -> c_int;
    pub fn al_ustr_ncompare(us1: *ALLEGRO_USTR, us2: *ALLEGRO_USTR, n: c_int) -> c_int;
    pub fn al_ustr_has_prefix(u: *ALLEGRO_USTR, v: *ALLEGRO_USTR) -> c_bool;
    pub fn al_ustr_has_prefix_cstr(u: *ALLEGRO_USTR, s: *c_schar) -> c_bool;
    pub fn al_ustr_has_suffix(u: *ALLEGRO_USTR, v: *ALLEGRO_USTR) -> c_bool;
    pub fn al_ustr_has_suffix_cstr(us1: *ALLEGRO_USTR, s: *c_schar) -> c_bool;
    pub fn al_utf8_width(c: int32_t) -> size_t;
    pub fn al_utf8_encode(s: c_void, c: int32_t) -> size_t;
    pub fn al_ustr_new_from_utf16(s: *uint16_t) -> *mut ALLEGRO_USTR;
    pub fn al_ustr_size_utf16(us: *ALLEGRO_USTR) -> size_t;
    pub fn al_ustr_encode_utf16(us: *ALLEGRO_USTR, s: *mut uint16_t, n: size_t) -> size_t;
    pub fn al_utf16_width(c: c_int) -> size_t;
    pub fn al_utf16_encode(s: c_void, c: int32_t) -> size_t;
}
