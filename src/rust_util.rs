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
