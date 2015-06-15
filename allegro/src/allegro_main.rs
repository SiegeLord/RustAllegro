// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#[macro_export]
macro_rules! allegro_main
{
	(^tt_expander $e:expr) => {$e};
	($($x:tt)*) =>
	{
		fn main()
		{
			allegro::run(user_main)
		}

		fn user_main()
		{
			allegro_main!(^tt_expander { $($x)* })
		}
	}
}
