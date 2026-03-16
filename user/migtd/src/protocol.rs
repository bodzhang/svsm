// SPDX-License-Identifier: MIT
//
// MigTD Protocol Definitions
// 
// Migration protocol messages and events

use crate::MigTdError;

/// Migration events that drive state transitions
#[derive(Debug, Clone, Copy)]
pub enum MigrationEvent {
    /// Migration request received from VMM
    MigrationRequest,
    /// Start attestation process
    AttestationStart,
    /// Attestation completed successfully  
    AttestationComplete,
    /// Attestation failed
    AttestationFailed,
    /// Start actual migration
    MigrationStart,
    /// Migration completed
    MigrationComplete,
    /// Abort migration
    Abort,
}

/// Message types for MigTD protocol
#[derive(Debug)]
#[repr(u8)]
pub enum MessageType {
    /// Query message
    Query = 1,
    /// Query response
    QueryResponse = 2,
    /// Migration request
    MigrationRequest = 3,
    /// Migration request response
    MigrationRequestResponse = 4,
    /// Attestation report
    AttestationReport = 5,
    /// Migration data
    MigrationData = 6,
    /// Shutdown notification
    Shutdown = 7,
}

/// Message header for MigTD protocol
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct MessageHeader {
    /// Protocol version
    pub version: u8,
    /// Message type
    pub msg_type: u8,
    /// Reserved
    pub reserved: [u8; 2],
    /// Payload length
    pub payload_len: u32,
}

impl MessageHeader {
    pub const SIZE: usize = core::mem::size_of::<MessageHeader>();
    
    pub fn new(msg_type: MessageType, payload_len: usize) -> Self {
        Self {
            version: 1,
            msg_type: msg_type as u8,
            reserved: [0; 2],
            payload_len: payload_len as u32,
        }
    }
}

/// Query capabilities
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MigTdInfo {
    /// MigTD version
    pub version: u32,
    /// Capabilities bitmap
    pub capabilities: u64,
    /// Reserved
    pub reserved: [u8; 48],
}

impl Default for MigTdInfo {
    fn default() -> Self {
        Self {
            version: 0,
            capabilities: 0,
            reserved: [0u8; 48],
        }
    }
}

/// Query capabilities response
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct QueryResponse {
    /// Local MigTD info
    pub local_info: MigTdInfo,
    /// Remote MigTD info (if available)
    pub remote_info: Option<MigTdInfo>,
}

/// Migration capabilities
pub mod capabilities {
    /// Supports basic migration
    pub const BASIC_MIGRATION: u64 = 1 << 0;
    /// Supports live migration
    pub const LIVE_MIGRATION: u64 = 1 << 1;
    /// Supports encrypted migration
    pub const ENCRYPTED_MIGRATION: u64 = 1 << 2;
    /// Supports SEV-SNP attestation
    pub const SNP_ATTESTATION: u64 = 1 << 3;
    /// Supports TDX attestation (for future TDX port)
    pub const TDX_ATTESTATION: u64 = 1 << 4;
}

/// Protocol operations (for future implementation)
#[allow(dead_code)]
pub trait MigrationProtocol {
    /// Send a query message
    fn send_query(&mut self) -> Result<QueryResponse, MigTdError>;
    
    /// Send migration request
    fn send_migration_request(&mut self) -> Result<(), MigTdError>;
    
    /// Receive migration request
    fn recv_migration_request(&mut self) -> Result<bool, MigTdError>;
    
    /// Send attestation report
    fn send_attestation_report(&mut self, report: &[u8]) -> Result<(), MigTdError>;
    
    /// Receive attestation report into provided buffer
    /// Returns the number of bytes written
    fn recv_attestation_report(&mut self, buffer: &mut [u8]) -> Result<usize, MigTdError>;
}
