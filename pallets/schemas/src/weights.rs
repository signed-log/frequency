
//! Autogenerated weights for `pallet_schemas`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 47.2.0
//! DATE: 2025-07-15, STEPS: `20`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-10-173-5-194`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: `1024`

// Executed Command:
// ./scripts/../target/release/frequency
// benchmark
// pallet
// --pallet=pallet_schemas
// --extrinsic
// *
// --heap-pages=4096
// --steps=20
// --repeat=10
// --output=./scripts/../pallets/schemas/src/weights.rs
// --template=./scripts/../.maintain/frame-weight-template.hbs
// --additional-trie-layers=3
// --runtime=./scripts/../target/release/wbuild/frequency-runtime/frequency_runtime.wasm
// --genesis-builder=runtime

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_schemas`.
pub trait WeightInfo {
	fn create_schema_v3_with_name(m: u32, ) -> Weight;
	fn create_schema_v3_without_name(m: u32, ) -> Weight;
	fn set_max_schema_model_bytes() -> Weight;
	fn create_schema_via_governance_v2_with_name(m: u32, ) -> Weight;
	fn create_schema_via_governance_v2_without_name(m: u32, ) -> Weight;
	fn propose_to_create_schema_v2_with_name(m: u32, ) -> Weight;
	fn propose_to_create_schema_v2_without_name(m: u32, ) -> Weight;
	fn propose_to_create_schema_name() -> Weight;
	fn create_schema_name_via_governance() -> Weight;
}

/// Weights for `pallet_schemas` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Schemas::GovernanceSchemaModelMaxBytes` (r:1 w:0)
	/// Proof: `Schemas::GovernanceSchemaModelMaxBytes` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::CurrentSchemaIdentifierMaximum` (r:1 w:1)
	/// Proof: `Schemas::CurrentSchemaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaNameToIds` (r:1 w:1)
	/// Proof: `Schemas::SchemaNameToIds` (`max_values`: None, `max_size`: Some(602), added: 3077, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:0 w:1)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaPayloads` (r:0 w:1)
	/// Proof: `Schemas::SchemaPayloads` (`max_values`: None, `max_size`: Some(65514), added: 67989, mode: `MaxEncodedLen`)
	/// The range of component `m` is `[16, 65499]`.
	fn create_schema_v3_with_name(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `316`
		//  Estimated: `4562`
		// Minimum execution time: 26_262_000 picoseconds.
		Weight::from_parts(12_285_273, 4562)
			// Standard Error: 53
			.saturating_add(Weight::from_parts(35_564, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Schemas::GovernanceSchemaModelMaxBytes` (r:1 w:0)
	/// Proof: `Schemas::GovernanceSchemaModelMaxBytes` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::CurrentSchemaIdentifierMaximum` (r:1 w:1)
	/// Proof: `Schemas::CurrentSchemaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:0 w:1)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaPayloads` (r:0 w:1)
	/// Proof: `Schemas::SchemaPayloads` (`max_values`: None, `max_size`: Some(65514), added: 67989, mode: `MaxEncodedLen`)
	/// The range of component `m` is `[16, 65499]`.
	fn create_schema_v3_without_name(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `261`
		//  Estimated: `1984`
		// Minimum execution time: 16_614_000 picoseconds.
		Weight::from_parts(1_674_380, 1984)
			// Standard Error: 60
			.saturating_add(Weight::from_parts(35_543, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Schemas::GovernanceSchemaModelMaxBytes` (r:0 w:1)
	/// Proof: `Schemas::GovernanceSchemaModelMaxBytes` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn set_max_schema_model_bytes() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_165_000 picoseconds.
		Weight::from_parts(5_423_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Schemas::GovernanceSchemaModelMaxBytes` (r:1 w:0)
	/// Proof: `Schemas::GovernanceSchemaModelMaxBytes` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::CurrentSchemaIdentifierMaximum` (r:1 w:1)
	/// Proof: `Schemas::CurrentSchemaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaNameToIds` (r:1 w:1)
	/// Proof: `Schemas::SchemaNameToIds` (`max_values`: None, `max_size`: Some(602), added: 3077, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:0 w:1)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaPayloads` (r:0 w:1)
	/// Proof: `Schemas::SchemaPayloads` (`max_values`: None, `max_size`: Some(65514), added: 67989, mode: `MaxEncodedLen`)
	/// The range of component `m` is `[16, 65499]`.
	fn create_schema_via_governance_v2_with_name(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `316`
		//  Estimated: `4562`
		// Minimum execution time: 26_387_000 picoseconds.
		Weight::from_parts(14_170_193, 4562)
			// Standard Error: 50
			.saturating_add(Weight::from_parts(36_354, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Schemas::GovernanceSchemaModelMaxBytes` (r:1 w:0)
	/// Proof: `Schemas::GovernanceSchemaModelMaxBytes` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::CurrentSchemaIdentifierMaximum` (r:1 w:1)
	/// Proof: `Schemas::CurrentSchemaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:0 w:1)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaPayloads` (r:0 w:1)
	/// Proof: `Schemas::SchemaPayloads` (`max_values`: None, `max_size`: Some(65514), added: 67989, mode: `MaxEncodedLen`)
	/// The range of component `m` is `[16, 65499]`.
	fn create_schema_via_governance_v2_without_name(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `261`
		//  Estimated: `1984`
		// Minimum execution time: 16_860_000 picoseconds.
		Weight::from_parts(1_477_244, 1984)
			// Standard Error: 65
			.saturating_add(Weight::from_parts(36_386, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalCount` (r:1 w:1)
	/// Proof: `Council::ProposalCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:0 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[16, 65499]`.
	fn propose_to_create_schema_v2_with_name(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `166`
		//  Estimated: `4126`
		// Minimum execution time: 20_450_000 picoseconds.
		Weight::from_parts(6_421_671, 4126)
			// Standard Error: 47
			.saturating_add(Weight::from_parts(3_369, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalCount` (r:1 w:1)
	/// Proof: `Council::ProposalCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:0 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[16, 65499]`.
	fn propose_to_create_schema_v2_without_name(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `166`
		//  Estimated: `4126`
		// Minimum execution time: 19_867_000 picoseconds.
		Weight::from_parts(6_156_148, 4126)
			// Standard Error: 47
			.saturating_add(Weight::from_parts(3_367, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalCount` (r:1 w:1)
	/// Proof: `Council::ProposalCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:0 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn propose_to_create_schema_name() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `680`
		//  Estimated: `4640`
		// Minimum execution time: 33_037_000 picoseconds.
		Weight::from_parts(34_453_000, 4640)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaNameToIds` (r:1 w:1)
	/// Proof: `Schemas::SchemaNameToIds` (`max_values`: None, `max_size`: Some(602), added: 3077, mode: `MaxEncodedLen`)
	fn create_schema_name_via_governance() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `569`
		//  Estimated: `4562`
		// Minimum execution time: 21_855_000 picoseconds.
		Weight::from_parts(22_353_000, 4562)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `Schemas::GovernanceSchemaModelMaxBytes` (r:1 w:0)
	/// Proof: `Schemas::GovernanceSchemaModelMaxBytes` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::CurrentSchemaIdentifierMaximum` (r:1 w:1)
	/// Proof: `Schemas::CurrentSchemaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaNameToIds` (r:1 w:1)
	/// Proof: `Schemas::SchemaNameToIds` (`max_values`: None, `max_size`: Some(602), added: 3077, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:0 w:1)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaPayloads` (r:0 w:1)
	/// Proof: `Schemas::SchemaPayloads` (`max_values`: None, `max_size`: Some(65514), added: 67989, mode: `MaxEncodedLen`)
	/// The range of component `m` is `[16, 65499]`.
	fn create_schema_v3_with_name(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `316`
		//  Estimated: `4562`
		// Minimum execution time: 26_262_000 picoseconds.
		Weight::from_parts(12_285_273, 4562)
			// Standard Error: 53
			.saturating_add(Weight::from_parts(35_564, 0).saturating_mul(m.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `Schemas::GovernanceSchemaModelMaxBytes` (r:1 w:0)
	/// Proof: `Schemas::GovernanceSchemaModelMaxBytes` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::CurrentSchemaIdentifierMaximum` (r:1 w:1)
	/// Proof: `Schemas::CurrentSchemaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:0 w:1)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaPayloads` (r:0 w:1)
	/// Proof: `Schemas::SchemaPayloads` (`max_values`: None, `max_size`: Some(65514), added: 67989, mode: `MaxEncodedLen`)
	/// The range of component `m` is `[16, 65499]`.
	fn create_schema_v3_without_name(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `261`
		//  Estimated: `1984`
		// Minimum execution time: 16_614_000 picoseconds.
		Weight::from_parts(1_674_380, 1984)
			// Standard Error: 60
			.saturating_add(Weight::from_parts(35_543, 0).saturating_mul(m.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `Schemas::GovernanceSchemaModelMaxBytes` (r:0 w:1)
	/// Proof: `Schemas::GovernanceSchemaModelMaxBytes` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn set_max_schema_model_bytes() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_165_000 picoseconds.
		Weight::from_parts(5_423_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Schemas::GovernanceSchemaModelMaxBytes` (r:1 w:0)
	/// Proof: `Schemas::GovernanceSchemaModelMaxBytes` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::CurrentSchemaIdentifierMaximum` (r:1 w:1)
	/// Proof: `Schemas::CurrentSchemaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaNameToIds` (r:1 w:1)
	/// Proof: `Schemas::SchemaNameToIds` (`max_values`: None, `max_size`: Some(602), added: 3077, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:0 w:1)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaPayloads` (r:0 w:1)
	/// Proof: `Schemas::SchemaPayloads` (`max_values`: None, `max_size`: Some(65514), added: 67989, mode: `MaxEncodedLen`)
	/// The range of component `m` is `[16, 65499]`.
	fn create_schema_via_governance_v2_with_name(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `316`
		//  Estimated: `4562`
		// Minimum execution time: 26_387_000 picoseconds.
		Weight::from_parts(14_170_193, 4562)
			// Standard Error: 50
			.saturating_add(Weight::from_parts(36_354, 0).saturating_mul(m.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `Schemas::GovernanceSchemaModelMaxBytes` (r:1 w:0)
	/// Proof: `Schemas::GovernanceSchemaModelMaxBytes` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::CurrentSchemaIdentifierMaximum` (r:1 w:1)
	/// Proof: `Schemas::CurrentSchemaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:0 w:1)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaPayloads` (r:0 w:1)
	/// Proof: `Schemas::SchemaPayloads` (`max_values`: None, `max_size`: Some(65514), added: 67989, mode: `MaxEncodedLen`)
	/// The range of component `m` is `[16, 65499]`.
	fn create_schema_via_governance_v2_without_name(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `261`
		//  Estimated: `1984`
		// Minimum execution time: 16_860_000 picoseconds.
		Weight::from_parts(1_477_244, 1984)
			// Standard Error: 65
			.saturating_add(Weight::from_parts(36_386, 0).saturating_mul(m.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalCount` (r:1 w:1)
	/// Proof: `Council::ProposalCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:0 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[16, 65499]`.
	fn propose_to_create_schema_v2_with_name(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `166`
		//  Estimated: `4126`
		// Minimum execution time: 20_450_000 picoseconds.
		Weight::from_parts(6_421_671, 4126)
			// Standard Error: 47
			.saturating_add(Weight::from_parts(3_369, 0).saturating_mul(m.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalCount` (r:1 w:1)
	/// Proof: `Council::ProposalCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:0 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[16, 65499]`.
	fn propose_to_create_schema_v2_without_name(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `166`
		//  Estimated: `4126`
		// Minimum execution time: 19_867_000 picoseconds.
		Weight::from_parts(6_156_148, 4126)
			// Standard Error: 47
			.saturating_add(Weight::from_parts(3_367, 0).saturating_mul(m.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalCount` (r:1 w:1)
	/// Proof: `Council::ProposalCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:0 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn propose_to_create_schema_name() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `680`
		//  Estimated: `4640`
		// Minimum execution time: 33_037_000 picoseconds.
		Weight::from_parts(34_453_000, 4640)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaNameToIds` (r:1 w:1)
	/// Proof: `Schemas::SchemaNameToIds` (`max_values`: None, `max_size`: Some(602), added: 3077, mode: `MaxEncodedLen`)
	fn create_schema_name_via_governance() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `569`
		//  Estimated: `4562`
		// Minimum execution time: 21_855_000 picoseconds.
		Weight::from_parts(22_353_000, 4562)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}


#[cfg(test)]
mod tests {
  use frame_support::{traits::Get, weights::Weight, dispatch::DispatchClass};
  use common_runtime::constants::{MAXIMUM_BLOCK_WEIGHT, NORMAL_DISPATCH_RATIO};
  use common_runtime::weights::extrinsic_weights::ExtrinsicBaseWeight;

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
	fn test_create_schema_v3_with_name() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4562
		);
	}
	#[test]
	fn test_create_schema_v3_without_name() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 1984
		);
	}
	#[test]
	fn test_create_schema_via_governance_v2_with_name() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4562
		);
	}
	#[test]
	fn test_create_schema_via_governance_v2_without_name() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 1984
		);
	}
	#[test]
	fn test_propose_to_create_schema_v2_with_name() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4126
		);
	}
	#[test]
	fn test_propose_to_create_schema_v2_without_name() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4126
		);
	}
	#[test]
	fn test_propose_to_create_schema_name() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4640
		);
	}
	#[test]
	fn test_create_schema_name_via_governance() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 4562
		);
	}
}
