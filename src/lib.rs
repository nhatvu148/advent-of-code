use std::{env, ffi::OsStr, path::PathBuf, time::Instant};

pub mod days;

pub fn runner(f: impl Fn(&str)) {
    let start_time = Instant::now();
    let file_path = format!("input/{}", file_name());

    f(&file_path);

    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:.2?}", elapsed_time.as_secs_f64());
}

pub fn file_name() -> String {
    env::args()
        .skip(1)
        .next()
        .map(PathBuf::from)
        .and_then(|path| {
            path.file_name()
                .map(OsStr::to_string_lossy)
                .map(|s| s.into_owned())
        })
        .expect("file not found")
}
