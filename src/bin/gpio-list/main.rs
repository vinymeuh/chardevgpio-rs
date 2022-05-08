// Copyright 2020 VinyMeuh. All rights reserved.
// Use of the source code is governed by a MIT-style license that can be found in the LICENSE file.

use std::path::Path;

use glob::glob;
use chardevgpio::{Chip};

fn print_chip_info(path: &Path) {

    let chip = match Chip::new(path) {
        Ok(chip) => chip,
        Err(err) => {
            eprintln!("{}: {:?}", path.display(), err);
            return;
        }        
    };

    println!("file = {}, name = {}, label = {}, lines = {}", path.display(), chip.name, chip.label, chip.lines);
}


fn main() {
    for entry in glob("/dev/gpiochip*").unwrap() {
        match entry {
            Ok(path) => print_chip_info(&path),
            Err(err) => eprintln!("{:?}", err),
        }
    }
}