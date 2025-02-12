// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2022.

// use core::fmt::Write;
use core::panic::PanicInfo;

/// Default panic handler for the Raspberry Pi Pico board.
///
/// We just use the standard default provided by the debug module in the kernel.
#[cfg(not(test))]
#[no_mangle]
#[panic_handler]
pub unsafe fn panic_fmt(_pi: &PanicInfo) -> ! {
    loop {}
}
