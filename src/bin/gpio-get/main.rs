// Copyright 2022 VinyMeuh. All rights reserved.
// Use of the source code is governed by a MIT-style license that can be found in the LICENSE file.

use std::env;
use std::path::Path;

use chardevgpio::Chip;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let exe_name = Path::new(&arguments[0])
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    if arguments.len() != 3 {
        eprintln!("Usage: {} device line_number", exe_name);
        eprintln!("For example: {} /dev/gpiochip0 22", exe_name);
        std::process::exit(1);
    }

    let chip = Chip::new(Path::new(&arguments[1])).unwrap_or_else(|err| {
        eprintln!("Error while opening chip: {:?}", err);
        std::process::exit(2);
    });

    let offset = &arguments[2].clone();
    let offset = offset.parse::<u32>().unwrap();

    let line = chip
        .request_reading_lines(&[offset], exe_name)
        .unwrap_or_else(|err| {
            eprintln!("Error while requesting input line: {:?}", err);
            std::process::exit(2)
        });

    let data = line.read().unwrap_or_else(|err| {
        eprintln!("Error while reading data: {:?}", err);
        std::process::exit(2)
    });

    println!("{}", data[0]);
}
