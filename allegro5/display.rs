mod c
{
	use core::libc::*;

	pub struct ALLEGRO_DISPLAY;

	pub extern "C"
	{
		fn al_create_display(w : c_int, h : c_int) -> *ALLEGRO_DISPLAY;
		fn al_destroy_display(display : *ALLEGRO_DISPLAY);
	}
}

pub struct ALLEGRO_DISPLAY
{
	Payload : *c::ALLEGRO_DISPLAY
}

impl Drop for ALLEGRO_DISPLAY
{
	fn finalize(&self)
	{
		debug!("%s", "Finalizing display.");
		unsafe
		{
			c::al_destroy_display(self.Payload);
		}
	}
}

pub fn al_create_display(w : i32, h : i32) -> Option<ALLEGRO_DISPLAY>
{
	unsafe
	{
		match c::al_create_display(w, h)
		{
			d if ptr::is_null(d) => None,
			d => Some(ALLEGRO_DISPLAY{Payload : d})
		}
	}
}
