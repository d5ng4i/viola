// SPDX-FileCopyrightText: 2025 Dhruvin Gandhi <contact@dhru.vin>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::io;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use linux as inner;

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
use windows as inner;

#[allow(dead_code)]
pub struct Runtime(inner::Runtime);

pub struct Builder(inner::Builder);

impl Builder {
    pub fn new(min_sq_len: u32, min_cq_len: u32) -> Self {
        Self(inner::Builder::new(min_sq_len, min_cq_len))
    }
    pub fn build(self) -> io::Result<Runtime> {
        self.0.build().map(Runtime)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        assert!(Builder::new(128, 256).build().is_ok());
    }
}
