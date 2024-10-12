
//! Autogenerated weights for `pallet_epm_unsigned`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-02, STEPS: `2`, REPEAT: `1`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `gpestanas-MBP.lan`, CPU: `<UNKNOWN>`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// /Users/gpestana/cargo_target/debug/staking-node
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet-epm-unsigned
// --extrinsic
// *
// --steps
// 2
// --repeat
// 1
// --output
// unsigned_weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

pub trait WeightInfo {
	fn submit_page_unsigned(v: u32, t: u32) -> Weight;
}

/// Weight functions for `pallet_epm_unsigned`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `ElectionProviderMultiBlock::CurrentPhase` (r:1 w:0)
	/// Proof: `ElectionProviderMultiBlock::CurrentPhase` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ElectionVerifierPallet::QueuedSolutionScore` (r:1 w:0)
	/// Proof: `ElectionVerifierPallet::QueuedSolutionScore` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ElectionVerifierPallet::MinimumScore` (r:1 w:0)
	/// Proof: `ElectionVerifierPallet::MinimumScore` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ElectionProviderMultiBlock::PagedTargetSnapshot` (r:1 w:0)
	/// Proof: `ElectionProviderMultiBlock::PagedTargetSnapshot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ElectionProviderMultiBlock::PagedVoterSnapshot` (r:1 w:0)
	/// Proof: `ElectionProviderMultiBlock::PagedVoterSnapshot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Staking::ValidatorCount` (r:1 w:0)
	/// Proof: `Staking::ValidatorCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `ElectionVerifierPallet::QueuedValidVariant` (r:1 w:0)
	/// Proof: `ElectionVerifierPallet::QueuedValidVariant` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ElectionVerifierPallet::QueuedSolutionY` (r:0 w:1)
	/// Proof: `ElectionVerifierPallet::QueuedSolutionY` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ElectionVerifierPallet::LastStoredPage` (r:0 w:1)
	/// Proof: `ElectionVerifierPallet::LastStoredPage` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ElectionVerifierPallet::QueuedSolutionBackings` (r:0 w:1)
	/// Proof: `ElectionVerifierPallet::QueuedSolutionBackings` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `v` is `[32, 1024]`.
	/// The range of component `t` is `[512, 2048]`.
	fn submit_page_unsigned(v: u32, t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `11869 + t * (10 ±0) + v * (71 ±0)`
		//  Estimated: `15334 + t * (10 ±0) + v * (71 ±0)`
		// Minimum execution time: 1_382_000_000 picoseconds.
		Weight::from_parts(3_157_322_580, 0)
			.saturating_add(Weight::from_parts(0, 15334))
			// Standard Error: 80_316
			.saturating_add(Weight::from_parts(4_146_169, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 10).saturating_mul(t.into()))
			.saturating_add(Weight::from_parts(0, 71).saturating_mul(v.into()))
	}
}


impl WeightInfo for () {
	fn submit_page_unsigned(_v: u32, _t: u32) -> Weight {
	    Default::default()
	}
}