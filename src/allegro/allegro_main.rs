// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#[macro_export]
macro_rules! allegro_main
{
	(^tt_expander $e:expr) => {$e};
	($($x:tt)*) =>
	{
		#[start]
		fn start(argc: int, argv: **u8) -> int
		{
			allegro5::run(argc, argv, main)
		}

		fn main()
		{
			allegro_main!(^tt_expander { $($x)* })
		}
	}
}
