// Copyright 2022 VinyMeuh. All rights reserved.
// Use of the source code is governed by a MIT-style license that can be found in the LICENSE file.

use std::ffi::CStr;
use std::fs::File;
use std::mem;
use std::os::unix::io::{AsRawFd};
use std::path::Path;

use nix;

pub mod uapi;

#[derive(Debug)]
pub enum ChardevgpioError {
    IOError(std::io::Error),
    IoctlError(nix::Error),
}

impl From<std::io::Error> for ChardevgpioError {
    fn from(err: std::io::Error) -> ChardevgpioError {
        ChardevgpioError::IOError(err)
    }
}

impl From<nix::Error> for ChardevgpioError {
    fn from(err: nix::Error) -> ChardevgpioError {
        ChardevgpioError::IoctlError(err)
    }
}

type Result<T> = std::result::Result<T, ChardevgpioError>;

pub struct Chip {
    pub name: String,
    pub label: String,
    pub lines: u32,
    file: File,
}

impl Chip {
    pub fn new(path: &Path) -> Result<Self> {
        let f = File::open(path)?;

        let mut info: uapi::gpiochip_info = unsafe { mem::zeroed() };
        unsafe { uapi::gpio_get_chipinfo_ioctl(f.as_raw_fd(), &mut info)? };

        Ok(Self {
            name: unsafe {
                CStr::from_ptr(info.name.as_ptr())
                    .to_owned()
                    .into_string().unwrap() // TODO: remove unwrap()
            },
            label: unsafe {
                CStr::from_ptr(info.label.as_ptr())
                    .to_owned()
                    .into_string().unwrap() // TODO: remove unwrap()
            },
            lines: info.lines,
            file: f,
        })
    }
}
