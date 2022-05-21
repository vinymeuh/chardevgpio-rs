// Copyright 2022 VinyMeuh. All rights reserved.
// Use of the source code is governed by a MIT-style license that can be found in the LICENSE file.

use std::ffi::CStr;
use std::fs::File;
use std::mem;
use std::ops::BitAnd;
use std::os::unix::io::AsRawFd;
use std::path::Path;

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

/// A GPIO chip controlling a set of lines.
pub struct Chip {
    name: String,
    label: String,
    lines: u32,
    file: File,
}

impl Chip {
    /// Create a Chip for a GPIO character device from its path.
    pub fn new(path: &Path) -> Result<Self> {
        let f = File::open(path)?;

        let mut info: uapi::gpiochip_info = unsafe { mem::zeroed() };
        unsafe { uapi::gpio_get_chipinfo_ioctl(f.as_raw_fd(), &mut info)? };

        Ok(Self {
            name: unsafe {
                CStr::from_ptr(info.name.as_ptr())
                    .to_owned()
                    .into_string()
                    .unwrap_or_default()
            },
            label: unsafe {
                CStr::from_ptr(info.label.as_ptr())
                    .to_owned()
                    .into_string()
                    .unwrap_or_default()
            },
            lines: info.lines,
            file: f,
        })
    }

    /// Return the name of the chip.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Return the label of the chip.
    pub fn label(&self) -> &str {
        self.label.as_str()
    }

    /// Return the number of lines managed by the chip.
    pub fn lines(&self) -> u32 {
        self.lines
    }

    /// Return informations about the requested line.
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
                    .into_string()
                    .unwrap_or_default()
            },
            consumer: unsafe {
                CStr::from_ptr(info.consumer.as_ptr())
                    .to_owned()
                    .into_string()
                    .unwrap_or_default()
            },
        })
    }

    /// Return a set of Lines for read
    pub fn request_reading_lines(&self, lines: &[u32], _consumer: &str) -> Result<Lines> {
        let mut handle = uapi::gpiohandle_request {
            lineoffsets: [0; uapi::GPIOHANDLES_MAX],
            flags: uapi::GPIOHANDLE_REQUEST_INPUT,
            default_values: [0; uapi::GPIOHANDLES_MAX],
            consumer_label: [0; 32],
            lines: lines.len() as u32, // number of lines requested in this request
            fd: 0, // if request successful this field will contain a valid anonymous file handle
        };

        handle.lineoffsets[0..lines.len()].copy_from_slice(&lines); // TODO check lines.len <= uapi::GPIOHANDLES_MAX or self.lines)

        unsafe { uapi::gpio_get_linehandle_ioctl(self.file.as_raw_fd(), &mut handle)? };

        Ok(Lines { h: handle })
    }
}

/// Informations about a GPIO line.
pub struct LineInfo {
    offset: u32,
    flags: u32,
    name: String,
    consumer: String,
}

impl LineInfo {
    /// Return the offset number of the line.
    pub fn offset(&self) -> u32 {
        self.offset
    }

    /// Return the name of the line.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Return the consumer of the line.
    pub fn consumer(&self) -> &str {
        self.consumer.as_str()
    }

    /// Return true if the line is configured as kernel.
    pub fn is_kernel(&self) -> bool {
        self.flags.bitand(uapi::GPIOLINE_FLAG_KERNEL) == uapi::GPIOLINE_FLAG_KERNEL
    }

    /// Return true if the line is configured as an output.
    pub fn is_output(&self) -> bool {
        self.flags.bitand(uapi::GPIOLINE_FLAG_IS_OUT) == uapi::GPIOLINE_FLAG_IS_OUT
    }

    /// Return true if the line is configured as active low.
    pub fn is_active_low(&self) -> bool {
        self.flags.bitand(uapi::GPIOLINE_FLAG_ACTIVE_LOW) == uapi::GPIOLINE_FLAG_ACTIVE_LOW
    }

    /// Return true if the line is configured as open drain.
    pub fn is_open_drain(&self) -> bool {
        self.flags.bitand(uapi::GPIOLINE_FLAG_OPEN_DRAIN) == uapi::GPIOLINE_FLAG_OPEN_DRAIN
    }

    /// Return true if the line is configured as open source.
    pub fn is_open_source(&self) -> bool {
        self.flags.bitand(uapi::GPIOLINE_FLAG_OPEN_SOURCE) == uapi::GPIOLINE_FLAG_OPEN_SOURCE
    }

    /// Return true if the line is configured as bias pull up.
    pub fn is_bias_pull_up(&self) -> bool {
        self.flags.bitand(uapi::GPIOLINE_FLAG_BIAS_PULL_UP) == uapi::GPIOLINE_FLAG_BIAS_PULL_UP
    }

    /// Return true if the line is configured as bias pull down.
    pub fn is_bias_pull_down(&self) -> bool {
        self.flags.bitand(uapi::GPIOLINE_FLAG_BIAS_PULL_DOWN) == uapi::GPIOLINE_FLAG_BIAS_PULL_DOWN
    }

    /// Return true if the line is disabled.
    pub fn is_disable(&self) -> bool {
        self.flags.bitand(uapi::GPIOLINE_FLAG_BIAS_DISABLE) == uapi::GPIOLINE_FLAG_BIAS_DISABLE
    }
}

/// A set of lines that can be used simultaneously
pub struct Lines {
    h: uapi::gpiohandle_request,
}

impl Lines {
    pub fn read(&self) -> Result<Vec<u8>> {
        let mut hdata = uapi::gpiohandle_data {
            values: [0; uapi::GPIOHANDLES_MAX],
        };

        unsafe { uapi::gpiohandle_get_line_values_ioctl(self.h.fd, &mut hdata)? };
        Ok(hdata.values.to_vec())
    }
}
