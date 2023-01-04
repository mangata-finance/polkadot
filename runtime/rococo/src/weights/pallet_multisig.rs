// Copyright 2017-2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `pallet_multisig`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-16, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm5`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("rococo-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=rococo-dev
// --steps=50
// --repeat=20
// --pallet=pallet_multisig
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/rococo/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_multisig`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_multisig::WeightInfo for WeightInfo<T> {
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_threshold_1(z: u32, ) -> Weight {
		// Minimum execution time: 15_772 nanoseconds.
		Weight::from_ref_time(16_255_218)
			// Standard Error: 2
			.saturating_add(Weight::from_ref_time(597).saturating_mul(z.into()))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_create(s: u32, z: u32, ) -> Weight {
		// Minimum execution time: 45_441 nanoseconds.
		Weight::from_ref_time(37_064_110)
			// Standard Error: 910
			.saturating_add(Weight::from_ref_time(95_519).saturating_mul(s.into()))
			// Standard Error: 8
			.saturating_add(Weight::from_ref_time(1_540).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	/// The range of component `s` is `[3, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_approve(s: u32, z: u32, ) -> Weight {
		// Minimum execution time: 35_187 nanoseconds.
		Weight::from_ref_time(27_053_340)
			// Standard Error: 605
			.saturating_add(Weight::from_ref_time(88_646).saturating_mul(s.into()))
			// Standard Error: 5
			.saturating_add(Weight::from_ref_time(1_551).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_complete(s: u32, z: u32, ) -> Weight {
		// Minimum execution time: 47_483 nanoseconds.
		Weight::from_ref_time(37_293_379)
			// Standard Error: 796
			.saturating_add(Weight::from_ref_time(118_230).saturating_mul(s.into()))
			// Standard Error: 7
			.saturating_add(Weight::from_ref_time(1_692).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_create(s: u32, ) -> Weight {
		// Minimum execution time: 32_427 nanoseconds.
		Weight::from_ref_time(35_007_792)
			// Standard Error: 1_114
			.saturating_add(Weight::from_ref_time(93_830).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_approve(s: u32, ) -> Weight {
		// Minimum execution time: 22_891 nanoseconds.
		Weight::from_ref_time(24_948_736)
			// Standard Error: 731
			.saturating_add(Weight::from_ref_time(86_594).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	/// The range of component `s` is `[2, 100]`.
	fn cancel_as_multi(s: u32, ) -> Weight {
		// Minimum execution time: 32_571 nanoseconds.
		Weight::from_ref_time(34_813_263)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(89_038).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
