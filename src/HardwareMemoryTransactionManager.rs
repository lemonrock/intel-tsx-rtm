// This file is part of intel-tsx-rtm. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-tsx-rtm/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of intel-tsx-rtm. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-tsx-rtm/master/COPYRIGHT.


/// A simple wrapper to avoid constantly having to check for support of hardware memory transactions.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct HardwareMemoryTransactionManager
{
	#[cfg(target_arch = "x86_64")] cpu_supports_hardware_transactions: bool,
}

impl HardwareMemoryTransactionManager
{
	/// Creates a new instance.
	#[cfg(target_arch = "x86_64")]
	#[inline(always)]
	pub fn new() -> Self
	{
		Self
		{
			cpu_supports_hardware_transactions:
			{
				let cpu_id = CpuId::new();
				
				if let Some(extended_features) = cpu_id.get_extended_feature_info()
				{
					extended_features.has_rtm()
				}
				else
				{
					false
				}
			}
		}
	}
	
	/// Creates a new instance.
	#[cfg(not(target_arch = "x86_64"))]
	#[inline(always)]
	pub fn new() -> Self
	{
		Self
		{
		}
	}
	
	/// Gracefully executes the transaction once if TSX transaction are supported.
	/// If not supported, returns an Err with None.
	/// Use this when executing a transaction for the first time in a loop, then adjust to either TSX specific code (eg calling `execute_transaction_once()` which will panic if TSX is not supported) or a software transactional manager.
	#[cfg(not(target_arch = "x86_64"))]
	#[inline(always)]
	pub fn execute_transaction_once_gracefully<TransactionCallback: FnMut() -> Result<(), u8>>(&self, mut transaction_callback: TransactionCallback) -> Result<(), Option<HardwareMemoryTransactionResult>>
	{
		Err(None)
	}
	
	/// Gracefully executes the transaction once if TSX transaction are supported.
	/// If not supported, returns an Err with None.
	/// Use this when executing a transaction for the first time in a loop, then adjust to either TSX specific code (eg calling `execute_transaction_once()` which will panic if TSX is not supported) or a software transactional manager.
	#[inline(always)]
	#[cfg(target_arch = "x86_64")]
	pub fn execute_transaction_once_gracefully<TransactionCallback: FnMut() -> Result<(), u8>>(&self, transaction_callback: TransactionCallback) -> Result<(), Option<HardwareMemoryTransactionResult>>
	{
		if self.cpu_supports_hardware_transactions()
		{
			self.execute_transaction_once(transaction_callback).map_err(|error| Some(error))
		}
		else
		{
			Err(None)
		}
	}
	
	/// Executes the transaction once.
	/// Panics on non x86_64 platforms.
	/// Panics on x86_64 platforms which don't have transactions if running with debug assertions enabled.
	/// `TransactionCallback` returns a status code if it wants to explicitly abort.
	#[cfg(target_arch = "x86_64")]
	#[inline(always)]
	pub fn execute_transaction_once<TransactionCallback: FnMut() -> Result<(), u8>>(&self, mut transaction_callback: TransactionCallback) -> Result<(), HardwareMemoryTransactionResult>
	{
		debug_assert!(self.cpu_supports_hardware_transactions(), "This x86_64 CPU does not have hardware transactions that we can use");
		
		let xbegin_result_code = unsafe { _xbegin() };
		if xbegin_result_code == _XBEGIN_STARTED
		{
			match transaction_callback()
			{
				Ok(()) =>
				{
					unsafe { _xend() };
					Ok(())
				},
				Err(result_status_code) => unsafe { _xabort(result_status_code as u32) },
			}
		}
		else
		{
			Err(HardwareMemoryTransactionResult::new(xbegin_result_code))
		}
	}
	
	/// Executes the transaction once.
	/// Panics on non x86_64 platforms.
	/// Panics on x86_64 platforms which don't have transactions if running with debug assertions enabled.
	#[cfg(not(target_arch = "x86_64"))]
	#[inline(always)]
	pub fn execute_transaction_once<TransactionCallback: FnMut() -> u8>(mut transaction_callback: TransactionCallback) -> Result<(), HardwareMemoryTransactionResult>
	{
		panic!("TSX hardware transactions are not supported on non x86_64 platforms")
	}
	
	/// Does this x86_64 CPU have hardware transactions (TSX)?
	/// This will always be false for non-x86-64 platforms.
	#[cfg(target_arch = "x86_64")]
	#[inline(always)]
	pub fn cpu_supports_hardware_transactions(&self) -> bool
	{
		self.cpu_supports_hardware_transactions
	}
	
	/// Use this to detect if inside a transaction callback.
	/// Always false if TSX transactions are not supported (eg not x86-64, not a recent Intel Skylake CPU).
	/// Can be used to prevent nested transactions, and in code meant to run either as a transaction or in a fallback.
	#[inline(always)]
	#[cfg(target_arch = "x86_64")]
	pub fn is_inside_a_running_transaction(&self) -> bool
	{
		if self.cpu_supports_hardware_transactions()
		{
			unsafe { _xtest() }
		}
		else
		{
			false
		}
	}
	
	/// Does this x86_64 CPU have hardware transactions (TSX)?
	/// This will always be false for non-x86-64 platforms.
	#[cfg(not(target_arch = "x86_64"))]
	#[inline(always)]
	pub fn cpu_supports_hardware_transactions(&self) -> bool
	{
		false
	}
}
