// This file is part of intel-tsx-rtm. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-tsx-rtm/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of intel-tsx-rtm. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-tsx-rtm/master/COPYRIGHT.


bitflags!
{
	/// This structure wraps the results of an execution of a transaction.
	pub struct HardwareMemoryTransactionResult: u32
	{
		#[doc(hidden)]
		const _XABORT_EXPLICIT = _XABORT_EXPLICIT;
		
		#[doc(hidden)]
		const _XABORT_RETRY = _XABORT_RETRY;
		
		#[doc(hidden)]
		const _XABORT_CONFLICT = _XABORT_CONFLICT;
		
		#[doc(hidden)]
		const _XABORT_CAPACITY = _XABORT_CAPACITY;
		
		#[doc(hidden)]
		const _XABORT_DEBUG = _XABORT_DEBUG;
		
		#[doc(hidden)]
		const _XABORT_NESTED = _XABORT_NESTED;
	}
}

impl HardwareMemoryTransactionResult
{
	/// Return this from `TransactionCallback` if a transaction fails due to a busy lock.
	/// Unofficial, see source code comments in <https://github.com/gcc-mirror/gcc/blob/da8dff89fa9398f04b107e388cb706517ced9505/libitm/config/x86/target.h>.
	pub const TransactionFailedDueToBusyLock: u8 = 0xFF;
	
	#[inline(always)]
	fn new(status: u32) -> Self
	{
		debug_assert_ne!(status, _XBEGIN_STARTED, "status should not be _XBEGIN_STARTED");
		
		Self
		{
			bits: status,
		}
	}
	
	/// Returns `Some(status_code)` if explicitly aborted.
	#[inline(always)]
	pub fn transaction_was_explicitly_aborted_by_callback(self) -> Option<u8>
	{
		if self.contains(Self::_XABORT_EXPLICIT)
		{
			Some(_XABORT_CODE(self.bits))
		}
		else
		{
			None
		}
	}
	
	/// Transaction can be retried.
	#[inline(always)]
	pub fn transaction_retry_is_possible(self) -> bool
	{
		self.contains(Self::_XABORT_RETRY)
	}
	
	/// Transaction abort due to a memory conflict with another thread.
	/// A re-try of this transaction is likely to succeed.
	/// Ideally use a back off.
	#[inline(always)]
	pub fn transaction_was_aborted_due_to_conflict_with_another_thread(self) -> bool
	{
		self.contains(Self::_XABORT_CONFLICT)
	}
	
	/// Capacity of the cache was exceeded.
	/// A re-try of this transaction might succeed, but it's not likely.
	#[inline(always)]
	pub fn transaction_was_aborted_due_to_using_too_much_memory(self) -> bool
	{
		self.contains(Self::_XABORT_CAPACITY)
	}
	
	/// Transaction aborted due to a debug trap.
	/// A re-try of this transaction is likely to succeed if the debug trap is removed.
	#[inline(always)]
	pub fn transaction_was_aborted_due_to_a_debug_trap(self) -> bool
	{
		self.contains(Self::_XABORT_DEBUG)
	}
	
	/// Transaction abort in an inner nested transaction.
	/// Transactions inside transactions are a failure of logic, and so it is highly unlikely that a retry would succeed.
	#[inline(always)]
	pub fn transaction_was_aborted_due_to_issuing_a_nested_transaction(self) -> bool
	{
		self.contains(Self::_XABORT_NESTED)
	}
}
