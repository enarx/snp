// SPDX-License-Identifier: Apache-2.0

//! Operations for managing the SEV platform.

#[cfg(target_os = "linux")]
mod linux;
mod types;

use super::*;
use std::fmt::Debug;

#[cfg(target_os = "linux")]
pub use linux::Firmware;

/// The platform state.
///
/// The underlying SEV-SNP platform behaves like a state machine and
/// can only perform certain actions while it is in certain states.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum State {
    /// The platform is uninitialized.
    Uninitialized,

    /// The platform is initialized, but not currently managing any
    /// guests.
    Initialized,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = match self {
            State::Uninitialized => "Uninitialized",
            State::Initialized => "Initialized",
        };
        write!(f, "{}", state)
    }
}

/// Information regarding the SEV-SNP platform's current status.
#[derive(Clone, Debug, PartialEq)]
pub struct Status {
    /// The build number.
    pub build: Build,

    /// The platform's current state.
    pub state: State,

    /// The number of valid guests supervised by this platform.
    pub guests: u32,

    /// The installed TCB version.
    pub tcb_version: u64,
}

/// System wide configuration value for SNP.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExtConfig {
    /// Address of the struct sev_user_data_snp_config or 0 when reported_tcb does not need to be
    /// updated.
    pub config_address: u64,
    /// Address of extended guest request certificate chain or 0 when previous certificate should
    /// be removed on SNP_SET_EXT_CONFIG.
    pub certs_address: u64,
    /// Length of the certs.
    pub certs_len: u32,
}
