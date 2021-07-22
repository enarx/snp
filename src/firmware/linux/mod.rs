// SPDX-License-Identifier: Apache-2.0

//! Operations for managing the SEV-SNP platform.

mod ioctl;

use std::fs::{File, OpenOptions};
use std::os::unix::io::{AsRawFd, RawFd};

use super::*;
use linux::ioctl::*;
use types::*;

/// A handle to the SEV-SNP platform.
pub struct Firmware(File);

impl Firmware {
    /// Create a handle to the SEV-SNP platform.
    pub fn open() -> std::io::Result<Firmware> {
        Ok(Firmware(
            OpenOptions::new().read(true).write(true).open("/dev/sev")?,
        ))
    }

    /// Query the SNP platform status.
    pub fn platform_status(&mut self) -> Result<Status, Indeterminate<Error>> {
        let mut info: PlatformStatus = Default::default();
        PLATFORM_STATUS.ioctl(&mut self.0, &mut Command::from_mut(&mut info))?;

        Ok(Status {
            build: Build {
                version: Version {
                    major: info.version.major,
                    minor: info.version.minor,
                },
                build: info.build_id,
            },
            guests: info.guest_count,
            tcb_version: info.tcb_version,
            state: match info.state {
                0 => State::Uninitialized,
                1 => State::Initialized,
                _ => return Err(Indeterminate::Unknown),
            },
        })
    }

    /// Set the system-wide SNP variables.
    pub fn set_ext_config(
        &mut self,
        config_address: u64,
        certs_address: u64,
        certs_len: u32,
    ) -> Result<(), Indeterminate<Error>> {
        let info: SetExtConfig = SetExtConfig::new(config_address, certs_address, certs_len);
        match SET_EXT_CONFIG.ioctl(&mut self.0, &mut Command::from(&info)) {
            Ok(_) => Ok(()),
            Err(e) => Err(Indeterminate::Known(firmware::Error::IoError(e))),
        }
    }

    /// Get the system-wide SNP variables.
    pub fn get_ext_config(&mut self) -> Result<ExtConfig, Indeterminate<Error>> {
        let mut info: GetExtConfig = Default::default();
        GET_EXT_CONFIG.ioctl(&mut self.0, &mut Command::from_mut(&mut info))?;

        Ok(ExtConfig {
            config_address: info.config_address,
            certs_address: info.certs_address,
            certs_len: info.certs_len,
        })
    }
}

impl AsRawFd for Firmware {
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}
