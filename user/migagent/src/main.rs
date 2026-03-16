// SPDX-License-Identifier: MIT
//
// Copyright (c) 2024-2025
//
// MigAgent User Application for Coconut-SVSM
// This runs in VMPL3 on SEV-SNP and provides migration agent functionality

#![no_std]
#![no_main]

use userlib::*;

mod state;
mod protocol;
mod attestation;

use state::MigAgentState;

declare_main!(main);

/// Main entry point for MigAgent
fn main() -> u32 {
    println!("=== MigAgent VMPL3 Application Starting ===");
    println!("Running on SEV-SNP via Coconut-SVSM");
    
    // Initialize MigAgent state machine
    let mut state = MigAgentState::new();
    
    // Main event loop
    match run_migagent(&mut state) {
        Ok(()) => {
            println!("MigAgent completed successfully");
            0
        }
        Err(e) => {
            println!("MigAgent failed with error: {:?}", e);
            1
        }
    }
}

/// Run the MigAgent state machine
fn run_migagent(state: &mut MigAgentState) -> Result<(), MigAgentError> {
    println!("Initializing MigAgent state machine...");
    
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
            return Err(MigAgentError::AttestationFailed);
        }
    }
    
    println!("State: {:?}", state.current_state());
    
    // Complete migration
    state.handle_event(protocol::MigrationEvent::MigrationComplete)?;
    println!("Final state: {:?}", state.current_state());
    
    Ok(())
}

/// MigAgent error types
#[derive(Debug, Clone, Copy)]
pub enum MigAgentError {
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

impl core::fmt::Display for MigAgentError {
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
