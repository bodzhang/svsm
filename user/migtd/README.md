# MigTD User Application for Coconut-SVSM

This directory contains a user-space MigTD (Migration Trust Domain) application
that runs in VMPL3 on SEV-SNP via Coconut-SVSM.

## Overview

MigTD provides trusted migration functionality for confidential VMs. This
implementation runs as a privileged user-space application within the SVSM
environment, leveraging SVSM's isolation and attestation capabilities.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Host/Hypervisor                         │
├─────────────────────────────────────────────────────────────┤
│                    SEV-SNP Hardware                         │
├─────────────────────────────────────────────────────────────┤
│ VMPL0: Coconut-SVSM Kernel                                  │
│   - Memory management                                        │
│   - Syscall handling                                         │
│   - Attestation services (GHCB → PSP)                        │
│   - vTPM services                                            │
├─────────────────────────────────────────────────────────────┤
│ VMPL3: MigTD User Application (this code)                   │
│   - State machine for migration workflow                     │
│   - Protocol handling                                        │
│   - Attestation verification                                 │
└─────────────────────────────────────────────────────────────┘
```

## Modules

### `main.rs`
Entry point and main event loop. Uses the `declare_main!` macro from userlib
to set up the entry point and panic handler.

### `state.rs`
State machine implementation for migration workflow:
- `Init` → `Ready` → `WaitingForAttestation` → `Attesting` → `MigrationReady` → `Complete`

### `protocol.rs`
Protocol definitions for MigTD communication:
- Message types and headers
- Capability negotiation
- Migration event types

### `attestation.rs`
SEV-SNP attestation support:
- Request attestation reports via SVSM kernel
- Verify remote attestation reports
- Report data binding for freshness/key binding

## Building

Build with the standard SVSM build process:

```bash
# Build migtd along with SVSM
make FEATURES=vtpm

# Or build just migtd for development
cargo build --package migtd --target=x86_64-unknown-none
```

## Testing

The migtd binary will be included in the SVSM filesystem image at `/migtd` when
built with `configs/migtd-target.json`:

```bash
cargo xbuild ./configs/migtd-target.json
```

## TODO

- [ ] Add syscall for requesting SNP attestation reports from userspace
- [ ] Implement actual attestation verification (signature chain validation)
- [ ] Add communication channel with VMM (virtio-vsock or shared memory)
- [ ] Implement migration data transfer
- [ ] Add vTPM PCR extend for runtime measurements (RTMR equivalent)
- [ ] Policy verification for migration authorization

## Relationship to TDX MigTD

This implementation is inspired by Intel's TDX MigTD but adapted for SEV-SNP:

| TDX MigTD | SEV-SNP MigTD |
|-----------|---------------|
| Runs as separate TD | Runs in VMPL3 |
| TDCALL for measurements | GHCB for attestation |
| RTMR for runtime measurements | vTPM PCRs |
| TD Quote | SNP Attestation Report |

## License

MIT (following Coconut-SVSM licensing)
