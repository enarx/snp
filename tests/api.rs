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

#[cfg_attr(not(has_sev), ignore)]
#[test]
fn ext_config() {
    let mut fw = Firmware::open().unwrap();

    fw.set_ext_config(0, 0, 0).unwrap();

    let config = fw.get_ext_config().unwrap();
    println!(
        "Get platform config results:\n
        config_address: {}\n
        certs_address: {}\n
        certs_len: {}",
        config.config_address, config.certs_address, config.certs_len
    );
}
