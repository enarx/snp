// SPDX-License-Identifier: Apache-2.0

use sev_iocuddle::sev::*;
use snp::firmware::Firmware;
use snp::launch::launcher::*;
use snp::launch::*;

use kvm_bindings::kvm_userspace_memory_region;
use kvm_ioctls::Kvm;
use mmarinus::{perms, Kind, Map};

#[cfg_attr(not(has_sev), ignore)]
#[test]
fn snp() {
    let mut sev = Firmware::open().unwrap();

    let kvm = Kvm::new().unwrap();
    let mut vm = kvm.create_vm().unwrap();

    let status = sev.platform_status().unwrap();

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

    let launcher = Launcher::new(&mut vm, &mut sev).unwrap();

    let x: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let mut start = Start {
        policy: Policy {
            flags: PolicyFlags::SMT,
            minfw: Version {
                major: status.build.version.major,
                minor: status.build.version.minor,
            },
        },
        ma_uaddr: 0,
        ma_en: false,
        imi_en: false,
        gosvw: x,
    };

    let mut launcher = launcher.start(&mut start).unwrap();

    let update = Update {
        imi_page: false,
        page_type: PageType::Normal,
        vmpl3_perms: VmplPerms::default(),
        vmpl2_perms: VmplPerms::default(),
        vmpl1_perms: VmplPerms::default(),
    };

    launcher
        .update_data(address_space.as_ref(), &update)
        .unwrap();

    let hd: [u8; 32] = [0; 32];

    let finish = Finish {
        id_block_uaddr: 0,
        id_auth_uaddr: 0,
        id_block_en: false,
        auth_key_en: false,
        host_data: hd,
    };

    launcher.finish(finish).unwrap();
}
