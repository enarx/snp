// SPDX-License-Identifier: Apache-2.0

use snp::firmware::Firmware;
use snp::launch::launcher::*;

use kvm_bindings::kvm_userspace_memory_region;
use kvm_ioctls::Kvm;
use mmarinus::{perms, Kind, Map};

#[cfg_attr(not(has_sev), ignore)]
#[test]
fn snp() {
    let mut sev = Firmware::open().unwrap();

    let kvm = Kvm::new().unwrap();
    let mut vm = kvm.create_vm().unwrap();

    const MEM_SIZE: usize = 0x1000;
    let address_space = Map::map(MEM_SIZE)
        .anywhere()
        .anonymously()
        .known::<perms::ReadWrite>(Kind::Private)
        .unwrap();

    let mem_region = kvm_userspace_memory_region {
        slot: 0,
        guest_phys_addr: 0,
        memory_size: address_space.size() as _,
        userspace_addr: address_space.addr() as _,
        flags: 0,
    };

    unsafe {
        vm.set_user_memory_region(mem_region).unwrap();
    }

    let _launcher = Launcher::new(&mut vm, &mut sev).unwrap();
}
