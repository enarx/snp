// SPDX-License-Identifier: Apache-2.0

//! Operations for managing the SEV platform.

#[cfg(target_os = "linux")]
mod linux;
mod types;

use super::*;
use std::fmt::Debug;

#[cfg(target_os = "linux")]
pub use linux::Firmware;

/// There are a number of error conditions that can occur between this
/// layer all the way down to the SEV-SNP platform. Most of these cases have
/// been enumerated; however, there is a possibility that some error
/// conditions are not encapsulated here.
#[derive(Debug)]
pub enum Indeterminate<T: Debug> {
    /// The error condition is known.
    Known(T),

    /// The error condition is unknown.
    Unknown,
}

/// Error conditions returned by the SEV platform or by layers above it
/// (i.e., the Linux kernel).
///
/// These error conditions are documented in the AMD SEV API spec, but
/// their documentation has been copied here for completeness.
#[derive(Debug)]
pub enum Error {
    /// Something went wrong when communicating with the "outside world"
    /// (kernel, SEV platform).
    IoError(std::io::Error),

    /// The platform state is invalid for this command.
    InvalidPlatformState,

    /// The guest state is invalid for this command.
    InvalidGuestState,

    /// The platform configuration is invalid.
    InvalidConfig,

    /// A memory buffer is too small.
    InvalidLen,

    /// The platform is already owned.
    AlreadyOwned,

    /// The certificate is invalid.
    InvalidCertificate,

    /// Request is not allowed by guest policy.
    PolicyFailure,

    /// The guest is inactive.
    Inactive,

    /// The address provided is invalid.
    InvalidAddress,

    /// The provided signature is invalid.
    BadSignature,

    /// The provided measurement is invalid.
    BadMeasurement,

    /// The ASID is already owned.
    AsidOwned,

    /// The ASID is invalid.
    InvalidAsid,

    /// WBINVD instruction required.
    WbinvdRequired,

    /// `DF_FLUSH` invocation required.
    DfFlushRequired,

    /// The guest handle is invalid.
    InvalidGuest,

    /// The command issued is invalid.
    InvalidCommand,

    /// The guest is active.
    Active,

    /// A hardware condition has occurred affecting the platform. It is safe
    /// to re-allocate parameter buffers.
    HardwarePlatform,

    /// A hardware condition has occurred affecting the platform. Re-allocating
    /// parameter buffers is not safe.
    HardwareUnsafe,

    /// Feature is unsupported.
    Unsupported,

    /// A given parameter is invalid.
    InvalidParam,

    /// The SEV firmware has run out of a resource required to carry out the
    /// command.
    ResourceLimit,

    /// The SEV platform observed a failed integrity check.
    SecureDataInvalid,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_description = match self {
            Error::IoError(_) => "I/O Error",
            Error::InvalidPlatformState => "Invalid platform state",
            Error::InvalidGuestState => "Invalid guest state",
            Error::InvalidConfig => "Platform configuration invalid",
            Error::InvalidLen => "Memory buffer too small",
            Error::AlreadyOwned => "Platform is already owned",
            Error::InvalidCertificate => "Invalid certificate",
            Error::PolicyFailure => "Policy failure",
            Error::Inactive => "Guest is inactive",
            Error::InvalidAddress => "Provided address is invalid",
            Error::BadSignature => "Provided signature is invalid",
            Error::BadMeasurement => "Provided measurement is invalid",
            Error::AsidOwned => "ASID is already owned",
            Error::InvalidAsid => "ASID is invalid",
            Error::WbinvdRequired => "WBINVD instruction required",
            Error::DfFlushRequired => "DF_FLUSH invocation required",
            Error::InvalidGuest => "Guest handle is invalid",
            Error::InvalidCommand => "Issued command is invalid",
            Error::Active => "Guest is active",
            Error::HardwarePlatform => {
                "Hardware condition occured, safe to re-allocate parameter buffers"
            }
            Error::HardwareUnsafe => {
                "Hardware condition occured, unsafe to re-allocate parameter buffers"
            }
            Error::Unsupported => "Feature is unsupported",
            Error::InvalidParam => "Given parameter is invalid",
            Error::ResourceLimit => {
                "SEV firmware has run out of required resources to carry out command"
            }
            Error::SecureDataInvalid => "SEV platform observed a failed integrity check",
        };
        write!(f, "{}", err_description)
    }
}

impl From<std::io::Error> for Error {
    #[inline]
    fn from(error: std::io::Error) -> Error {
        Error::IoError(error)
    }
}

impl From<std::io::Error> for Indeterminate<Error> {
    #[inline]
    fn from(error: std::io::Error) -> Indeterminate<Error> {
        Indeterminate::Known(error.into())
    }
}

impl From<u32> for Indeterminate<Error> {
    #[inline]
    fn from(error: u32) -> Indeterminate<Error> {
        Indeterminate::Known(match error {
            0 => std::io::Error::last_os_error().into(),
            1 => Error::InvalidPlatformState,
            2 => Error::InvalidGuestState,
            3 => Error::InvalidConfig,
            4 => Error::InvalidLen,
            5 => Error::AlreadyOwned,
            6 => Error::InvalidCertificate,
            7 => Error::PolicyFailure,
            8 => Error::Inactive,
            9 => Error::InvalidAddress,
            10 => Error::BadSignature,
            11 => Error::BadMeasurement,
            12 => Error::AsidOwned,
            13 => Error::InvalidAsid,
            14 => Error::WbinvdRequired,
            15 => Error::DfFlushRequired,
            16 => Error::InvalidGuest,
            17 => Error::InvalidCommand,
            18 => Error::Active,
            19 => Error::HardwarePlatform,
            20 => Error::HardwareUnsafe,
            21 => Error::Unsupported,
            22 => Error::InvalidParam,
            23 => Error::ResourceLimit,
            24 => Error::SecureDataInvalid,
            _ => return Indeterminate::Unknown,
        })
    }
}

impl std::fmt::Display for Indeterminate<Error> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err = match self {
            Indeterminate::Known(e) => format!("Known error: {}", e),
            Indeterminate::Unknown => "Unknown error".to_string(),
        };

        write!(f, "{}", err)
    }
}

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
