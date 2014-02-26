#[allow(non_camel_case_types)]
pub type c_bool = u8;

pub trait Flag
{
	fn zero() -> Self;
}
