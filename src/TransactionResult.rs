// This file is part of intel-tsx-rtm. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-tsx-rtm/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of intel-tsx-rtm. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-tsx-rtm/master/COPYRIGHT.


bitflags!
{
	/// This structure wraps up both creating transactions and getting the results of an execution of a transaction.
	pub struct TransactionResult: u32
	{
		#[doc(hidden)]
		const _XABORT_EXPLICIT = (1 << 0);
		
		#[doc(hidden)]
		const _XABORT_RETRY = (1 << 1);
		
		#[doc(hidden)]
		const _XABORT_CONFLICT = (1 << 2);
		
		#[doc(hidden)]
		const _XABORT_CAPACITY = (1 << 3);
		
		#[doc(hidden)]
		const _XABORT_DEBUG = (1 << 4);
		
		#[doc(hidden)]
		const _XABORT_NESTED = (1 << 5);
		
//		// From Andi Kleen's tsx-tools `rtm-patched.h`
//		#[doc(hidden)]
//		const _XBEGIN_SOFTWARE = -2;
	}
}

impl TransactionResult
{
	/// Return this from `TransactionCallback` if a transaction is successful.
	pub const TransactionIsSuccessful: u8 = 0;
	
	/// Return this from `TransactionCallback` if a transaction fails due to a busy lock. Unofficial, see source code comments in <https://github.com/gcc-mirror/gcc/blob/da8dff89fa9398f04b107e388cb706517ced9505/libitm/config/x86/target.h>.
	pub const TransactionFailedDueToBusyLock: u8 = 0xFF;
	
	/// Use this to detect if inside a transaction callback.
	/// Always false if TSX transactions are not supported.
	/// Can be used to prevent nested transactions, and in code meant to run either as a transaction or in a fallback.
	#[inline(always)]
	pub fn is_inside_a_running_transaction() -> bool
	{
		unsafe { hsx_xtest() != 0 }
	}
	
	/// Executes the transaction once.
	pub fn execute_transaction_once<TransactionCallback: FnMut() -> u8>(mut transaction_callback: TransactionCallback) -> TransactionResult
	{
		let mut trait_object: &mut FnMut() -> u8 = &mut transaction_callback;
		let pointer_to_pointer: extern fn() -> u8 = unsafe { transmute(&mut trait_object) };
		
		let status = unsafe { hsx_transaction(pointer_to_pointer) };
		TransactionResult::new(status)
	}
	
	#[inline(always)]
	fn new(status: u32) -> Self
	{
		Self
		{
			bits: status,
		}
	}
	
	/// The transaction succeeded.
	#[inline(always)]
	pub fn transaction_was_successful(self) -> bool
	{
		self.is_empty()
	}
	
//	/// The transaction can never success, because the hardware does not support TSX.
//	/// This is only checked for when compiled with a specialized form of `tsx-tools`.
//	#[inline(always)]
//	pub fn transaction_can_never_succeed_because_hardware_does_not_support_tsx(self) -> bool
//	{
//		self == Self::_XBEGIN_SOFTWARE
//	}
	
	/// Returns `Some(status_code)` if explicitly aborted.
	/// `status_code` will never be zero.
	/// Transaction was explicitly aborted with `_xabort()`. The parameter passed to `_xabort` is available with `_XABORT_CODE(status)`.
	#[inline(always)]
	pub fn transaction_was_explicitly_aborted_by_callback(self) -> Option<u8>
	{
		if self.contains(Self::_XABORT_EXPLICIT)
		{
			// Equivalent to `_XCODE_ABORT(status)`.
			let status = (self.bits >> 24) & 0xFF;
			Some(status as u8)
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
