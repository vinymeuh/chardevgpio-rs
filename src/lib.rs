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
pub enum Error {
    IOError(std::io::Error),
    IoctlError(nix::Error),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IOError(err)
    }
}

impl From<nix::Error> for Error {
    fn from(err: nix::Error) -> Error {
        Error::IoctlError(err)
    }
}

type Result<T> = std::result::Result<T, Error>;

pub struct Chip {
    name: String,
    label: String,
    lines: u32,
    file: File,
}

pub struct LineInfo {
    offset: u32,
    flags: u32,
    name: String,
    consumer: String,
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
                    .into_string().unwrap_or_default()
            },
            label: unsafe {
                CStr::from_ptr(info.label.as_ptr())
                    .to_owned()
                    .into_string().unwrap_or_default()
            },
            lines: info.lines,
            file: f,
        })
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn label(&self) -> &str {
        self.label.as_str()
    }

    pub fn lines(&self) -> u32 {
        self.lines
    }

    pub fn line_info(&mut self, offset: u32) -> Result<LineInfo> {
        let mut info: uapi::gpioline_info = unsafe { mem::zeroed() };
        info.line_offset = offset;
        unsafe { uapi::gpio_get_lineinfo_ioctl(self.file.as_raw_fd(), &mut info)? };

        Ok(LineInfo {
            offset: info.line_offset,
            flags: info.flags,
            name: unsafe {
                CStr::from_ptr(info.name.as_ptr())
                    .to_owned()
                    .into_string().unwrap_or_default()
            },
            consumer: unsafe {
                CStr::from_ptr(info.consumer.as_ptr())
                    .to_owned()
                    .into_string().unwrap_or_default()
            },           
        })
    }
}


impl LineInfo {
    pub fn offset(&self) -> u32 {
        self.offset
    }

    pub fn flags(&self) -> u32 {
        self.flags
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn consumer(&self) -> &str {
        self.consumer.as_str()
    }
}