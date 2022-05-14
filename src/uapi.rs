// Copyright 2022 VinyMeuh. All rights reserved.
// Use of the source code is governed by a MIT-style license that can be found in the LICENSE file.

use libc::c_char;
use nix::{ioctl_read, ioctl_readwrite};

// Code in this file mimics directly the Linux kernel code
// For reference see https://elixir.bootlin.com/linux/v5.5.9/source/include/uapi/linux/gpio.h

const GPIO_IOCTL_MAGIC: u8 = 0xB4;

const GPIO_GET_CHIPINFO_IOCTL: u8 = 0x01;
const GPIO_GET_LINEINFO_IOCTL: u8 = 0x02;
//const GPIO_GET_LINEHANDLE_IOCTL: u8 = 0x03;
//const GPIO_GET_LINEEVENT_IOCTL: u8 = 0x04;

/* Informational flags */
pub const GPIOLINE_FLAG_KERNEL: u32 = 1 << 0;
pub const GPIOLINE_FLAG_IS_OUT: u32 = 1 << 1;
pub const GPIOLINE_FLAG_ACTIVE_LOW: u32 = 1 << 2;
pub const GPIOLINE_FLAG_OPEN_DRAIN: u32 = 1 << 3;
pub const GPIOLINE_FLAG_OPEN_SOURCE: u32 = 1 << 4;
pub const GPIOLINE_FLAG_BIAS_PULL_UP: u32 = 1 << 5;
pub const GPIOLINE_FLAG_BIAS_PULL_DOWN: u32 = 1 << 6;
pub const GPIOLINE_FLAG_BIAS_DISABLE: u32 = 1 << 7;

// Information about a certain GPIO chip
// See https://elixir.bootlin.com/linux/v5.5.9/source/include/uapi/asm-generic/ioctl.h#L24
#[repr(C)]
pub struct gpiochip_info {
    pub name: [c_char; 32],
    pub label: [c_char; 32],
    pub lines: u32,
}

// Information about a certain GPIO line
// See https://elixir.bootlin.com/linux/v5.5.9/source/include/uapi/asm-generic/ioctl.h#L52
#[repr(C)]
pub struct gpioline_info {
    pub line_offset: u32,
    pub flags: u32,
    pub name: [c_char; 32],
    pub consumer: [c_char; 32],
}

ioctl_read!(
    gpio_get_chipinfo_ioctl,
    GPIO_IOCTL_MAGIC,
    GPIO_GET_CHIPINFO_IOCTL,
    gpiochip_info
);

ioctl_readwrite!(
    gpio_get_lineinfo_ioctl,
    GPIO_IOCTL_MAGIC,
    GPIO_GET_LINEINFO_IOCTL,
    gpioline_info
);
