// SPDX-License-Identifier: Apache-2.0

/// Launcher API
pub mod launcher;

#[cfg(target_os = "linux")]
mod linux;

use super::*;
use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    /// Configurable SNP Policy options.
    #[derive(Default, Deserialize, Serialize)]
    pub struct PolicyFlags: u16 {
        /// Enable if SMT is enabled in the host machine.
        const SMT =         1;

        /// If enabled, association with a migration agent is allowed.
        const MIGRATE_MA =  1 << 2;

        /// If enabled, debugging is allowed.
        const DEBUG =       1 << 3;
    }
}

/// Describes a policy that the AMD Secure Processor will
/// enforce.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct Policy {
    /// The various policy optons are encoded as bit flags.
    pub flags: PolicyFlags,

    /// The desired minimum platform firmware version.
    pub minfw: Version,
}

impl Policy {
    /// Convert a Policy to it's u64 counterpart.
    pub fn as_u64(&self) -> u64 {
        let mut val: u64 = 0;

        let minor_version = u64::from(self.minfw.minor);
        let mut major_version = u64::from(self.minfw.major);

        /*
         * According to the SNP firmware spec, bit 1 of the policy flags is reserved and must
         * always be set to 1. Rather than passing this responsibility off to callers, set this bit
         * every time an ioctl is issued to the kernel.
         */
        let flags = self.flags.bits | 0b10;
        let mut flags_64 = u64::from(flags);

        major_version <<= 8;
        flags_64 <<= 16;

        val |= minor_version;
        val |= major_version;
        val |= flags_64;
        val &= 0x00FFFFFF;

        val
    }
}

/// Encapsulates the various data needed to begin the launch process.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct Start {
    /// Describes a policy that the AMD Secure Processor will enforce.
    pub policy: Policy,

    /// Userspace address of migration agent. Ignored if MA_EN is 0.
    pub ma_uaddr: u64,

    /// Indicates if this guest is associated with a migration agent. Otherwise 0.
    pub ma_en: bool,

    /// Indicates that this launch flow is launching an IMI for the purpose of guest-assisted migration.
    pub imi_en: bool,

    /// Hypervisor provided value to indicate guest OS visible workarounds.The format is hypervisor defined.
    pub gosvw: [u8; 16],
}
