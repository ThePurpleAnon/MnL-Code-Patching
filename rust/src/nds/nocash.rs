//! Module that allows special interactions with certain emulators.

// Taken from IronDS, which is licensed under the Zlib license:
// Copyright (C) 2022 Quinn Painter
//
// This software is provided 'as-is', without any express or implied
// warranty.  In no event will the authors be held liable for any damages
// arising from the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it
// freely, subject to the following restrictions:
//
// 1. The origin of this software must not be misrepresented; you must not
//    claim that you wrote the original software. If you use this software
//    in a product, an acknowledgment in the product documentation would be
//    appreciated but is not required.
// 2. Altered source versions must be plainly marked as such, and must not be
//    misrepresented as being the original software.
// 3. This notice may not be removed or altered from any source distribution.

// See the "Debug Messages" section of the NO$GBA help for more detail.
// (the website is outdated, view it in the actual app)

use core::{arch::asm, ffi::CStr};

use alloc::ffi::CString;

use super::mmio;

/// Prints a message to the emulator's debug window.
///
/// Works in NO$GBA, melonDS and DeSmuME.
#[instruction_set(arm::a32)]
#[inline(never)]
pub fn print_using_opcode(s: &str) {
    for chunk in s.as_bytes().chunks(100) {
        unsafe {
            asm!(
                "ldr r0, =2f",
                "add r3, r0, r2", //
                "mov r4, #0",     // insert 0 terminator at end of string
                "strb r4, [r3]",  //
                "ldr r3, =__aeabi_memcpy", // copy the input string into the .space below
                "bl 3f", // jumps to "bx r3"
                "mov r12, r12",
                "b 4f", // f = local label is forwards (llvm bug prevents using labels 0 and 1?)
                ".hword 0x6464", // magic number
                ".hword 0", // flags?
                "2:",
                ".space 101", // extra byte for 0 terminator
                ".align",
                "3:",
                "bx r3", // separate bx since arm7 doesn't have blx
                "4:",
                in("r1") chunk as *const [u8] as *const u8, // inline version of ".as_ptr()"
                in("r2") chunk.len(),
                out("r4") _,
                clobber_abi("C"),
            );
        }
    }
}
#[inline(always)]
pub fn print_raw(s: &CStr) {
    mmio::NOCASH_STROUT_RAW.write(s.as_ptr().addr().try_into().unwrap());
}
pub fn print_raw_str(s: &str) {
    print_raw(CString::new(s).unwrap().as_c_str());
}
#[inline(always)]
pub fn print_param(s: &CStr) {
    mmio::NOCASH_STROUT_PARAM.write(s.as_ptr().addr().try_into().unwrap());
}
pub fn print_param_str(s: &str) {
    print_param(CString::new(s).unwrap().as_c_str());
}
#[inline(always)]
pub fn print_paramln(s: &CStr) {
    mmio::NOCASH_STROUT_PARAM_LF.write(s.as_ptr().addr().try_into().unwrap());
}
pub fn print_paramln_str(s: &str) {
    print_paramln(CString::new(s).unwrap().as_c_str());
}
/// Prints a message to the emulator's debug window.
///
/// Works in NO$GBA and melonDS.
/// You should probably use "print_using_opcode" instead, as it is more compatible. This is just included for posterity.
pub fn print_using_charout(s: &str) {
    for b in s.bytes() {
        mmio::NOCASH_CHAROUT.write(b as u32);
    }
}

/// A code breakpoint that will pause the emulator when it is executed.
///
/// Works in NO$GBA.
// Works in both ARM and THUMB mode.
pub macro breakpoint() {
    unsafe {
        asm!("mov r11, r11", options(nomem, preserves_flags, nostack));
    }
}
