// SPDX-License-Identifier: MIT
//
// Copyright (c) 2025 Microsoft Corporation
//
// MigAgent syscall wrappers for migration agent communication

use super::call::{SysCallError, syscall2, syscall3};
use super::{SYS_MIGAGENT_RECV, SYS_MIGAGENT_SEND, SYS_MIGAGENT_STATUS, SYS_MIGAGENT_WAIT};

/// Wait for a migration request from the VMM.
///
/// This function blocks until the VMM signals that a migration operation
/// should begin.
///
/// # Arguments
/// * `buffer` - Buffer to receive migration request data
///
/// # Returns
/// * `Ok(usize)` - Number of bytes of request data received
/// * `Err(SysCallError)` - If the operation failed
pub fn migagent_wait(buffer: &mut [u8]) -> Result<usize, SysCallError> {
    // SAFETY: SYS_MIGAGENT_WAIT is a supported syscall number.
    // The kernel will validate the buffer pointer and length.
    unsafe {
        syscall2(
            SYS_MIGAGENT_WAIT,
            buffer.as_mut_ptr() as u64,
            buffer.len() as u64,
        )
        .map(|ret| ret as usize)
    }
}

/// Send data to the VMM as part of a migration operation.
///
/// # Arguments
/// * `request_id` - Migration request identifier from VMM
/// * `data` - Data to send
///
/// # Returns
/// * `Ok(())` - Data sent successfully
/// * `Err(SysCallError)` - If the operation failed
pub fn migagent_send(request_id: u64, data: &[u8]) -> Result<(), SysCallError> {
    // SAFETY: SYS_MIGAGENT_SEND is a supported syscall number.
    // The kernel will validate the buffer pointer and length.
    unsafe {
        syscall3(
            SYS_MIGAGENT_SEND,
            request_id,
            data.as_ptr() as u64,
            data.len() as u64,
        )
        .map(|_| ())
    }
}

/// Receive data from the VMM as part of a migration operation.
///
/// # Arguments
/// * `request_id` - Migration request identifier from VMM
/// * `buffer` - Buffer to receive data
///
/// # Returns
/// * `Ok(usize)` - Number of bytes received
/// * `Err(SysCallError)` - If the operation failed
pub fn migagent_receive(request_id: u64, buffer: &mut [u8]) -> Result<usize, SysCallError> {
    // SAFETY: SYS_MIGAGENT_RECV is a supported syscall number.
    // The kernel will validate the buffer pointer and length.
    unsafe {
        syscall3(
            SYS_MIGAGENT_RECV,
            request_id,
            buffer.as_mut_ptr() as u64,
            buffer.len() as u64,
        )
        .map(|ret| ret as usize)
    }
}

/// Report migration status to the VMM.
///
/// # Arguments
/// * `request_id` - Migration request identifier from VMM
/// * `status` - Status code to report (0 = success, non-zero = error)
///
/// # Returns
/// * `Ok(())` - Status reported successfully
/// * `Err(SysCallError)` - If the operation failed
pub fn migagent_report_status(request_id: u64, status: u64) -> Result<(), SysCallError> {
    // SAFETY: SYS_MIGAGENT_STATUS is a supported syscall number.
    unsafe { syscall2(SYS_MIGAGENT_STATUS, request_id, status).map(|_| ()) }
}
