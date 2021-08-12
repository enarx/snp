// SPDX-License-Identifier: Apache-2.0

use crate::kvm::types::*;
use crate::launch::linux::ioctl::*;

use std::io::Result;
use std::os::unix::io::AsRawFd;

/// A new SNP-encrypted VM instance, one that was not previously running.
pub struct New;

/// Facilitates the correct execution of the V launch process.
pub struct Launcher<'a, T, U: AsRawFd, V: AsRawFd> {
    _state: T,
    kvm: &'a mut U,
    sev: &'a mut V,
}

impl<'a, U: AsRawFd, V: AsRawFd> Launcher<'a, New, U, V> {
    /// Begin the SEV-SNP launch process by creating a Launcher and issuing the
    /// KVM_SNP_INIT ioctl.
    pub fn new(kvm: &'a mut U, sev: &'a mut V) -> Result<Self> {
        let launcher = Launcher {
            _state: New,
            kvm,
            sev,
        };

        let init = Init::default();

        let mut cmd = Command::from(launcher.sev, &init);
        SNP_INIT.ioctl(launcher.kvm, &mut cmd)?;

        Ok(launcher)
    }
}
