// SPDX-License-Identifier: Apache-2.0

/// Launcher API
pub mod launcher;

#[cfg(target_os = "linux")]
mod linux;

use crate::kvm::types::KVM_SEV_SNP_FINISH_DATA_SIZE;

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

/// Encapsulates the various data needed to begin the update process.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Update {
    /// Indicates that this page is part of the IMI of the guest.
    pub imi_page: bool,

    /// Encoded page type.
    pub page_type: PageType,

    /// VMPL3 permission mask.
    pub vmpl3_perms: VmplPerms,

    /// VMPL2 permission mask.
    pub vmpl2_perms: VmplPerms,

    /// VMPL1 permission mask.
    pub vmpl1_perms: VmplPerms,
}

bitflags! {
    #[derive(Default, Deserialize, Serialize)]
    /// VMPL permission masks.
    pub struct VmplPerms: u8 {
        /// Page is readable by the VMPL.
        const READ =                1;

        /// Page is writeable by the VMPL.
        const WRITE =               1 << 1;

        /// Page is executable by the VMPL in CPL3.
        const EXECUTE_USER =        1 << 2;

        /// Page is executable by the VMPL in CPL2, CPL1, and CPL0.
        const EXECUTE_SUPERVISOR =  1 << 3;
    }
}

/// Encoded page types for a launch update. See Table 58 of the SNP Firmware
/// specification for further details.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum PageType {
    /// A normal data page.
    Normal,

    /// A VMSA page.
    Vmsa,

    /// A page full of zeroes.
    Zero,

    /// A page that is encrypted but not measured
    Unmeasured,

    /// A page for the firmware to store secrets for the guest.
    Secrets,

    /// A page for the hypervisor to provide CPUID function values.
    Cpuid,
}

impl PageType {
    /// Get the encoded value for a page type. See Table 58 of the SNP
    /// Firmware specification for further details.
    pub fn value(self) -> u8 {
        match self {
            PageType::Normal => 0x1,
            PageType::Vmsa => 0x2,
            PageType::Zero => 0x3,
            PageType::Unmeasured => 0x4,
            PageType::Secrets => 0x5,
            PageType::Cpuid => 0x6,
        }
    }
}

/// Encapsulates the data needed to complete a guest launch.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct Finish {
    /// Userspace address of the ID block. Ignored if ID_BLOCK_EN is 0.
    pub id_block_uaddr: u64,

    /// Userspace address of the authentication information of the ID block. Ignored if ID_BLOCK_EN is 0.
    pub id_auth_uaddr: u64,

    /// Indicates that the ID block is present.
    pub id_block_en: bool,

    /// Indicates that the author key is present in the ID authentication information structure.
    /// Ignored if ID_BLOCK_EN is 0.
    pub auth_key_en: bool,

    /// Opaque host-supplied data to describe the guest. The firmware does not interpret this value.
    pub host_data: [u8; KVM_SEV_SNP_FINISH_DATA_SIZE],
}
