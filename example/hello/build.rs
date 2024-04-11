extern crate built;

use std::path::Path;
use std::env;

fn main() {
    built::write_built_file().expect("Failed to acquire build-time information");
}
