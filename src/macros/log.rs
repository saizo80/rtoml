#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        if env::var("RUST_LOG") == Ok("debug".to_string()) {
            const BLUE: &str = "\x1b[34m";
            const RESET: &str = "\x1b[0m";
            let exe_path = env::current_exe().unwrap_or(std::path::PathBuf::from(""));
            let exe = exe_path.file_name().unwrap_or(std::ffi::OsStr::new("")).to_str().unwrap_or("");
            eprintln!("{BLUE}DEBUG{RESET} {exe} > {}", format!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        const RESET: &str = "\x1b[0m";
        const RED: &str = "\x1b[31m";
        let exe_path = env::current_exe().unwrap_or(std::path::PathBuf::from(""));
        let exe = exe_path.file_name().unwrap_or(std::ffi::OsStr::new("")).to_str().unwrap_or("");
        eprintln!("{RED}ERROR{RESET} {exe} > {}", format!($($arg)*));
    };
}
