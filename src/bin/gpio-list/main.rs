// Copyright 2020 VinyMeuh. All rights reserved.
// Use of the source code is governed by a MIT-style license that can be found in the LICENSE file.

use std::path::Path;

use glob::glob;
use chardevgpio::{Chip};

fn print_chip_info(path: &Path) -> Result<(), chardevgpio::Error> {

    let mut chip = Chip::new(path)?;
    println!("file = {}, name = {}, label = {}, lines = {}", path.display(), chip.name(), chip.label(), chip.lines());

    for i in 0..chip.lines() {
        let line = chip.line_info(i)?;
        println!("    line {}: name = \"{}\", consumer = \"{}\", flags = ", line.offset(), line.name(), line.consumer());
    }

    Ok(())
}


fn main() {
    for entry in glob("/dev/gpiochip*").unwrap() {
        match entry {
            Ok(path) => {
                print_chip_info(&path)
                    .unwrap_or_else(|err| eprintln!("{}: {:?}", path.display(), err))
            },
            Err(err) => eprintln!("{:?}", err),
        }
    }
}