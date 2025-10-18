// SPDX-FileCopyrightText: 2025 Dhruvin Gandhi <contact@dhru.vin>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::io;

use windows::Win32::Storage::FileSystem::{CreateIoRing, HIORING, IORING_VERSION_3};

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
        let handle = unsafe {
            CreateIoRing(
                IORING_VERSION_3,
                Default::default(),
                self.min_sq_len,
                self.min_cq_len,
            )
        }?;
        Ok(Runtime { _handle: handle })
    }
}

pub(super) struct Runtime {
    _handle: HIORING,
}
