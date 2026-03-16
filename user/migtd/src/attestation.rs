// SPDX-License-Identifier: MIT
//
// MigTD Attestation Module
//
// SEV-SNP attestation for migration trust domain
// Uses SVSM kernel services for attestation report generation

use crate::MigTdError;
// Import print macros and console_print function from userlib
use userlib::{print, println, console_print};

/// SNP attestation report size
pub const SNP_REPORT_SIZE: usize = 0x4A0;  // 1184 bytes

/// Maximum report data size (for nonce/binding)
pub const REPORT_DATA_SIZE: usize = 64;

/// Static buffer for attestation report (to avoid alloc)
static mut ATTESTATION_REPORT_BUFFER: [u8; SNP_REPORT_SIZE] = [0u8; SNP_REPORT_SIZE];

/// Attestation report wrapper
#[derive(Debug)]
pub struct AttestationReport {
    /// Length of valid report data
    len: usize,
}

impl AttestationReport {
    /// Get the raw report bytes
    pub fn as_bytes(&self) -> &[u8] {
        // SAFETY: Single-threaded access, we control the buffer
        unsafe { &ATTESTATION_REPORT_BUFFER[..self.len] }
    }
    
    /// Get report length
    pub fn len(&self) -> usize {
        self.len
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

/// Perform attestation and return report length
/// 
/// In SVSM, this calls the kernel's attestation service via GHCB to get
/// an SNP attestation report. The report proves the VM's identity and
/// measurement state to a remote verifier.
///
/// Returns the length of the attestation report written to the static buffer.
pub fn perform_attestation() -> Result<usize, MigTdError> {
    println!("Requesting SNP attestation report...");
    
    // Create report data (nonce/binding data)
    let report_data = create_report_data();
    
    // Get attestation report via kernel service
    // Currently we just create a placeholder - the actual implementation
    // will require a new syscall to request attestation from the SVSM kernel
    let len = get_snp_report(&report_data)?;
    
    println!("Attestation report obtained: {} bytes", len);
    
    Ok(len)
}

/// Create report data for attestation binding
///
/// This data is included in the SNP attestation report's REPORT_DATA field
/// and can be used to:
/// - Include a nonce for freshness
/// - Bind to a cryptographic key
/// - Include session identifiers
fn create_report_data() -> [u8; REPORT_DATA_SIZE] {
    let mut data = [0u8; REPORT_DATA_SIZE];
    
    // TODO: Generate proper nonce or binding data
    // For now, use a placeholder
    data[0..8].copy_from_slice(b"MIGTD001");
    
    data
}

/// Get SNP attestation report from SVSM kernel
///
/// This function requests an SNP attestation report via the SVSM kernel.
/// The kernel will use GHCB GUEST_REQUEST to get the report from the PSP.
/// Returns the length of valid data in the static buffer.
fn get_snp_report(report_data: &[u8; REPORT_DATA_SIZE]) -> Result<usize, MigTdError> {
    // TODO: Implement syscall to request attestation from kernel
    // For now, return a placeholder report
    
    // The actual implementation would:
    // 1. Pass buffer address to kernel via syscall
    // 2. Kernel calls snp_get_report via GHCB
    // 3. Return report length to userspace
    
    // SAFETY: Single-threaded access, we control the buffer
    unsafe {
        // Clear buffer
        ATTESTATION_REPORT_BUFFER = [0u8; SNP_REPORT_SIZE];
        
        // Copy report data to beginning
        ATTESTATION_REPORT_BUFFER[..REPORT_DATA_SIZE].copy_from_slice(report_data);
        
        // Mark as placeholder
        ATTESTATION_REPORT_BUFFER[64..75].copy_from_slice(b"PLACEHOLDER");
    }
    
    Ok(SNP_REPORT_SIZE)
}

/// Verify a remote attestation report
/// 
/// Verifies that:
/// - Report signature is valid (signed by AMD root key)
/// - Platform TCB meets requirements
/// - VM measurements match expected values
/// - Report data matches expected binding
#[allow(unused_variables)]
pub fn verify_report(report: &[u8], expected_measurement: Option<&[u8; 48]>) -> Result<bool, MigTdError> {
    if report.len() < SNP_REPORT_SIZE {
        return Err(MigTdError::AttestationFailed);
    }
    
    // TODO: Implement actual verification
    // This would:
    // 1. Parse the SNP report structure
    // 2. Verify the signature chain (VCEK -> ASK -> ARK)
    // 3. Check TCB version against requirements
    // 4. Compare measurements if provided
    
    // For now, placeholder
    println!("Report verification: placeholder (not implemented)");
    
    Ok(true)
}

/// Get report from static buffer
/// 
/// Returns a reference to the attestation report in the static buffer.
#[allow(dead_code)]
pub fn get_report_buffer() -> &'static [u8] {
    // SAFETY: Single-threaded access, we control the buffer
    // Using raw pointer to avoid shared reference to mutable static error
    unsafe {
        let ptr = (&raw const ATTESTATION_REPORT_BUFFER) as *const [u8; SNP_REPORT_SIZE];
        &*ptr
    }
}
