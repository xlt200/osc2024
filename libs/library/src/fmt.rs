use super::string::String;
use core::fmt::{Arguments, Write};

pub fn format(args: Arguments<'_>) -> String {
    fn format_inner(args: Arguments<'_>) -> String {
        let mut output = String::new();
        output
            .write_fmt(args)
            .expect("a formatting trait implementation returned an error");
        output
    }
    args.as_str()
        .map_or_else(|| format_inner(args), String::from)
}

#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {{
        let res = ($crate::fmt::format(format_args!($($arg)*)));
        res
    }}
}
