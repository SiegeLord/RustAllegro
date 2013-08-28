pub type c_bool = u8;

macro_rules! flag_type
(
	(mod $m:ident { $f: ident { $($n: ident = $v: expr),*} }) =>
	{
		pub mod $m
		{
			use std::num::Zero;

			pub struct $f
			{
				priv bits: u32
			}

			impl $f
			{
				#[inline]
				pub fn get(&self) -> u32
				{
					self.bits
				}
			}

			impl Zero for $f
			{
				#[inline]
				fn zero() -> $f
				{
					$f{bits: 0}
				}

				#[inline]
				fn is_zero(&self) -> bool
				{
					self.bits == 0
				}
			}

			impl BitOr<$f, $f> for $f
			{
				fn bitor(&self, e: &$f) -> $f
				{
					$f{bits: self.bits | e.bits}
				}
			}

			impl BitAnd<$f, bool> for $f
			{
				fn bitand(&self, e: &$f) -> bool
				{
					self.bits & e.bits != 0
				}
			}

			$(
				pub static $n: $f = $f{bits: $v};
			)+
		}
	}
)

macro_rules! cast_to_c
(
	($p:ident, float) =>
	{
		$p as c_float
	};
	($p:ident, Color) =>
	{
		*$p
	}
)

macro_rules! wrap_bitmap_drawing
(
	($cf:ident -> $rf:ident ( $( $p:ident : $t:ident ),* )) =>
	{
		fn $rf<T: BitmapLike>(dummy:dummy, bitmap: &T, $( $p : $t ),* , flags: BitmapDrawingFlags)
		{
			target_bitmap_check(self.get_target_bitmap());
			unsafe
			{
				$cf(bitmap.get_bitmap(),
					$(
						cast_to_c!($p, $t)
					),*
				, (flags.get() << 1) as c_int);
			}
		}
	}
)

//~ wrap_bitmap_drawing!(al_draw_bitmap_region -> draw_bitmap_region(sx: float, sy: float, sw: float, sh: float, dx: float, dy: float))
//~ wrap_bitmap_drawing!(al_draw_bitmap_region -> draw_bitmap_region(sx: float, sy: float, sw: float, sh: float, dx: float, dy: float))
//~ wrap_bitmap_drawing!(al_draw_scaled_bitmap -> draw_scaled_bitmap(sx: float, sy: float, sw: float, sh: float, dx: float, dy: float, dw: float, dh: float))
//~ wrap_bitmap_drawing!(al_draw_rotated_bitmap -> draw_rotated_bitmap(cx: float, cy: float, dx: float, dy: float, angle: float))
//~ wrap_bitmap_drawing!(al_draw_scaled_rotated_bitmap -> draw_scaled_rotated_bitmap(cx: float, cy: float, dx: float, dy: float, xscale: float, yscale: float, angle: float))
//~ wrap_bitmap_drawing!(al_draw_tinted_bitmap -> draw_tinted_bitmap(tint: Color, dx: float, dy: float))
//~ wrap_bitmap_drawing!(al_draw_tinted_bitmap_region -> draw_tinted_bitmap_region(tint: Color, sx: float, sy: float, sw: float, sh: float, dx: float, dy: float))
//~ wrap_bitmap_drawing!(al_draw_tinted_scaled_bitmap -> draw_tinted_scaled_bitmap(tint: Color, sx: float, sy: float, sw: float, sh: float, dx: float, dy: float, dw: float, dh: float))
//~ wrap_bitmap_drawing!(al_draw_tinted_rotated_bitmap -> draw_tinted_rotated_bitmap(tint: Color, cx: float, cy: float, dx: float, dy: float, angle: float))
//~ wrap_bitmap_drawing!(al_draw_tinted_scaled_rotated_bitmap -> draw_tinted_scaled_rotated_bitmap(tint: Color, cx: float, cy: float, dx: float, dy: float, xscale: float, yscale: float, angle: float))
//~ wrap_bitmap_drawing!(al_draw_tinted_scaled_rotated_bitmap_region -> draw_tinted_scaled_rotated_bitmap_region(sx: float, sy: float, sw: float, sh: float, tint: Color, cx: float, cy: float, dx: float, dy: float, xscale: float, yscale: float, angle: float))
