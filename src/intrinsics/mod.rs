// This file is part of intel-tsx-rtm. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-tsx-rtm/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of intel-tsx-rtm. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-tsx-rtm/master/COPYRIGHT.


/// Value returned from `_xbegin()` if beginning a transaction.
#[cfg(target_arch = "x86_64")]
pub const _XBEGIN_STARTED: u32 = !0;

/// If this flag is in the return code from `_xbegin()`, and the return code was not `_XBEGIN_STARTED`, then a transaction explicitly aborted by transaction logic calling `_xabort()`.
/// The bottom 8-bits of the `status` code passed to `_xabort(status)` can be extracted using the function `_XABORT_CODE()`.
#[cfg(target_arch = "x86_64")]
pub const _XABORT_EXPLICIT: u32 = 1 << 0;

/// If this flag is in the return code from `_xbegin()`, and the return code was not `_XBEGIN_STARTED`, then a transaction that was aborted can be retried.
/// It should only be retried for a small number of times.
/// Retries may occur because of, say, page faults when accessing memory or interrupts.
#[cfg(target_arch = "x86_64")]
pub const _XABORT_RETRY: u32 = 1 << 1;

/// If this flag is in the return code from `_xbegin()`, and the return code was not `_XBEGIN_STARTED`, then a transaction was aborted by the CPU because it read or wrote memory that was also read to or written to by another hyperthreaded (logical) CPU.
/// Such reads and writes may fully or partially overlap.
#[cfg(target_arch = "x86_64")]
pub const _XABORT_CONFLICT: u32 = 1 << 2;

/// If this flag is in the return code from `_xbegin()`, and the return code was not `_XBEGIN_STARTED`, then a transaction was aborted by the CPU because the CPU's (small) amount of memory for tracking transactions was exceeded.
/// This is typically because many memory locations were read or written during the transaction, or because too many CPUs were doing transactions at once.
/// It may be worth retrying the transaction if the `_XABORT_RETRY` flag is set, but only once or twice; typically this result means that the transaction logic is 'trying to do too much'.
#[cfg(target_arch = "x86_64")]
pub const _XABORT_CAPACITY: u32 = 1 << 3;

/// If this flag is in the return code from `_xbegin()`, and the return code was not `_XBEGIN_STARTED`, then a transaction was aborted by the CPU because a debug trap was hit.
/// It may be worth retrying the transaction if the `_XABORT_RETRY` flag is set.
/// This flag should not be in the return code in normal, production use.
#[cfg(target_arch = "x86_64")]
pub const _XABORT_DEBUG: u32 = 1 << 4;

/// If this flag is in the return code from `_xbegin()`, and the return code was not `_XBEGIN_STARTED`, then a transaction was aborted by the CPU because a second call to `_xbegin()` was made inside transactional logic, ie a an attempt was made to nest transactions.
/// This is not supported and indicates programmer error.
#[cfg(target_arch = "x86_64")]
pub const _XABORT_NESTED: u32 = 1 << 5;

/// If the flag `_XABORT_EXPLICIT` is in the return code from `_xbegin()`, and the return code was not `_XBEGIN_STARTED`, then this function returns the `status` value of a transaction explicitly aborted by transaction logic calling `_xabort(status)`
#[cfg(target_arch = "x86_64")]
#[allow(non_snake_case)]
#[inline(always)]
pub fn _XABORT_CODE(_xbegin_return_code: u32) -> u8
{
	(_xbegin_return_code >> 24) as u8
}

/// Begin a transaction.
/// Will raise the signal `SIGILL` (an illegal instruction abort) on CPUs without Intel TSX support, so be careful.
/// Can appear to execute twice, a bit like `setjmp` / `longjmp` or `fork()`.
/// If a transaction has started, then the return code is `_XBEGIN_STARTED`.
/// If a transaction is committed successfully, then the return code will be zero, 0.
/// If a transaction aborts or rolls back, execution restarts here with all memory effects undone and a return code that is *not* `_XBEGIN_STARTED`; in which case, the return code is a mix of a bit set of flags and a result code.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn _xbegin() -> u32
{
	let mut r: u32 = _XBEGIN_STARTED;
	asm!
	(
		".byte 0xc7,0xf8 ; .long 0"
		:
			"+a" (r)
		:
		:
			"memory"
		:
			"volatile"
	);
	r
}

/// Ends a transaction by committing it.
/// Will raise the signal `SIGILL` (an illegal instruction abort) on CPUs without Intel TSX support, so be careful.
/// The value returned as the result code of `_xbegin()` will be zero, 0, if the CPU can commit this transaction without any other thread having changed the memory read or written to by it.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn _xend()
{
	asm!
	(
		".byte 0x0f,0x01,0xd5"
		:
		:
		:
			"memory"
		:
			"volatile"
	)
}

/// Aborts a transaction.
/// Will raise the signal `SIGILL` (an illegal instruction abort) on CPUs without Intel TSX support, so be careful.
/// Returns the bottom 8 bits of `status` as a result code embedded in the return code from `_xbegin()`.
/// It is unclear what happens to the top 24-bits of `status`; you're advised to make them 0.
/// Additionally, it will be difficult to distinguish an abort code of `0` from success in higher-level transaction handling logic.
/// The `status` value of `0xFF` is unofficially reserved (and is used in `libitm`) to mean 'transaction failed due to busy lock'.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn _xabort(status: u32) -> !
{
	asm!
	(
		".byte 0xc6,0xf8,%P0"
		:
		:
			"i" (status)
		:
			"memory"
		:
			"volatile"
	);
	unreachable!("_xabort causes logic to restart at _xbegin() with a jump");
}

/// Tests whether code is executing in a transaction.
/// Will raise the signal `SIGILL` (an illegal instruction abort) on CPUs without Intel TSX support, so be careful.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn _xtest() -> bool
{
	let result: u8;
	asm!
	(
		".byte 0x0f,0x01,0xd6; setnz %0"
		:
			"=r" (result)
		:
		:
			"memory"
		:
			"volatile"
	);
	
	debug_assert!(result == 0 || result == 1, "result was neither 0 or 1, but '{}'", result);
	result == 1
}
