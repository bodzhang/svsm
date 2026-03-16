// SPDX-License-Identifier: MIT
//
// MigAgent VMM Communication Module
//
// Provides communication with the VMM via GHCB-based syscalls

use crate::MigAgentError;
use userlib::{migagent_wait, migagent_send, migagent_receive, migagent_report_status};

/// Maximum size for migration data buffers
pub const MAX_DATA_SIZE: usize = 4096;

/// Migration request information from VMM
#[derive(Debug, Clone, Copy)]
pub struct MigrationRequest {
    /// Request ID assigned by VMM
    pub request_id: u64,
    /// Type of migration request
    pub request_type: MigRequestType,
}

/// Types of migration requests
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MigRequestType {
    /// Query capabilities
    Query = 0,
    /// Start migration as source
    StartSource = 1,
    /// Start migration as destination  
    StartDestination = 2,
    /// Abort migration
    Abort = 3,
}

impl TryFrom<u8> for MigRequestType {
    type Error = MigAgentError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MigRequestType::Query),
            1 => Ok(MigRequestType::StartSource),
            2 => Ok(MigRequestType::StartDestination),
            3 => Ok(MigRequestType::Abort),
            _ => Err(MigAgentError::Protocol),
        }
    }
}

/// Migration status codes
#[repr(u64)]
pub enum MigStatus {
    Success = 0,
    InProgress = 1,
    AttestationFailed = 2,
    ProtocolError = 3,
    InternalError = 4,
    Aborted = 5,
}

/// Wait for a migration request from the VMM.
///
/// This function blocks until the VMM signals that a migration operation
/// should begin.
///
/// # Returns
/// * `Ok(MigrationRequest)` - The migration request from VMM
/// * `Err(MigAgentError)` - If the operation failed
pub fn wait_for_request() -> Result<MigrationRequest, MigAgentError> {
    let mut buffer = [0u8; MAX_DATA_SIZE];
    
    let received = migagent_wait(&mut buffer)
        .map_err(|_| MigAgentError::Io)?;
    
    if received < 9 {
        return Err(MigAgentError::Protocol);
    }
    
    // Parse request: [request_id: u64][request_type: u8]
    let request_id = u64::from_le_bytes([
        buffer[0], buffer[1], buffer[2], buffer[3],
        buffer[4], buffer[5], buffer[6], buffer[7],
    ]);
    let request_type = MigRequestType::try_from(buffer[8])?;
    
    Ok(MigrationRequest {
        request_id,
        request_type,
    })
}

/// Send data to the VMM.
///
/// # Arguments
/// * `request_id` - Migration request identifier
/// * `data` - Data to send
///
/// # Returns
/// * `Ok(())` - Data sent successfully
/// * `Err(MigAgentError)` - If the operation failed
pub fn send_data(request_id: u64, data: &[u8]) -> Result<(), MigAgentError> {
    if data.len() > MAX_DATA_SIZE {
        return Err(MigAgentError::Protocol);
    }
    
    migagent_send(request_id, data)
        .map_err(|_| MigAgentError::Io)
}

/// Receive data from the VMM.
///
/// # Arguments
/// * `request_id` - Migration request identifier
/// * `buffer` - Buffer to receive data
///
/// # Returns
/// * `Ok(usize)` - Number of bytes received
/// * `Err(MigAgentError)` - If the operation failed
pub fn receive_data(request_id: u64, buffer: &mut [u8]) -> Result<usize, MigAgentError> {
    migagent_receive(request_id, buffer)
        .map_err(|_| MigAgentError::Io)
}

/// Report migration status to the VMM.
///
/// # Arguments
/// * `request_id` - Migration request identifier
/// * `status` - Status to report
///
/// # Returns
/// * `Ok(())` - Status reported successfully
/// * `Err(MigAgentError)` - If the operation failed
pub fn report_status(request_id: u64, status: MigStatus) -> Result<(), MigAgentError> {
    migagent_report_status(request_id, status as u64)
        .map_err(|_| MigAgentError::Io)
}
