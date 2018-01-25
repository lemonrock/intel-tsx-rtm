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
	#[inline(always)]
	#[cfg(target_arch = "x86_64")]
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
	#[inline(always)]
	#[cfg(not(target_arch = "x86_64"))]
	pub fn new() -> Self
	{
		Self
		{
		}
	}
	
	/// Executes the transaction once.
	/// Panics on non x86_64 platforms.
	/// Panics on x86_64 platforms which don't have transactions if running with debug assertions enabled.
	#[inline(always)]
	#[cfg(target_arch = "x86_64")]
	pub fn execute_transaction_once<TransactionCallback: FnMut() -> u8>(&self, mut transaction_callback: TransactionCallback) -> Result<(), HardwareMemoryTransactionResult>
	{
		debug_assert!(self.cpu_supports_hardware_transactions(), "This x86_64 CPU does not have hardware transactions that we can use");
		
		let mut trait_object: &mut FnMut() -> u8 = &mut transaction_callback;
		let pointer_to_pointer: extern fn() -> u8 = unsafe { transmute(&mut trait_object) };
		
		let status = unsafe { hsx_transaction(pointer_to_pointer) };
		if status == 0
		{
			Ok(())
		}
		else
		{
			Err(HardwareMemoryTransactionResult::new(status))
		}
	}
	
	/// Executes the transaction once.
	/// Panics on non x86_64 platforms.
	/// Panics on x86_64 platforms which don't have transactions if running with debug assertions enabled.
	#[inline(always)]
	#[cfg(not(target_arch = "x86_64"))]
	pub fn execute_transaction_once<TransactionCallback: FnMut() -> u8>(mut transaction_callback: TransactionCallback) -> Self
	{
		panic!("TSX hardware transactions are not supported on non x86_64 platforms")
	}
	
	/// Does this x86_64 CPU have hardware transactions (TSX)?
	/// This will always be false for non-x86-64 platforms.
	#[inline(always)]
	#[cfg(target_arch = "x86_64")]
	pub fn cpu_supports_hardware_transactions(&self) -> bool
	{
		self.cpu_supports_hardware_transactions
	}
	
	/// Does this x86_64 CPU have hardware transactions (TSX)?
	/// This will always be false for non-x86-64 platforms.
	#[inline(always)]
	#[cfg(not(target_arch = "x86_64"))]
	pub fn cpu_supports_hardware_transactions(&self) -> bool
	{
		false
	}
	
	/// Use this to detect if inside a transaction callback.
	/// Always false if TSX transactions are not supported (eg not x86-64, not a recent Intel Skylake CPU).
	/// Can be used to prevent nested transactions, and in code meant to run either as a transaction or in a fallback.
	#[inline(always)]
	#[cfg(target_arch = "x86_64")]
	pub fn is_inside_a_running_transaction(&self) -> bool
	{
		if cfg!(target_arch = "x86_64")
		{
			if self.cpu_supports_hardware_transactions()
			{
				unsafe { hsx_xtest() != 0 }
			}
			else
			{
				false
			}
		}
		else
		{
			false
		}
	}
}
