#[macro_export]
macro_rules! test {
    ( $count:literal, $out_file:literal, $function:expr, $arg:expr ) => {{
        use std::io::Write;
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .write(true)
            .open($out_file)
            .unwrap();
        for _ in 0..$count {
            let start = std::time::Instant::now();
            let _ = $function($arg);
            let time = std::time::Instant::now().duration_since(start);
            file.write(format!("{} ", time.as_nanos()).as_bytes())
                .unwrap();
        }
        file.write("\n".as_bytes()).unwrap();
    }};
}
