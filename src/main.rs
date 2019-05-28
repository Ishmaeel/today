#![windows_subsystem = "windows"]

fn main() {
    for i in std::env::args_os().skip(1) {
        process_arg(i);
    }
}

fn process_arg(arg: std::ffi::OsString) {
    let path = std::path::PathBuf::from(arg);

    if path.is_file() {
        copy_file(&path);
    }
}

fn copy_file(source: &std::path::PathBuf) {
    let mut target = std::path::PathBuf::from(source);

    let now = chrono::Utc::now();

    target.set_file_name(now.format("%Y.%m.%d.ext").to_string());

    target.set_extension(source.extension().unwrap_or_default());

    if target.is_file() || target.is_dir() {
        println!("Nope.");
    } else {
        match std::fs::copy(&source, &target) {
            Ok(_) => println!("Done."),
            Err(err) => println!("Ouch: {}", err),
        }
    }
}
