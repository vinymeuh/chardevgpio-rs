// Copyright 2022 VinyMeuh. All rights reserved.
// Use of the source code is governed by a MIT-style license that can be found in the LICENSE file.

use libc::{c_char, c_int};
use nix::{ioctl_read, ioctl_readwrite};

// Code in this file mimics directly the Linux kernel code
// For reference see https://elixir.bootlin.com/linux/v5.5.9/source/include/uapi/linux/gpio.h

// Information about a certain GPIO chip
// See https://elixir.bootlin.com/linux/v5.5.9/source/include/uapi/linux/gpio.h#L24
#[repr(C)]
pub struct gpiochip_info {
    pub name: [c_char; 32],
    pub label: [c_char; 32],
    pub lines: u32,
}

/* Informational flags */
pub const GPIOLINE_FLAG_KERNEL: u32 = 1 << 0;
pub const GPIOLINE_FLAG_IS_OUT: u32 = 1 << 1;
pub const GPIOLINE_FLAG_ACTIVE_LOW: u32 = 1 << 2;
pub const GPIOLINE_FLAG_OPEN_DRAIN: u32 = 1 << 3;
pub const GPIOLINE_FLAG_OPEN_SOURCE: u32 = 1 << 4;
pub const GPIOLINE_FLAG_BIAS_PULL_UP: u32 = 1 << 5;
pub const GPIOLINE_FLAG_BIAS_PULL_DOWN: u32 = 1 << 6;
pub const GPIOLINE_FLAG_BIAS_DISABLE: u32 = 1 << 7;

// Information about a certain GPIO line
// See https://elixir.bootlin.com/linux/v5.5.9/source/include/uapi/linux/gpio.h#L52
#[repr(C)]
pub struct gpioline_info {
    pub line_offset: u32,
    pub flags: u32,
    pub name: [c_char; 32],
    pub consumer: [c_char; 32],
}

/* Maximum number of requested handles */
pub const GPIOHANDLES_MAX: usize = 64;

/* Linerequest flags */
pub const GPIOHANDLE_REQUEST_INPUT: u32 = 1 << 0;
pub const GPIOHANDLE_REQUEST_OUTPUT: u32 = 1 << 1;
pub const GPIOHANDLE_REQUEST_ACTIVE_LOW: u32 = 1 << 2;
pub const GPIOHANDLE_REQUEST_OPEN_DRAIN: u32 = 1 << 3;
pub const GPIOHANDLE_REQUEST_OPEN_SOURCE: u32 = 1 << 4;
pub const GPIOHANDLE_REQUEST_BIAS_PULL_UP: u32 = 1 << 5;
pub const GPIOHANDLE_REQUEST_BIAS_PULL_DOWN: u32 = 1 << 6;
pub const GPIOHANDLE_REQUEST_BIAS_DISABLE: u32 = 1 << 7;

// Information about a GPIO handle request
// See https://elixir.bootlin.com/linux/v5.5.9/source/include/uapi/linux/gpio.h#L94
#[repr(C)]
pub struct gpiohandle_request {
    pub lineoffsets: [u32; GPIOHANDLES_MAX],
    pub flags: u32,
    pub default_values: [u8; GPIOHANDLES_MAX],
    pub consumer_label: [c_char; 32],
    pub lines: u32,
    pub fd: c_int,
}

// Configuration for a GPIO handle request
// See https://elixir.bootlin.com/linux/v5.5.9/source/include/uapi/linux/gpio.h#L113
#[repr(C)]
pub struct gpiohandle_config {
    pub flags: u32,
    pub default_values: [u8; GPIOHANDLES_MAX],
    pub padding: [u32; 4],
}

// Information of values on a GPIO handle
// See https://elixir.bootlin.com/linux/v5.5.9/source/include/uapi/linux/gpio.h#L127
#[repr(C)]
pub struct gpiohandle_data {
    pub values: [u8; GPIOHANDLES_MAX],
}

/* Eventrequest flags */
pub const GPIOEVENT_REQUEST_RISING_EDGE: u32 = 1 << 0;
pub const GPIOEVENT_REQUEST_FALLING_EDGE: u32 = 1 << 1;
pub const GPIOEVENT_REQUEST_BOTH_EDGES: u32 = (1 << 0) | (1 << 1);

// Information of values on a GPIO handle
// See https://elixir.bootlin.com/linux/v5.5.9/source/include/uapi/linux/gpio.h#L153
#[repr(C)]
pub struct gpioevent_request {
    pub lineoffset: u32,
    pub handleflags: u32,
    pub eventflags: u32,
    pub consumer: [c_char; 32],
    pub fd: i32,
}

// The actual event being pushed to userspace
// See https://elixir.bootlin.com/linux/v5.5.9/source/include/uapi/linux/gpio.h#L172
#[repr(C)]
pub struct gpioevent_data {
    pub timestamp: u64,
    pub id: u32,
}

/* GPIO event types */
pub const GPIOEVENT_EVENT_RISING_EDGE: u32 = 0x01;
pub const GPIOEVENT_EVENT_FALLING_EDGE: u32 = 0x02;

/* IOCTL syscalls */
const GPIO_IOCTL_MAGIC: u8 = 0xB4;

const GPIO_GET_CHIPINFO_IOCTL: u8 = 0x01;
const GPIO_GET_LINEINFO_IOCTL: u8 = 0x02;
const GPIO_GET_LINEHANDLE_IOCTL: u8 = 0x03;
const GPIO_GET_LINEEVENT_IOCTL: u8 = 0x04;

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

ioctl_readwrite!(
    gpio_get_linehandle_ioctl,
    GPIO_IOCTL_MAGIC,
    GPIO_GET_LINEHANDLE_IOCTL,
    gpiohandle_request
);

ioctl_readwrite!(
    gpio_get_lineevent_ioctl,
    GPIO_IOCTL_MAGIC,
    GPIO_GET_LINEEVENT_IOCTL,
    gpioevent_request
);

const GPIOHANDLE_GET_LINE_VALUES_IOCTL: u8 = 0x08;
const GPIOHANDLE_SET_LINE_VALUES_IOCTL: u8 = 0x09;
const GPIOHANDLE_SET_CONFIG_IOCTL: u8 = 0x0a;

ioctl_readwrite!(
    gpiohandle_get_line_values_ioctl,
    GPIO_IOCTL_MAGIC,
    GPIOHANDLE_GET_LINE_VALUES_IOCTL,
    gpiohandle_data
);

ioctl_readwrite!(
    gpiohandle_set_line_values_ioctl,
    GPIO_IOCTL_MAGIC,
    GPIOHANDLE_SET_LINE_VALUES_IOCTL,
    gpiohandle_data
);

ioctl_readwrite!(
    gpiohandle_set_config_ioctl,
    GPIO_IOCTL_MAGIC,
    GPIOHANDLE_SET_CONFIG_IOCTL,
    gpiohandle_config
);
