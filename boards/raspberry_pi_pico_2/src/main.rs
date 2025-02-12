// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2022.

//! Tock kernel for the Raspberry Pi Pico.
//!
//! It is based on RP2040SoC SoC (Cortex M0+).

#![no_std]
// Disable this attribute when documenting, as a workaround for
// https://github.com/rust-lang/rust/issues/62184.
#![cfg_attr(not(doc), no_main)]
#![deny(missing_docs)]

#[allow(unused)]
use rp2350::BASE_VECTORS;

mod io;

mod flash_bootloader;

/// Allocate memory for the stack
#[no_mangle]
#[link_section = ".stack_buffer"]
pub static mut STACK_MEMORY: [u8; 0x1500] = [0; 0x1500];

// Manually setting the boot header section that contains the FCB header
#[used]
#[link_section = ".flash_bootloader"]
static FLASH_BOOTLOADER: [u8; 256] = flash_bootloader::FLASH_BOOTLOADER;

#[used]
#[link_section = ".metadata_block"]
static METADATA_BLOCK: [u8; 28] = flash_bootloader::METADATA_BLOCK;

#[allow(dead_code)]
extern "C" {
    /// Entry point used for debugger
    ///
    /// When loaded using gdb, the Raspberry Pi Pico is not reset
    /// by default. Without this function, gdb sets the PC to the
    /// beginning of the flash. This is not correct, as the RP2040
    /// has a more complex boot process.
    ///
    /// This function is set to be the entry point for gdb and is used
    /// to send the RP2040 back in the bootloader so that all the boot
    /// sequence is performed.
    fn jump_to_bootloader();
}

#[cfg(any(doc, all(target_arch = "arm", target_os = "none")))]
core::arch::global_asm!(
    "
    .section .jump_to_bootloader, \"ax\"
    .global jump_to_bootloader
    .thumb_func
  jump_to_bootloader:
    movs r0, #0
    ldr r1, =(0xe0000000 + 0x0000ed08)
    str r0, [r1]
    ldmia r0!, {{r1, r2}}
    msr msp, r1
    bx r2
    "
);

/// Main function called after RAM initialized.
#[no_mangle]
pub unsafe fn main() {
    loop {
        cortexm33::support::nop();
    }
}
