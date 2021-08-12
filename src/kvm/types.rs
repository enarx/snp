// SPDX-License-Identifier: Apache-2.0

use crate::launch::*;

use std::marker::PhantomData;

/// Initialize the SEV-SNP platform in KVM.
#[repr(C, packed)]
pub struct Init {
    /// Reserved space, must be always set to 0 when issuing the ioctl.
    flags: u64,
}

impl Default for Init {
    fn default() -> Self {
        Self { flags: 0 }
    }
}

/// Initialize the flow to launch a guest.
#[repr(C)]
pub struct LaunchStart<'a> {
    /// Guest policy. See Table 7 of the AMD SEV-SNP Firmware
    /// specification for a description of the guest policy structure.
    policy: u64,

    /// Userspace address of migration agent
    ma_uaddr: u64,

    /// 1 if this guest is associated with a migration agent. Otherwise 0.
    ma_en: u8,

    /// 1 if this launch flow is launching an IMI for the purpose of
    /// guest-assisted migration. Otherwise 0.
    imi_en: u8,

    /// Hypervisor provided value to indicate guest OS visible workarounds.
    /// The format is hypervisor defined.
    gosvw: [u8; 16],

    _phantom: PhantomData<&'a ()>,
}

impl<'a> LaunchStart<'a> {
    pub fn new(start: &'a Start) -> Self {
        Self {
            policy: start.policy.as_u64(),
            ma_uaddr: start.ma_uaddr,
            ma_en: start.ma_en as _,
            imi_en: start.imi_en as _,
            gosvw: start.gosvw,
            _phantom: PhantomData,
        }
    }
}
