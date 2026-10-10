// SPDX-License-Identifier: MIT
use std::fs;
use std::io;

pub fn load(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}
