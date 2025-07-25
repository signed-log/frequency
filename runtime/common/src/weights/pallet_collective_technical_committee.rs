//! Autogenerated weights for pallet_collective
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 47.2.0
//! DATE: 2025-07-15, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-10-173-5-194`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/frequency
// benchmark
// pallet
// --pallet=pallet_collective
// --extrinsic
// *
// --heap-pages=4096
// --steps=50
// --repeat=20
// --output=./scripts/../runtime/common/src/weights
// --template=./scripts/../.maintain/runtime-weight-template.hbs
// --additional-trie-layers=3
// --runtime=./scripts/../target/release/wbuild/frequency-runtime/frequency_runtime.wasm
// --genesis-builder=runtime

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weights for `pallet_collective` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for SubstrateWeight<T> {
	/// Storage: `TechnicalCommittee::Members` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Proposals` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Voting` (r:25 w:25)
	/// Proof: `TechnicalCommittee::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Prime` (r:0 w:1)
	/// Proof: `TechnicalCommittee::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[0, 10]`.
	/// The range of component `n` is `[0, 10]`.
	/// The range of component `p` is `[0, 25]`.
	/// The range of component `m` is `[0, 10]`.
	/// The range of component `n` is `[0, 10]`.
	/// The range of component `p` is `[0, 25]`.
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + m * (832 ±0) + p * (310 ±0)`
		//  Estimated: `4856 + m * (489 ±7) + p * (2615 ±2)`
		// Minimum execution time: 9_363_000 picoseconds.
		Weight::from_parts(9_634_000, 4856)
			// Standard Error: 107_779
			.saturating_add(Weight::from_parts(3_696_037, 0).saturating_mul(m.into()))
			// Standard Error: 43_797
			.saturating_add(Weight::from_parts(4_139_524, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p.into())))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
			.saturating_add(Weight::from_parts(0, 489).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 2615).saturating_mul(p.into()))
	}
	/// Storage: `TechnicalCommittee::Members` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 10]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 10]`.
	fn execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `137 + m * (32 ±0)`
		//  Estimated: `2116 + m * (32 ±0)`
		// Minimum execution time: 13_151_000 picoseconds.
		Weight::from_parts(13_485_214, 2116)
			// Standard Error: 26
			.saturating_add(Weight::from_parts(1_431, 0).saturating_mul(b.into()))
			// Standard Error: 2_798
			.saturating_add(Weight::from_parts(17_323, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
	}
	/// Storage: `TechnicalCommittee::Members` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::ProposalOf` (r:1 w:0)
	/// Proof: `TechnicalCommittee::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 10]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 10]`.
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `137 + m * (32 ±0)`
		//  Estimated: `4096 + m * (32 ±0)`
		// Minimum execution time: 16_080_000 picoseconds.
		Weight::from_parts(16_537_264, 4096)
			// Standard Error: 28
			.saturating_add(Weight::from_parts(1_485, 0).saturating_mul(b.into()))
			// Standard Error: 3_099
			.saturating_add(Weight::from_parts(27_290, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
	}
	/// Storage: `TechnicalCommittee::Members` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::ProposalOf` (r:1 w:1)
	/// Proof: `TechnicalCommittee::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Proposals` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::ProposalCount` (r:1 w:1)
	/// Proof: `TechnicalCommittee::ProposalCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Voting` (r:0 w:1)
	/// Proof: `TechnicalCommittee::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 10]`.
	/// The range of component `p` is `[1, 25]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 10]`.
	/// The range of component `p` is `[1, 25]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `180 + m * (32 ±0) + p * (51 ±0)`
		//  Estimated: `4092 + m * (42 ±0) + p * (49 ±0)`
		// Minimum execution time: 21_557_000 picoseconds.
		Weight::from_parts(22_853_996, 4092)
			// Standard Error: 104
			.saturating_add(Weight::from_parts(2_320, 0).saturating_mul(b.into()))
			// Standard Error: 12_375
			.saturating_add(Weight::from_parts(181_401, 0).saturating_mul(m.into()))
			// Standard Error: 4_318
			.saturating_add(Weight::from_parts(350_549, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(Weight::from_parts(0, 42).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 49).saturating_mul(p.into()))
	}
	/// Storage: `TechnicalCommittee::Members` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Voting` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[5, 10]`.
	/// The range of component `m` is `[5, 10]`.
	fn vote(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `638 + m * (64 ±0)`
		//  Estimated: `4598 + m * (64 ±0)`
		// Minimum execution time: 22_127_000 picoseconds.
		Weight::from_parts(23_500_406, 4598)
			// Standard Error: 7_031
			.saturating_add(Weight::from_parts(52_907, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
	}
	/// Storage: `TechnicalCommittee::Voting` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Members` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Proposals` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::ProposalOf` (r:0 w:1)
	/// Proof: `TechnicalCommittee::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[4, 10]`.
	/// The range of component `p` is `[1, 25]`.
	/// The range of component `m` is `[4, 10]`.
	/// The range of component `p` is `[1, 25]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `108 + m * (64 ±0) + p * (54 ±0)`
		//  Estimated: `4086 + m * (80 ±0) + p * (50 ±0)`
		// Minimum execution time: 24_856_000 picoseconds.
		Weight::from_parts(25_475_098, 4086)
			// Standard Error: 9_759
			.saturating_add(Weight::from_parts(154_120, 0).saturating_mul(m.into()))
			// Standard Error: 2_603
			.saturating_add(Weight::from_parts(263_255, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 80).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 50).saturating_mul(p.into()))
	}
	/// Storage: `TechnicalCommittee::Voting` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Members` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::ProposalOf` (r:1 w:1)
	/// Proof: `TechnicalCommittee::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Proposals` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 10]`.
	/// The range of component `p` is `[1, 25]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 10]`.
	/// The range of component `p` is `[1, 25]`.
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `139 + b * (1 ±0) + m * (64 ±0) + p * (72 ±0)`
		//  Estimated: `4092 + b * (1 ±0) + m * (85 ±1) + p * (65 ±0)`
		// Minimum execution time: 34_693_000 picoseconds.
		Weight::from_parts(34_023_703, 4092)
			// Standard Error: 92
			.saturating_add(Weight::from_parts(1_978, 0).saturating_mul(b.into()))
			// Standard Error: 14_259
			.saturating_add(Weight::from_parts(127_043, 0).saturating_mul(m.into()))
			// Standard Error: 3_815
			.saturating_add(Weight::from_parts(430_556, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 85).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 65).saturating_mul(p.into()))
	}
	/// Storage: `TechnicalCommittee::Voting` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Members` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Prime` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Proposals` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::ProposalOf` (r:0 w:1)
	/// Proof: `TechnicalCommittee::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[4, 10]`.
	/// The range of component `p` is `[1, 25]`.
	/// The range of component `m` is `[4, 10]`.
	/// The range of component `p` is `[1, 25]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `128 + m * (64 ±0) + p * (54 ±0)`
		//  Estimated: `4106 + m * (80 ±0) + p * (50 ±0)`
		// Minimum execution time: 27_245_000 picoseconds.
		Weight::from_parts(27_222_277, 4106)
			// Standard Error: 9_956
			.saturating_add(Weight::from_parts(206_334, 0).saturating_mul(m.into()))
			// Standard Error: 2_655
			.saturating_add(Weight::from_parts(270_377, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 80).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 50).saturating_mul(p.into()))
	}
	/// Storage: `TechnicalCommittee::Voting` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Members` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Prime` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::ProposalOf` (r:1 w:1)
	/// Proof: `TechnicalCommittee::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Proposals` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 10]`.
	/// The range of component `p` is `[1, 25]`.
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 10]`.
	/// The range of component `p` is `[1, 25]`.
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `159 + b * (1 ±0) + m * (64 ±0) + p * (72 ±0)`
		//  Estimated: `4112 + b * (1 ±0) + m * (85 ±1) + p * (65 ±0)`
		// Minimum execution time: 36_961_000 picoseconds.
		Weight::from_parts(35_310_359, 4112)
			// Standard Error: 94
			.saturating_add(Weight::from_parts(2_361, 0).saturating_mul(b.into()))
			// Standard Error: 14_548
			.saturating_add(Weight::from_parts(183_992, 0).saturating_mul(m.into()))
			// Standard Error: 3_892
			.saturating_add(Weight::from_parts(451_690, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 85).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 65).saturating_mul(p.into()))
	}
	/// Storage: `TechnicalCommittee::Proposals` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Voting` (r:0 w:1)
	/// Proof: `TechnicalCommittee::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::ProposalOf` (r:0 w:1)
	/// Proof: `TechnicalCommittee::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `p` is `[1, 25]`.
	/// The range of component `p` is `[1, 25]`.
	fn disapprove_proposal(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `293 + p * (32 ±0)`
		//  Estimated: `2273 + p * (32 ±0)`
		// Minimum execution time: 14_044_000 picoseconds.
		Weight::from_parts(14_830_487, 2273)
			// Standard Error: 1_579
			.saturating_add(Weight::from_parts(220_348, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(p.into()))
	}
	/// Storage: `TechnicalCommittee::ProposalOf` (r:1 w:1)
	/// Proof: `TechnicalCommittee::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::CostOf` (r:1 w:0)
	/// Proof: `TechnicalCommittee::CostOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Proposals` (r:1 w:1)
	/// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Voting` (r:0 w:1)
	/// Proof: `TechnicalCommittee::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `d` is `[0, 1]`.
	/// The range of component `p` is `[1, 25]`.
	/// The range of component `d` is `[0, 1]`.
	/// The range of component `p` is `[1, 25]`.
	fn kill(d: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1252 + p * (52 ±0)`
		//  Estimated: `5257 + d * (88 ±2) + p * (51 ±0)`
		// Minimum execution time: 22_383_000 picoseconds.
		Weight::from_parts(23_247_550, 5257)
			// Standard Error: 45_752
			.saturating_add(Weight::from_parts(502_352, 0).saturating_mul(d.into()))
			// Standard Error: 2_840
			.saturating_add(Weight::from_parts(324_842, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 88).saturating_mul(d.into()))
			.saturating_add(Weight::from_parts(0, 51).saturating_mul(p.into()))
	}
	/// Storage: `TechnicalCommittee::ProposalOf` (r:1 w:0)
	/// Proof: `TechnicalCommittee::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::CostOf` (r:1 w:0)
	/// Proof: `TechnicalCommittee::CostOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn release_proposal_cost() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `742`
		//  Estimated: `4702`
		// Minimum execution time: 13_248_000 picoseconds.
		Weight::from_parts(13_869_000, 4702)
			.saturating_add(T::DbWeight::get().reads(2_u64))
	}
}


#[cfg(test)]
mod tests {
  use frame_support::{traits::Get, weights::Weight, dispatch::DispatchClass};
  use crate::constants::{MAXIMUM_BLOCK_WEIGHT, NORMAL_DISPATCH_RATIO};
  use crate::weights::extrinsic_weights::ExtrinsicBaseWeight;

  #[allow(dead_code)]
  struct BlockWeights;
  impl Get<frame_system::limits::BlockWeights> for BlockWeights {
  	fn get() -> frame_system::limits::BlockWeights {
  		frame_system::limits::BlockWeights::builder()
  			.base_block(Weight::zero())
  			.for_class(DispatchClass::all(), |weights| {
  				weights.base_extrinsic = ExtrinsicBaseWeight::get();
  			})
  			.for_class(DispatchClass::non_mandatory(), |weights| {
  				weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
  			})
  			.build_or_panic()
  	}
  }

	#[test]
	fn test_set_members() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4856
		);
	}
	#[test]
	fn test_execute() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 2116
		);
	}
	#[test]
	fn test_propose_execute() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4096
		);
	}
	#[test]
	fn test_propose_proposed() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4092
		);
	}
	#[test]
	fn test_vote() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4598
		);
	}
	#[test]
	fn test_close_early_disapproved() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4086
		);
	}
	#[test]
	fn test_close_early_approved() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4092
		);
	}
	#[test]
	fn test_close_disapproved() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4106
		);
	}
	#[test]
	fn test_close_approved() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4112
		);
	}
	#[test]
	fn test_disapprove_proposal() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 2273
		);
	}
	#[test]
	fn test_kill() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 5257
		);
	}
	#[test]
	fn test_release_proposal_cost() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4702
		);
	}
}
