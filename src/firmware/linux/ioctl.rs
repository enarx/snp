// SPDX-License-Identifier: Apache-2.0

//! A collection of type-safe ioctl implementations for the AMD Secure Encrypted Virtualization -
//! Secure Nested Paging (SEV-SNP) platform. These ioctls are exported by the Linux kernel.

use crate::firmware::types::*;
use crate::impl_const_id;

use iocuddle::*;

use std::marker::PhantomData;

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

/// The Rust-flavored, FFI-friendly version of `struct sev_issue_cmd` which is
/// used to pass arguments to the SEV ioctl implementation.
///
/// This struct is defined in the Linux kernel: include/uapi/linux/psp-sev.h
#[repr(C, packed)]
pub struct Command<'a, T: Id> {
    code: u32,
    data: u64,
    error: u32,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T: Id> Command<'a, T> {
    /// Create an SEV-SNP command with the expectation that the host platform/kernel will write to
    /// the caller's address space either to the data held in the `Command.subcmd` field or some
    /// other region specified by the `Command.subcmd` field.
    pub fn from_mut(subcmd: &'a mut T) -> Self {
        Command {
            code: T::ID,
            data: subcmd as *mut T as u64,
            error: 0,
            _phantom: PhantomData,
        }
    }

    /// Create an SEV-SNP command with the expectation that the host platform/kernel *WILL NOT* mutate
    /// the caller's address space in its response. Note: this does not actually prevent the host
    /// platform/kernel from writing to the caller's address space if it wants to. This is primarily
    /// a semantic tool for programming against the SEV-SNP ioctl API.
    pub fn from(subcmd: &'a T) -> Self {
        Command {
            code: T::ID,
            data: subcmd as *const T as u64,
            error: 0,
            _phantom: PhantomData,
        }
    }
}
