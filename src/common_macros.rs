// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

macro_rules! if_ok
{
	($e: expr) =>
	{
		if ($e).is_err()
		{
			return Err(());
		}
	}
}

macro_rules! opaque
(
	($f: ident) =>
	{
		/* Mimicking c_void */
		pub enum $f
		{
		}
	}
)

macro_rules! flag_type
(
	($f: ident { $($n: ident = $v: expr),*}) =>
	{
		pub struct $f
		{
			bits: u32
		}

		impl $f
		{
			#[inline]
			pub fn get(&self) -> u32
			{
				self.bits
			}
		}

		impl Flag for $f
		{
			fn zero() -> $f
			{
				$f{bits: 0}
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
)

macro_rules! cast_to_c
(
	($p:ident, f32) =>
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

//~ wrap_bitmap_drawing!(al_draw_bitmap_region -> draw_bitmap_region(sx: f32, sy: f32, sw: f32, sh: f32, dx: f32, dy: f32))
//~ wrap_bitmap_drawing!(al_draw_bitmap_region -> draw_bitmap_region(sx: f32, sy: f32, sw: f32, sh: f32, dx: f32, dy: f32))
//~ wrap_bitmap_drawing!(al_draw_scaled_bitmap -> draw_scaled_bitmap(sx: f32, sy: f32, sw: f32, sh: f32, dx: f32, dy: f32, dw: f32, dh: f32))
//~ wrap_bitmap_drawing!(al_draw_rotated_bitmap -> draw_rotated_bitmap(cx: f32, cy: f32, dx: f32, dy: f32, angle: f32))
//~ wrap_bitmap_drawing!(al_draw_scaled_rotated_bitmap -> draw_scaled_rotated_bitmap(cx: f32, cy: f32, dx: f32, dy: f32, xscale: f32, yscale: f32, angle: f32))
//~ wrap_bitmap_drawing!(al_draw_tinted_bitmap -> draw_tinted_bitmap(tint: Color, dx: f32, dy: f32))
//~ wrap_bitmap_drawing!(al_draw_tinted_bitmap_region -> draw_tinted_bitmap_region(tint: Color, sx: f32, sy: f32, sw: f32, sh: f32, dx: f32, dy: f32))
//~ wrap_bitmap_drawing!(al_draw_tinted_scaled_bitmap -> draw_tinted_scaled_bitmap(tint: Color, sx: f32, sy: f32, sw: f32, sh: f32, dx: f32, dy: f32, dw: f32, dh: f32))
//~ wrap_bitmap_drawing!(al_draw_tinted_rotated_bitmap -> draw_tinted_rotated_bitmap(tint: Color, cx: f32, cy: f32, dx: f32, dy: f32, angle: f32))
//~ wrap_bitmap_drawing!(al_draw_tinted_scaled_rotated_bitmap -> draw_tinted_scaled_rotated_bitmap(tint: Color, cx: f32, cy: f32, dx: f32, dy: f32, xscale: f32, yscale: f32, angle: f32))
//~ wrap_bitmap_drawing!(al_draw_tinted_scaled_rotated_bitmap_region -> draw_tinted_scaled_rotated_bitmap_region(sx: f32, sy: f32, sw: f32, sh: f32, tint: Color, cx: f32, cy: f32, dx: f32, dy: f32, xscale: f32, yscale: f32, angle: f32))
