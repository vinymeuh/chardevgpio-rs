// Copyright 2022 VinyMeuh. All rights reserved.
// Use of the source code is governed by a MIT-style license that can be found in the LICENSE file.

use libc::c_char;
use nix::{ioctl_read, ioctl_readwrite};

const GPIO_IOCTL_MAGIC: u8 = 0xB4;

const GPIO_GET_CHIPINFO_IOCTL: u8 = 0x01;
const GPIO_GET_LINEINFO_IOCTL: u8 = 0x02;
//const GPIO_GET_LINEHANDLE_IOCTL: u8 = 0x03;
//const GPIO_GET_LINEEVENT_IOCTL: u8 = 0x04;

#[repr(C)]
pub struct gpiochip_info {
    pub name: [c_char; 32],
    pub label: [c_char; 32],
    pub lines: u32,
}

#[repr(C)]
pub struct gpioline_info {
    pub line_offset: u32,
    pub flags: u32,
    pub name: [c_char; 32],
    pub consumer: [c_char; 32],
}

ioctl_read!(gpio_get_chipinfo_ioctl, GPIO_IOCTL_MAGIC, GPIO_GET_CHIPINFO_IOCTL, gpiochip_info);
ioctl_readwrite!(gpio_get_lineinfo_ioctl, GPIO_IOCTL_MAGIC, GPIO_GET_LINEINFO_IOCTL, gpioline_info);
