// Copyright 2024 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//! This crate provides a logic for creating an ext2 filesystem on memory.

#![cfg(target_os = "linux")]
#![deny(missing_docs)]

mod fs;
mod superblock;

pub use fs::Ext2;
