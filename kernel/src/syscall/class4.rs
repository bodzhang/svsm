// SPDX-License-Identifier: MIT
//
// Copyright (c) 2025 Microsoft Corporation
//
// MigAgent kernel syscall handlers

use crate::address::VirtAddr;
use crate::cpu::percpu::current_ghcb;
use crate::mm::guestmem::{copy_from_user, copy_to_user};
use crate::mm::PageBox;
use crate::types::PAGE_SIZE;
use syscall::SysCallError;

/// Kernel handler for SYS_MIGAGENT_WAIT
///
/// Waits for a migration request from the VMM.
pub fn sys_migagent_wait(buffer: usize, len: usize) -> Result<u64, SysCallError> {
    if len == 0 || len > PAGE_SIZE {
        return Err(SysCallError::EINVAL);
    }

    // Allocate a shared page for communication with VMM
    let shared_page = PageBox::<[u8; PAGE_SIZE]>::try_new_zeroed()
        .map_err(|_| SysCallError::ENOMEM)?;
    
    let vaddr = shared_page.vaddr();
    
    // Call GHCB to wait for migration request
    let ghcb = current_ghcb();
    let received = ghcb
        .migagent_wait(vaddr, len)
        .map_err(|_| SysCallError::EFAULT)?;

    // Copy received data to user buffer
    if received > 0 {
        let copy_len = core::cmp::min(received, len);
        // SAFETY: shared_page is valid and we're copying within bounds
        unsafe {
            let src_slice = core::slice::from_raw_parts(vaddr.as_ptr::<u8>(), copy_len);
            copy_to_user(src_slice, VirtAddr::from(buffer))
                .map_err(|_| SysCallError::EFAULT)?;
        }
    }

    Ok(received as u64)
}

/// Kernel handler for SYS_MIGAGENT_SEND
///
/// Sends data to the VMM as part of a migration operation.
pub fn sys_migagent_send(request_id: usize, buffer: usize, len: usize) -> Result<u64, SysCallError> {
    if len == 0 || len > PAGE_SIZE {
        return Err(SysCallError::EINVAL);
    }

    // Allocate a shared page for communication with VMM
    let shared_page = PageBox::<[u8; PAGE_SIZE]>::try_new_zeroed()
        .map_err(|_| SysCallError::ENOMEM)?;
    
    let vaddr = shared_page.vaddr();

    // Copy user data to shared page
    // SAFETY: shared_page is valid and we're copying within bounds
    unsafe {
        let dst_slice = core::slice::from_raw_parts_mut(vaddr.as_mut_ptr::<u8>(), len);
        copy_from_user(VirtAddr::from(buffer), dst_slice)
            .map_err(|_| SysCallError::EFAULT)?;
    }

    // Call GHCB to send data
    let ghcb = current_ghcb();
    ghcb.migagent_send(request_id as u64, vaddr, len)
        .map_err(|_| SysCallError::EFAULT)?;

    Ok(0)
}

/// Kernel handler for SYS_MIGAGENT_RECV
///
/// Receives data from the VMM as part of a migration operation.
pub fn sys_migagent_recv(request_id: usize, buffer: usize, len: usize) -> Result<u64, SysCallError> {
    if len == 0 || len > PAGE_SIZE {
        return Err(SysCallError::EINVAL);
    }

    // Allocate a shared page for communication with VMM
    let shared_page = PageBox::<[u8; PAGE_SIZE]>::try_new_zeroed()
        .map_err(|_| SysCallError::ENOMEM)?;
    
    let vaddr = shared_page.vaddr();

    // Call GHCB to receive data
    let ghcb = current_ghcb();
    let received = ghcb
        .migagent_receive(request_id as u64, vaddr, len)
        .map_err(|_| SysCallError::EFAULT)?;

    // Copy received data to user buffer
    if received > 0 {
        let copy_len = core::cmp::min(received, len);
        // SAFETY: shared_page is valid and we're copying within bounds
        unsafe {
            let src_slice = core::slice::from_raw_parts(vaddr.as_ptr::<u8>(), copy_len);
            copy_to_user(src_slice, VirtAddr::from(buffer))
                .map_err(|_| SysCallError::EFAULT)?;
        }
    }

    Ok(received as u64)
}

/// Kernel handler for SYS_MIGAGENT_STATUS
///
/// Reports migration status to the VMM.
pub fn sys_migagent_status(request_id: usize, status: usize) -> Result<u64, SysCallError> {
    let ghcb = current_ghcb();
    ghcb.migagent_report_status(request_id as u64, status as u64)
        .map_err(|_| SysCallError::EFAULT)?;

    Ok(0)
}
