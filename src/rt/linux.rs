// SPDX-FileCopyrightText: 2025 Dhruvin Gandhi <contact@dhru.vin>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{io, os::fd::OwnedFd};

use rustix::io_uring::{IoringSetupFlags, io_uring_params, io_uring_setup};

pub(super) struct Builder {
    min_sq_len: u32,
    min_cq_len: u32,
}

impl Builder {
    pub(super) fn new(min_sq_len: u32, min_cq_len: u32) -> Self {
        Self {
            min_sq_len,
            min_cq_len,
        }
    }
    pub(super) fn build(self) -> io::Result<Runtime> {
        let mut params = io_uring_params::default();
        params.flags = IoringSetupFlags::CQSIZE;
        params.cq_entries = self.min_cq_len;
        let fd = unsafe { io_uring_setup(self.min_sq_len, &mut params) }?;
        Ok(Runtime { _fd: fd })
    }
}

pub(super) struct Runtime {
    _fd: OwnedFd,
}
