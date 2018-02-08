// This file is part of intel-tsx-rtm. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-tsx-rtm/master/COPYRIGHT. No part of intel-tsx-rtm, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of intel-tsx-rtm. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-tsx-rtm/master/COPYRIGHT.


#![allow(non_upper_case_globals)]
#![deny(missing_docs)]
#![feature(asm)]


//! # intel-tsx-rtm
//! This crates provides a simple set of wrappers around Intel's TSX RTM instructions and associated intrinsics.
//! It uses code to provide runtime detection of CPUs without TSX and fallback to non-hardware paths.
//! It is best to start with `HardwareMemoryTransactionManager`.
//!
//! It will only compile with nightly (as of February 8th, 2018) because it uses the `asm!` macro.
//!


#[macro_use] extern crate bitflags;
#[cfg(target_arch = "x86_64")] extern crate raw_cpuid;


#[cfg(target_arch = "x86_64")] use self::intrinsics::*;
#[cfg(target_arch = "x86_64")] use ::raw_cpuid::CpuId;


/// Very low-level intrinsics closely matching those defined in gcc, clang and Andi Kleen's tsx-tools.
#[cfg(target_arch = "x86_64")] pub mod intrinsics;


include!("HardwareMemoryTransactionManager.rs");
include!("HardwareMemoryTransactionResult.rs");
