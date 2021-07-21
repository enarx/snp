// SPDX-License-Identifier: Apache-2.0

use crate::Version;

/// Query the SEV-SNP platform status.
///
/// (Chapter 8.3; Table 38)
#[derive(Default)]
#[repr(C, packed)]
pub struct PlatformStatus {
    /// The firmware API version (major.minor)
    pub version: Version,

    /// The platform state.
    pub state: u8,

    /// The platform build ID.
    pub build_id: u32,

    /// The number of valid guests maintained by the SEV-SNP firmware.
    pub guest_count: u32,

    /// The installed TCB version.
    pub tcb_version: u64,
}
