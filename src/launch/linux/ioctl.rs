// SPDX-License-Identifier: Apache-2.0

use crate::kvm::types::*;

use iocuddle::*;

use sev_iocuddle::impl_const_id;
use sev_iocuddle::kvm::*;
use sev_iocuddle::sev::Id;

impl_const_id! {
    pub Id => u32;
    Init = 256,
    LaunchStart<'_> = 257,
    LaunchUpdate<'_> = 258,
    LaunchFinish<'_> = 259,
}

// Note: the iocuddle::Ioctl::lie() constructor has been used here because
// KVM_MEMORY_ENCRYPT_OP ioctl was defined like this:
//
// _IOWR(KVMIO, 0xba, unsigned long)
//
// Instead of something like this:
//
// _IOWR(KVMIO, 0xba, struct kvm_sev_cmd)
//
// which would require extra work to wrap around the design decision for
// that ioctl.

/// Initialize the SEV-SNP platform in KVM.
pub const SNP_INIT: Ioctl<WriteRead, &Command<Init>> = unsafe { ENC_OP.lie() };

/// Initialize the flow to launch a guest.
pub const SNP_LAUNCH_START: Ioctl<WriteRead, &Command<LaunchStart>> = unsafe { ENC_OP.lie() };

/// Insert pages into the guest physical address space.
pub const SNP_LAUNCH_UPDATE: Ioctl<WriteRead, &Command<LaunchUpdate>> = unsafe { ENC_OP.lie() };

/// Complete the guest launch flow.
pub const SNP_LAUNCH_FINISH: Ioctl<WriteRead, &Command<LaunchFinish>> = unsafe { ENC_OP.lie() };
