// SPDX-License-Identifier: Apache-2.0

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
