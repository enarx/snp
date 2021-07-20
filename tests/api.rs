// SPDX-License-Identifier: Apache-2.0

#[test]
fn generic_test() {
    assert_eq!(2 + 2, 4);
}

#[cfg_attr(not(has_sev), ignore)]
#[test]
fn sev_required_test() {
    assert_eq!(2 + 2, 4);
}
