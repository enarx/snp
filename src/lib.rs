// SPDX-License-Identifier: Apache-2.0

//! The `snp` crate provides an implementation of [AMD Secure Encrypted
//! Virtualization - Secure Nested Paging (SEV-SNP)] APIs.
//!
//! The Linux kernel exposes two technically distinct AMD SEV-SNP APIs:
//!
//! 1. An API for managing the SEV-SNP platform itself
//! 2. An API for managing SNP-enabled KVM virtual machines
//!
//! This crate implements both of those APIs and offers them to client
//! code through a flexible and type-safe high level interface.
//!
//! ## Remarks
//!
//! Note that the Linux kernel provides access to these APIs through a set
//! of `ioctl`s that are meant to be called on device nodes (`/dev/kvm` and
//! `/dev/sev`, to be specific). As a result, these `ioctl`s form the substrate
//! of the `snp` crate. Binaries that result from consumers of this crate are
//! expected to run as a process with the necessary privileges to interact
//! with the device nodes.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![allow(unknown_lints)]
#![allow(clippy::identity_op)]
#![allow(clippy::unreadable_literal)]

pub mod firmware;
mod util;

use crate::util::{TypeLoad, TypeSave};

use serde::{Deserialize, Serialize};

use std::io::{Read, Write};

/// Information about the SEV-SNP platform version.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Version {
    /// The major version number.
    pub major: u8,

    /// The minor version number.
    pub minor: u8,
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

/// A description of the SEV-SNP platform's build information.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Build {
    /// The version information.
    pub version: Version,

    /// The build ID.
    pub build: u32,
}

impl std::fmt::Display for Build {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}.{}", self.version, self.build)
    }
}

impl codicon::Decoder<()> for Build {
    type Error = std::io::Error;

    fn decode(mut reader: impl Read, _: ()) -> std::io::Result<Self> {
        reader.load()
    }
}

impl codicon::Encoder<()> for Build {
    type Error = std::io::Error;

    fn encode(&self, mut writer: impl Write, _: ()) -> std::io::Result<()> {
        writer.save(self)
    }
}
