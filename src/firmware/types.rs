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

/// Set the system wide SNP variables.
#[derive(Default)]
#[repr(C, packed)]
pub struct SetExtConfig {
    /// Address of the struct sev_user_data_snp_config or 0 when reported_tcb does not need to be
    /// updated.
    pub config_address: u64,
    /// Address of extended guest request certificate chain or 0 when previous certificate should
    /// be removed on SNP_SET_EXT_CONFIG.
    pub certs_address: u64,
    /// Length of the certs.
    pub certs_len: u32,
}

impl SetExtConfig {
    pub fn new(config_address: u64, certs_address: u64, certs_len: u32) -> Self {
        Self {
            config_address,
            certs_address,
            certs_len,
        }
    }
}

/// Get the system wide SNP variables.
#[derive(Default)]
#[repr(C, packed)]
pub struct GetExtConfig {
    /// Address of the struct sev_user_data_snp_config or 0 when reported_tcb does not need to be
    /// updated.
    pub config_address: u64,
    /// Address of extended guest request certificate chain or 0 when previous certificate should
    /// be removed on SNP_SET_EXT_CONFIG.
    pub certs_address: u64,
    /// Length of the certs.
    pub certs_len: u32,
}
