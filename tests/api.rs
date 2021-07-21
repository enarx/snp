// SPDX-License-Identifier: Apache-2.0

use snp::firmware::Firmware;

#[cfg_attr(not(has_sev), ignore)]
#[test]
fn platform_status() {
    let mut fw = Firmware::open().unwrap();
    let status = fw.platform_status().unwrap();

    println!(
        "Platform status ioctl results:\n
              version (major, minor): {}.{}\n
              build id: {}\n
              guests: {}\n
              tcb version: {}\n
              state: {}\n",
        status.build.version.major,
        status.build.version.minor,
        status.build.build,
        status.guests,
        status.tcb_version,
        status.state
    );
}
