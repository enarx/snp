// SPDX-License-Identifier: Apache-2.0

use crate::Version;

use std::marker::PhantomData;

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
pub struct SetExtConfig<'a> {
    /// Address of the struct sev_user_data_snp_config or 0 when reported_tcb does not need to be
    /// updated.
    pub config_address: u64,

    /// Address of extended guest request certificate chain or 0 when previous certificate should
    /// be removed on SNP_SET_EXT_CONFIG.
    pub certs_address: u64,

    /// Length of the certs.
    pub certs_len: u32,

    _phantom: PhantomData<&'a ()>,
}

impl<'a> SetExtConfig<'a> {
    pub fn new(config_address: u64, certs_address: u64, certs_len: u32) -> Self {
        Self {
            config_address,
            certs_address,
            certs_len,
            _phantom: PhantomData,
        }
    }
}

/// Get the system wide SNP variables.
#[derive(Default)]
#[repr(C, packed)]
pub struct GetExtConfig<'a> {
    /// Address of the struct sev_user_data_snp_config or 0 when reported_tcb does not need to be
    /// updated.
    pub config_address: u64,

    /// Address of extended guest request certificate chain or 0 when previous certificate should
    /// be removed on SNP_SET_EXT_CONFIG.
    pub certs_address: u64,

    /// Length of the certs.
    pub certs_len: u32,

    _phantom: PhantomData<&'a ()>,
}
