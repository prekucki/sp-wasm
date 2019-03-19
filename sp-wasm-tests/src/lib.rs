use std::env;
use std::path;
use std::ffi;

pub fn current_prog_name() -> Option<String> {
    env::args().next().as_ref().map(path::Path::new).and_then(path::Path::file_name).and_then(ffi::OsStr::to_str).map(String::from)
}