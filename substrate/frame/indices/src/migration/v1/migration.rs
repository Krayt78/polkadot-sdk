// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Migration from v0 to v1: Convert index reserves to holds.
//!
//! This migration uses multi-block execution with index preservation:
//! - Multi-block: Handles accounts with weight-limited batching without timing out
//! - Index preservation: Migration failures preserve index relationships with zero deposits
//! - No permanent fund loss, all funds move to free balance
//!
//! ## Zero-Deposit Preservation Strategy
//!
//! When hold creation fails, we preserve index relationships to avoid breaking users' access:
//!
//! ### Scenario 1: Regular Account with Successful Migration
//! ```text
//! Before migration:
//! - Account A owns index 123 with 30 tokens reserved
//!
//! After successful migration:
//! - Index relationship A→123 preserved
//! - 30 tokens moved from reserves to holds
//! - Full functionality maintained seamlessly
//! ```
//!
//! ### Scenario 2: Regular Account with Failed Migration
//! ```text
//! Before migration:
//! - Account A owns index 123 with 30 tokens reserved
//! - Hold creation fails 
//!
//! After failed migration:
//! - Index relationship A→123 preserved
//! - 30 tokens unreserved to free balance
//! - Deposit field set to 0 (marking failed migration)
//! - Index continues working normally
//!
//! Self-recovery:
//! - When A wants to use the index:
//!   - System detects zero deposit with existing index
//!   - Requires deposit for the index
//!   - A provides full deposit via hold system
//! ```
//!
//! ### Scenario 3: Frozen Index Account
//! ```text
//! Before migration:
//! - Account A owns frozen index 456 with no tokens reserved (they got slashed to have the account frozen)
//! - Index is permanently assigned to A
//!
//! After migration:
//! - Index relationship A→456 preserved
//! - Frozen status maintained
//! - Index remains fully accessible!
//! ```
//! ### Implementation Details
//! 1. Always unreserves funds from the old currency system
//! 2. Attempts to create holds in the new system
//! 3. On hold failure: keeps index config intact, sets deposit to 0
//! 4. Zero deposit serves as a permanent marker for failed migration
//! 5. No additional storage needed - uses existing deposit field

extern crate alloc;
use frame_support::traits::fungible::MutateHold;
use frame_support::traits::Get;
use frame_support::traits::Currency;
use crate::pallet::{Config, Accounts, HoldReason};
use crate::{BalanceOf};
use frame_support::{
	migrations::{MigrationId, SteppedMigration, SteppedMigrationError},
	pallet_prelude::PhantomData,
	weights::WeightMeter,
};
use sp_runtime::traits::Zero;

#[cfg(feature = "try-runtime")]
use alloc::collections::btree_map::BTreeMap;

#[cfg(feature = "try-runtime")]
use alloc::vec::Vec;

const PALLET_MIGRATIONS_ID: &[u8; 18] = b"pallet-indices-mbm";

// Module containing the OLD (v0) storage items that used Currency trait.
pub mod v0 {
	use super::Config;
	use crate::pallet::Pallet;
	use frame_support::{storage_alias, Blake2_128Concat};
	use frame_support::traits::{Currency, ReservableCurrency};

	// Old balance type using Currency trait
	type OldBalanceOf<T> =
		<<T as Config>::OldCurrency as ReservableCurrency<T::AccountId>>::Balance;

	#[storage_alias]
	/// The old storage item that used Currency trait with reserves.
	pub type OldAccounts<T: Config> =
		StorageMap<Pallet<T>, Blake2_128Concat, T::AccountIndex, (T::AccountId, OldBalanceOf<T>, bool)>;
}

/// Migration from Currency trait (v0) to Fungibles trait (v1).
/// 
/// This migration converts from the old Currency system with reserves to the new
/// Fungibles system with holds, preserving all index relationships and ensuring
/// no funds are lost.
pub struct MigrateCurrencyToFungibles<T: Config, OldCurrency>(
	PhantomData<(T, OldCurrency)>,
);

impl<T: Config, OldCurrency> SteppedMigration for MigrateCurrencyToFungibles<T, OldCurrency>
{
	type Cursor = Option<T::AccountIndex>;
	type Identifier = MigrationId<18>;

	fn id() -> Self::Identifier {
		MigrationId { pallet_id: *PALLET_MIGRATIONS_ID, version_from: 0, version_to: 1 }
	}

	fn step(
		cursor: Option<Self::Cursor>,
		meter: &mut WeightMeter,
	) -> Result<Option<Self::Cursor>, SteppedMigrationError> {
		// Check if we have minimal weight to proceed
		// We need at least enough weight to read one storage item to make progress
		let min_required = T::DbWeight::get().reads(1);
		
		if meter.remaining().any_lt(min_required) {
			return Err(SteppedMigrationError::InsufficientWeight { required: min_required });
		}

		// Process one account per step
		if meter.try_consume(min_required).is_err() {
			return Ok(cursor);
		}

		// Get the iterator for the old accounts
		let mut iter = if let Some(last_key) = cursor {
			v0::OldAccounts::<T>::iter_from(v0::OldAccounts::<T>::hashed_key_for(last_key))
		} else {
			v0::OldAccounts::<T>::iter()
		};

		// If there is a next item in the iterator, perform the migration.
		if let Some((index, (account, old_deposit, frozen))) = iter.next() {
			// Convert old balance to new balance type
			let old_deposit: BalanceOf<T> = old_deposit.into();
			
			// Get current reserved balance from old currency system
			let old_reserved = OldCurrency::reserved_balance(&account);
			let reserved_balance: BalanceOf<T> = old_reserved.into();
			
			// Migrate what was actually deposited, bounded by actual reserves
			let to_migrate = old_deposit.min(reserved_balance);

			// If there is something to migrate, perform the migration
			if !to_migrate.is_zero() {
				// Unreserve from old currency system
				let old_to_migrate: <OldCurrency as Currency<<T as frame_system::Config>::AccountId>>::Balance = to_migrate.into();
				let _unreserved = OldCurrency::unreserve(&account, old_to_migrate);
				
				// Try to hold in new fungibles system
				match T::Currency::hold(&HoldReason::DepositForIndex.into(), &account, to_migrate) {
					Ok(_) => {
						// Success: migrate to new storage with hold
						Accounts::<T>::insert(index, (account, to_migrate, frozen));
					},
					Err(_) => {
						// Failed: preserve index with zero deposit
						// Funds stay in account's free balance (from unreserve)
						Accounts::<T>::insert(index, (account, Zero::zero(), frozen));
					},
				}
			} else {
				// No funds to migrate, just preserve the index
				Accounts::<T>::insert(index, (account, Zero::zero(), frozen));
			}
			
			Ok(Some(Some(index)))
		} else {
			// Migration complete
			Ok(None)
		}
	}

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, frame_support::sp_runtime::TryRuntimeError> {
		use codec::Encode;
		Ok(v0::OldAccounts::<T>::iter().collect::<BTreeMap<_, _>>().encode())
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(prev: Vec<u8>) -> Result<(), frame_support::sp_runtime::TryRuntimeError> {
		use codec::Decode;
		
		let prev_map: BTreeMap<T::AccountIndex, (T::AccountId, BalanceOf<T>, bool)> = 
			Decode::decode(&mut &prev[..])
				.expect("Failed to decode the previous storage state");

		// Verify all accounts were migrated
		for (index, (account, old_deposit, frozen)) in prev_map {
			let current = Accounts::<T>::get(index);
			match current {
				Some((current_account, current_deposit, current_frozen)) => {
					assert_eq!(current_account, account, "Account mismatch for index {:?}", index);
					assert_eq!(current_frozen, frozen, "Frozen status mismatch for index {:?}", index);
					
					// Check that either:
					// 1. Deposit was successfully migrated to hold, OR
					// 2. Deposit was set to zero (preserved with zero deposit)
					if !old_deposit.is_zero() {
						let held = T::Currency::balance_on_hold(&HoldReason::DepositForIndex.into(), &account);
						if current_deposit.is_zero() {
							// Should have zero deposit but funds released to free balance
							assert!(held.is_zero(), "Should have no holds for zero deposit account");
						} else {
							// Should have hold matching the deposit
							assert!(held >= current_deposit, "Insufficient hold for index {:?}", index);
						}
					}
				},
				None => panic!("Index {:?} was not migrated", index),
			}
		}
		
		Ok(())
	}
}