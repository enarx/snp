[![Workflow Status](https://github.com/enarx/snp/workflows/test/badge.svg)](https://github.com/enarx/snp/actions?query=workflow%3A%22test%22)
[![Average time to resolve an issue](https://isitmaintained.com/badge/resolution/enarx/snp.svg)](https://isitmaintained.com/project/enarx/snp "Average time to resolve an issue")
[![Percentage of issues still open](https://isitmaintained.com/badge/open/enarx/snp.svg)](https://isitmaintained.com/project/enarx/snp "Percentage of issues still open")
![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)

# snp

The `snp` crate provides an implementation of the AMD Secure Encrypted
Virtualization - Secure Nested Paging (SEV-SNP) APIs.

The Linux kernel exposes two technically distinct AMD SEV-SNP APIs:

1. An API for managing the SEV-SNP platform itself
2. An API for managing SNP-enabled KVM virtual machines

This crate implements both of those APIs and offers them to client
code through a flexible and type-safe high level interface.

### Remarks

Note that the Linux kernel provides access to these APIs through a set
of `ioctl`s that are meant to be called on device nodes (`/dev/kvm` and
`/dev/sev`, to be specific). As a result, these `ioctl`s form the substrate
of the `snp` crate. Binaries that result from consumers of this crate are
expected to run as a process with the necessary privileges to interact
with the device nodes.

License: Apache-2.0
