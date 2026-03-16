// SPDX-License-Identifier: MIT
//
// MigAgent State Machine
// 
// State transitions for migration agent operations

use crate::{MigAgentError, protocol::MigrationEvent};

/// MigAgent state machine states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Initial state, waiting for initialization
    Init,
    /// Initialized and ready
    Ready,  
    /// Waiting for peer attestation
    WaitingForAttestation,
    /// Attestation in progress
    Attesting,
    /// Attestation completed, migration ready
    MigrationReady,
    /// Migration in progress
    Migrating,
    /// Migration complete
    Complete,
    /// Error state
    Error,
}

impl Default for State {
    fn default() -> Self {
        Self::Init
    }
}

/// MigAgent state machine
#[derive(Debug)]
pub struct MigAgentState {
    state: State,
    /// Role: source or destination
    role: MigrationRole,
}

/// Migration role identifiers
#[derive(Debug, Clone, Copy)]
pub enum MigrationRole {
    Unknown,
    Source,
    Destination,
}

impl MigAgentState {
    /// Create a new MigAgent state machine
    pub fn new() -> Self {
        Self {
            state: State::Init,
            role: MigrationRole::Unknown,
        }
    }
    
    /// Get current state
    pub fn current_state(&self) -> State {
        self.state
    }
    
    /// Initialize the state machine
    pub fn initialize(&mut self) -> Result<(), MigAgentError> {
        if self.state != State::Init {
            return Err(MigAgentError::InvalidState);
        }
        
        // Perform initialization
        // - Read configuration
        // - Setup communication channels
        // - Initialize cryptographic state
        
        self.state = State::Ready;
        Ok(())
    }
    
    /// Handle a migration event
    pub fn handle_event(&mut self, event: MigrationEvent) -> Result<(), MigAgentError> {
        let next_state = self.transition(event)?;
        self.state = next_state;
        Ok(())
    }
    
    /// State transition logic
    fn transition(&self, event: MigrationEvent) -> Result<State, MigAgentError> {
        match (self.state, event) {
            // From Ready state
            (State::Ready, MigrationEvent::MigrationRequest) => {
                Ok(State::WaitingForAttestation)
            }
            
            // From WaitingForAttestation
            (State::WaitingForAttestation, MigrationEvent::AttestationStart) => {
                Ok(State::Attesting)
            }
            (State::WaitingForAttestation, MigrationEvent::AttestationComplete) => {
                Ok(State::MigrationReady)
            }
            
            // From Attesting
            (State::Attesting, MigrationEvent::AttestationComplete) => {
                Ok(State::MigrationReady)
            }
            (State::Attesting, MigrationEvent::AttestationFailed) => {
                Ok(State::Error)
            }
            
            // From MigrationReady
            (State::MigrationReady, MigrationEvent::MigrationStart) => {
                Ok(State::Migrating)
            }
            (State::MigrationReady, MigrationEvent::MigrationComplete) => {
                Ok(State::Complete)
            }
            
            // From Migrating
            (State::Migrating, MigrationEvent::MigrationComplete) => {
                Ok(State::Complete)
            }
            
            // Any state can abort
            (_, MigrationEvent::Abort) => {
                Ok(State::Error)
            }
            
            // Invalid transition
            _ => Err(MigAgentError::InvalidState),
        }
    }
    
    /// Set the migration role
    pub fn set_role(&mut self, role: MigrationRole) {
        self.role = role;
    }
    
    /// Get the migration role
    pub fn role(&self) -> MigrationRole {
        self.role
    }
    
    /// Check if in error state
    pub fn is_error(&self) -> bool {
        self.state == State::Error
    }
    
    /// Check if migration is complete
    pub fn is_complete(&self) -> bool {
        self.state == State::Complete
    }
}

impl Default for MigAgentState {
    fn default() -> Self {
        Self::new()
    }
}
