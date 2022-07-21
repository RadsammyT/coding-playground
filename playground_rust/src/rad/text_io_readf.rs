use text_io::*;
// that should not have been harder than it already was
#[macro_export]
macro_rules! readf {
	($($arg:tt)*) => {{
		use std::io::Write;
		std::io::stdout().flush().unwrap();
		try_read!($($arg)*)
	}};
}

// #[macro_export]
// macro_rules! read_test(
    
//     ($($arg:tt)*) => {
//         {use std::io::Write;
//         std::io::stdout().flush().unwrap();}
//         $crate::try_read!($($arg)*)
//     };
// );