// Copyright 2020 VinyMeuh. All rights reserved.
// Use of the source code is governed by a MIT-style license that can be found in the LICENSE file.

use std::path::Path;

use glob::glob;
use chardevgpio::{Chip};

fn print_chip_info(path: &Path) {

    let mut chip = match Chip::new(path) {
        Ok(chip) => chip,
        Err(err) => {
            eprintln!("{}: {:?}", path.display(), err);
            return;
        }        
    };
    println!("file = {}, name = {}, label = {}, lines = {}", path.display(), chip.name, chip.label, chip.lines);

    for i in 0..chip.lines {
        let line = match chip.line_info(i) {
            Ok(line) => line,
            Err(err) => {
                eprintln!("line {:02}: {:?}", i, err);
                return;
            }
        };
        println!("    line {}: name = \"{}\", consumer = \"{}\", flags = ", line.offset, line.name, line.consumer);
    }
}


fn main() {
    for entry in glob("/dev/gpiochip*").unwrap() {
        match entry {
            Ok(path) => print_chip_info(&path),
            Err(err) => eprintln!("{:?}", err),
        }
    }
}