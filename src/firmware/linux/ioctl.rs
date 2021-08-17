// SPDX-License-Identifier: Apache-2.0

//! A collection of type-safe ioctl implementations for the AMD Secure Encrypted Virtualization -
//! Secure Nested Paging (SEV-SNP) platform. These ioctls are exported by the Linux kernel.

use crate::firmware::types::*;

use iocuddle::*;
use sev_iocuddle::impl_const_id;
use sev_iocuddle::sev::{Command, Id};

// These enum ordinal values are defined in the Linux kernel
// source code: include/uapi/linux/psp-sev.h
impl_const_id! {
    pub Id => u32;
    PlatformStatus = 256,
    SetExtConfig<'_> = 257,
    GetExtConfig<'_> = 258,
}

// SEV-SNP ioctls are grouped with other SEV-ioctls.
const SEV: Group = Group::new(b'S');

/// Return information about the current status and capabilities of the SEV-SNP platform.
pub const PLATFORM_STATUS: Ioctl<WriteRead, &Command<PlatformStatus>> =
    unsafe { SEV.write_read(0) };

/// Set the system wide configuration values for SNP.
pub const SET_EXT_CONFIG: Ioctl<WriteRead, &Command<SetExtConfig>> = unsafe { SEV.write_read(0) };

/// Get the system wide configuration values for SNP.
pub const GET_EXT_CONFIG: Ioctl<WriteRead, &Command<GetExtConfig>> = unsafe { SEV.write_read(0) };
