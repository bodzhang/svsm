// SPDX-License-Identifier: MIT
//
// Copyright (c) 2024-2025
//
// MigTD User Application for Coconut-SVSM
// This runs in VMPL3 on SEV-SNP and provides migration trust domain functionality

#![no_std]
#![no_main]

use userlib::*;

mod state;
mod protocol;
mod attestation;

use state::MigTdState;

declare_main!(main);

/// Main entry point for MigTD
fn main() -> u32 {
    println!("=== MigTD VMPL3 Application Starting ===");
    println!("Running on SEV-SNP via Coconut-SVSM");
    
    // Initialize MigTD state machine
    let mut state = MigTdState::new();
    
    // Main event loop
    match run_migtd(&mut state) {
        Ok(()) => {
            println!("MigTD completed successfully");
            0
        }
        Err(e) => {
            println!("MigTD failed with error: {:?}", e);
            1
        }
    }
}

/// Run the MigTD state machine
fn run_migtd(state: &mut MigTdState) -> Result<(), MigTdError> {
    println!("Initializing MigTD state machine...");
    
    // Initialize
    state.initialize()?;
    println!("State: {:?}", state.current_state());
    
    // Wait for migration request
    // In SVSM, this would come via a syscall or shared memory protocol
    println!("Waiting for migration events...");
    
    // For now, just demonstrate the basic flow
    // In full implementation, this would be an event loop
    state.handle_event(protocol::MigrationEvent::MigrationRequest)?;
    println!("State: {:?}", state.current_state());
    
    // Perform attestation
    match attestation::perform_attestation() {
        Ok(report_len) => {
            println!("Attestation successful, report size: {}", report_len);
            state.handle_event(protocol::MigrationEvent::AttestationComplete)?;
        }
        Err(e) => {
            println!("Attestation failed: {:?}", e);
            return Err(MigTdError::AttestationFailed);
        }
    }
    
    println!("State: {:?}", state.current_state());
    
    // Complete migration
    state.handle_event(protocol::MigrationEvent::MigrationComplete)?;
    println!("Final state: {:?}", state.current_state());
    
    Ok(())
}

/// MigTD error types
#[derive(Debug, Clone, Copy)]
pub enum MigTdError {
    /// State machine error
    InvalidState,
    /// Protocol error
    Protocol,
    /// Attestation failed
    AttestationFailed,
    /// I/O error
    Io,
    /// Not supported
    NotSupported,
}

impl core::fmt::Display for MigTdError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidState => write!(f, "Invalid state"),
            Self::Protocol => write!(f, "Protocol error"),
            Self::AttestationFailed => write!(f, "Attestation failed"),
            Self::Io => write!(f, "I/O error"),
            Self::NotSupported => write!(f, "Not supported"),
        }
    }
}
