#[macro_export]
macro_rules! test {
    ( $($count:literal, $out_file:literal, $function:expr, ($($arg:expr),*))? ) => {
        let file = std::fs::OpenOptions::new().write(true).create(true).open($out_file);
        for _ in 0..$count {
            let start = std::
            let _ = $function($($arg)*);

        }
    }
}
