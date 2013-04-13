pub mod C
{
	use core::libc::*;

	pub struct ALLEGRO_EVENT_SOURCE
	{
		priv __pad : [int, ..32]
	}
}
