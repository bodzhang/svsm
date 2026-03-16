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
mod vmm_comm;

use state::MigAgentState;
use vmm_comm::{MigRequestType, MigStatus};

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

/// Run the MigAgent event loop
fn run_migagent(state: &mut MigAgentState) -> Result<(), MigAgentError> {
    println!("Initializing MigAgent state machine...");
    
    // Initialize
    state.initialize()?;
    println!("State: {:?}", state.current_state());
    
    // Event loop - wait for VMM requests
    println!("Entering event loop, waiting for VMM requests...");
    
    loop {
        // Wait for migration request from VMM
        // Note: This blocks until the VMM sends a request via GHCB
        match vmm_comm::wait_for_request() {
            Ok(request) => {
                println!("Received request: id={}, type={:?}", 
                         request.request_id, request.request_type);
                
                // Handle the request based on type
                let result = handle_request(state, &request);
                
                // Report status back to VMM
                let status = match result {
                    Ok(()) => MigStatus::Success,
                    Err(MigAgentError::AttestationFailed) => MigStatus::AttestationFailed,
                    Err(MigAgentError::Protocol) => MigStatus::ProtocolError,
                    Err(_) => MigStatus::InternalError,
                };
                
                if let Err(e) = vmm_comm::report_status(request.request_id, status) {
                    println!("Failed to report status: {:?}", e);
                }
                
                // Exit loop on abort or completion
                if request.request_type == MigRequestType::Abort {
                    println!("Migration aborted by VMM");
                    return Err(MigAgentError::Protocol);
                }
                
                if state.is_complete() {
                    println!("Migration completed successfully");
                    return Ok(());
                }
            }
            Err(e) => {
                // For now, if wait fails, fall back to demo mode
                println!("VMM communication not available ({:?}), running demo mode", e);
                return run_demo_mode(state);
            }
        }
    }
}

/// Handle a migration request from the VMM
fn handle_request(state: &mut MigAgentState, request: &vmm_comm::MigrationRequest) -> Result<(), MigAgentError> {
    match request.request_type {
        MigRequestType::Query => {
            // TODO: Send capabilities
            println!("Query request - sending capabilities");
            Ok(())
        }
        MigRequestType::StartSource | MigRequestType::StartDestination => {
            // Start migration process
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
            
            // Complete migration
            state.handle_event(protocol::MigrationEvent::MigrationComplete)?;
            println!("Final state: {:?}", state.current_state());
            
            Ok(())
        }
        MigRequestType::Abort => {
            state.handle_event(protocol::MigrationEvent::Abort)?;
            Ok(())
        }
    }
}

/// Demo mode - run without VMM communication (for testing)
fn run_demo_mode(state: &mut MigAgentState) -> Result<(), MigAgentError> {
    println!("Running in demo mode...");
    
    // Simulate migration flow
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
