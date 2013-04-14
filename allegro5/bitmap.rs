use core::libc::*;

use allegro5::color::ALLEGRO_COLOR;
use allegro5::rust_util::c_bool;

pub mod C
{
	use core::libc::*;

	use allegro5::color::ALLEGRO_COLOR;
	use allegro5::rust_util::c_bool;

	pub struct ALLEGRO_BITMAP;

	pub extern "C"
	{
		fn al_set_new_bitmap_format(format : c_int);
		fn al_set_new_bitmap_flags(flags : c_int);
		fn al_get_new_bitmap_format() -> c_int;
		fn al_get_new_bitmap_flags() -> c_int;
		fn al_add_new_bitmap_flag(flag : c_int);

		fn al_get_bitmap_width(bitmap : *mut ALLEGRO_BITMAP) -> c_int;
		fn al_get_bitmap_height(bitmap : *mut ALLEGRO_BITMAP) -> c_int;
		fn al_get_bitmap_format(bitmap : *mut ALLEGRO_BITMAP) -> c_int;
		fn al_get_bitmap_flags(bitmap : *mut ALLEGRO_BITMAP) -> c_int;

		fn al_create_bitmap(w : c_int, h : c_int) -> *mut ALLEGRO_BITMAP;
		fn al_create_custom_bitmap(w : c_int, h : c_int, upload : extern "C" fn(bitmap : *mut ALLEGRO_BITMAP, data : *mut c_void) -> c_bool, data : *mut c_void) -> *mut ALLEGRO_BITMAP;
		fn al_destroy_bitmap(bitmap : *mut ALLEGRO_BITMAP);

		fn al_put_pixel(x : c_int, y : c_int, color : ALLEGRO_COLOR);
		fn al_put_blended_pixel(x : c_int, y : c_int, color : ALLEGRO_COLOR);
		fn al_get_pixel(bitmap : *mut ALLEGRO_BITMAP, x : c_int, y : c_int) -> ALLEGRO_COLOR;

		fn al_convert_mask_to_alpha(bitmap : *mut ALLEGRO_BITMAP, mask_color : ALLEGRO_COLOR);

		fn al_set_clipping_rectangle(x : c_int, y : c_int, width : c_int, height : c_int);
		fn al_reset_clipping_rectangle();
		fn al_get_clipping_rectangle(x : *mut c_int, y : *mut c_int, w : *mut c_int, h : *mut c_int);

		fn al_create_sub_bitmap(parent : *mut ALLEGRO_BITMAP, x : c_int, y : c_int, w : c_int, h : c_int) -> *mut ALLEGRO_BITMAP;
		fn al_is_sub_bitmap(bitmap : *mut ALLEGRO_BITMAP) -> c_bool;
		fn al_get_parent_bitmap(bitmap : *mut ALLEGRO_BITMAP) -> *mut ALLEGRO_BITMAP;

		fn al_clone_bitmap(bitmap : *mut ALLEGRO_BITMAP) -> *mut ALLEGRO_BITMAP;
		fn al_convert_bitmap(bitmap : *mut ALLEGRO_BITMAP);
		fn al_convert_bitmaps();
	}
}

pub struct ALLEGRO_BITMAP
{
	Payload : *mut C::ALLEGRO_BITMAP
}

unsafe fn ToOption(c_bmp : *mut C::ALLEGRO_BITMAP) -> Option<ALLEGRO_BITMAP>
{
	match c_bmp
	{
		b if ptr::is_null(b) => None,
		b => Some(ALLEGRO_BITMAP{Payload : b})
	}
}

impl Drop for ALLEGRO_BITMAP
{
	fn finalize(&self)
	{
		debug!("%s", "Finalizing bitmap.");
		unsafe
		{
			C::al_destroy_bitmap(self.Payload);
		}
	}
}

pub static ALLEGRO_MEMORY_BITMAP: i32                 = 0x0001;
pub static _ALLEGRO_KEEP_BITMAP_FORMAT: i32           = 0x0002;
pub static ALLEGRO_FORCE_LOCKING: i32                 = 0x0004;
pub static ALLEGRO_NO_PRESERVE_TEXTURE: i32           = 0x0008;
pub static _ALLEGRO_ALPHA_TEST: i32                   = 0x0010;
pub static _ALLEGRO_INTERNAL_OPENGL: i32              = 0x0020;
pub static ALLEGRO_MIN_LINEAR: i32                    = 0x0040;
pub static ALLEGRO_MAG_LINEAR: i32                    = 0x0080;
pub static ALLEGRO_MIPMAP: i32                        = 0x0100;
pub static _ALLEGRO_NO_PREMULTIPLIED_ALPHA: i32       = 0x0200;
pub static ALLEGRO_VIDEO_BITMAP: i32                  = 0x0400;
pub static ALLEGRO_CONVERT_BITMAP: i32                = 0x1000;

pub fn al_set_new_bitmap_format(format : i32)
{
	unsafe
	{
		C::al_set_new_bitmap_format(format as c_int);
	}
}

pub fn al_set_new_bitmap_flags(flags : i32)
{
	unsafe
	{
		C::al_set_new_bitmap_flags(flags as c_int);
	}
}

pub fn al_get_new_bitmap_format() -> i32
{
	unsafe
	{
		return C::al_get_new_bitmap_format() as i32;
	}
}

pub fn al_get_new_bitmap_flags() -> i32
{
	unsafe
	{
		return C::al_get_new_bitmap_flags() as i32;
	}
}

pub fn al_add_new_bitmap_flag(flag : i32)
{
	unsafe
	{
		C::al_add_new_bitmap_flag(flag as c_int);
	}
}

pub fn al_get_bitmap_width(bitmap : &ALLEGRO_BITMAP) -> i32
{
	unsafe
	{
		return C::al_get_bitmap_width(bitmap.Payload) as i32;
	}
}

pub fn al_get_bitmap_height(bitmap : &ALLEGRO_BITMAP) -> i32
{
	unsafe
	{
		return C::al_get_bitmap_height(bitmap.Payload) as i32;
	}
}

pub fn al_get_bitmap_format(bitmap : &ALLEGRO_BITMAP) -> i32
{
	unsafe
	{
		return C::al_get_bitmap_format(bitmap.Payload) as i32;
	}
}

pub fn al_get_bitmap_flags(bitmap : &ALLEGRO_BITMAP) -> i32
{
	unsafe
	{
		return C::al_get_bitmap_flags(bitmap.Payload) as i32;
	}
}

pub fn al_create_bitmap(w : i32, h : i32) -> Option<ALLEGRO_BITMAP>
{
	unsafe
	{
		return ToOption(C::al_create_bitmap(w as c_int, h as c_int));
	}
}

pub fn al_create_custom_bitmap<T>(w : i32, h : i32, upload : &fn(bitmap : Option<ALLEGRO_BITMAP>, data : &T) -> bool, data : &T) -> Option<ALLEGRO_BITMAP>
{
	unsafe
	{
		struct RustData<'self, T>
		{
			Upload : &'self fn(bitmap : Option<ALLEGRO_BITMAP>, &T) -> bool,
			Data : &'self T
		}

		extern "C"
		fn RustCallback<T>(bitmap : *mut C::ALLEGRO_BITMAP, data : *mut c_void) -> c_bool
		{
			unsafe
			{
				let rust_data : &mut RustData<T> = cast::transmute(data);
				return (rust_data.Upload)(ToOption(bitmap), rust_data.Data) as c_bool;
			}
		}

		let mut rust_data = RustData{Upload : upload, Data : data};

		ToOption(C::al_create_custom_bitmap(w as c_int, h as c_int, cast::transmute(RustCallback), cast::transmute(&rust_data)))
	}
}

pub fn al_convert_mask_to_alpha(bitmap : &ALLEGRO_BITMAP, mask_color : ALLEGRO_COLOR)
{
	unsafe
	{
		C::al_convert_mask_to_alpha(bitmap.Payload, mask_color);
	}
}

pub fn al_set_clipping_rectangle(x : i32, y : i32, width : i32, height : i32)
{
	unsafe
	{
		C::al_set_clipping_rectangle(x as c_int, y as c_int, width as c_int, height as c_int);
	}
}

pub fn al_reset_clipping_rectangle()
{
	unsafe
	{
		C::al_reset_clipping_rectangle();
	}
}

pub fn al_get_clipping_rectangle(x : &mut i32, y : &mut i32, w : &mut i32, h : &mut i32)
{
	unsafe
	{
		C::al_get_clipping_rectangle(x as *mut c_int, y as *mut c_int, w as *mut c_int, h as *mut c_int);
	}
}

pub fn al_create_sub_bitmap(parent : &ALLEGRO_BITMAP, x : i32, y : i32, w : i32, h : i32) -> Option<ALLEGRO_BITMAP>
{
	unsafe
	{
		return ToOption(C::al_create_sub_bitmap(parent.Payload, x as c_int, y as c_int, w as c_int, h as c_int));
	}
}

pub fn al_is_sub_bitmap(bitmap : &ALLEGRO_BITMAP) -> bool
{
	unsafe
	{
		return C::al_is_sub_bitmap(bitmap.Payload) as bool;
	}
}

pub fn al_get_parent_bitmap(bitmap : &ALLEGRO_BITMAP) -> Option<ALLEGRO_BITMAP>
{
	unsafe
	{
		return ToOption(C::al_get_parent_bitmap(bitmap.Payload));
	}
}

pub fn al_clone_bitmap(bitmap : &ALLEGRO_BITMAP) -> Option<ALLEGRO_BITMAP>
{
	unsafe
	{
		return ToOption(C::al_clone_bitmap(bitmap.Payload));
	}
}

pub fn al_convert_bitmap(bitmap : &ALLEGRO_BITMAP)
{
	unsafe
	{
		C::al_convert_bitmap(bitmap.Payload);
	}
}

pub fn al_convert_bitmaps()
{
	unsafe
	{
		C::al_convert_bitmaps();
	}
}
