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
//! Autogenerated weights for `runtime_parachains::hrmp`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-16, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm4`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=polkadot-dev
// --steps=50
// --repeat=20
// --pallet=runtime_parachains::hrmp
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/runtime_parachains_hrmp.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `runtime_parachains::hrmp`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_parachains::hrmp::WeightInfo for WeightInfo<T> {
	/// Storage: Paras ParaLifecycles (r:2 w:0)
	/// Proof Skipped: Paras ParaLifecycles (max_values: None, max_size: None, mode: Measured)
	/// Storage: Configuration ActiveConfig (r:1 w:0)
	/// Proof Skipped: Configuration ActiveConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpOpenChannelRequests (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpOpenChannelRequests (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpChannels (r:1 w:0)
	/// Proof Skipped: Hrmp HrmpChannels (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpEgressChannelsIndex (r:1 w:0)
	/// Proof Skipped: Hrmp HrmpEgressChannelsIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpOpenChannelRequestCount (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpOpenChannelRequestCount (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpOpenChannelRequestsList (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpOpenChannelRequestsList (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	/// Proof Skipped: Dmp DownwardMessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	/// Proof Skipped: Dmp DownwardMessageQueueHeads (max_values: None, max_size: None, mode: Measured)
	fn hrmp_init_open_channel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `577`
		//  Estimated: `34893`
		// Minimum execution time: 39_502_000 picoseconds.
		Weight::from_parts(39_981_000, 0)
			.saturating_add(Weight::from_parts(0, 34893))
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Hrmp HrmpOpenChannelRequests (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpOpenChannelRequests (max_values: None, max_size: None, mode: Measured)
	/// Storage: Configuration ActiveConfig (r:1 w:0)
	/// Proof Skipped: Configuration ActiveConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras ParaLifecycles (r:1 w:0)
	/// Proof Skipped: Paras ParaLifecycles (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpIngressChannelsIndex (r:1 w:0)
	/// Proof Skipped: Hrmp HrmpIngressChannelsIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpAcceptedChannelRequestCount (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpAcceptedChannelRequestCount (max_values: None, max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	/// Proof Skipped: Dmp DownwardMessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	/// Proof Skipped: Dmp DownwardMessageQueueHeads (max_values: None, max_size: None, mode: Measured)
	fn hrmp_accept_open_channel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `809`
		//  Estimated: `27938`
		// Minimum execution time: 40_489_000 picoseconds.
		Weight::from_parts(41_076_000, 0)
			.saturating_add(Weight::from_parts(0, 27938))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Hrmp HrmpChannels (r:1 w:0)
	/// Proof Skipped: Hrmp HrmpChannels (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpCloseChannelRequests (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpCloseChannelRequests (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpCloseChannelRequestsList (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpCloseChannelRequestsList (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Configuration ActiveConfig (r:1 w:0)
	/// Proof Skipped: Configuration ActiveConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	/// Proof Skipped: Dmp DownwardMessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	/// Proof Skipped: Dmp DownwardMessageQueueHeads (max_values: None, max_size: None, mode: Measured)
	fn hrmp_close_channel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `680`
		//  Estimated: `20910`
		// Minimum execution time: 36_361_000 picoseconds.
		Weight::from_parts(37_148_000, 0)
			.saturating_add(Weight::from_parts(0, 20910))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Hrmp HrmpIngressChannelsIndex (r:128 w:128)
	/// Proof Skipped: Hrmp HrmpIngressChannelsIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpEgressChannelsIndex (r:128 w:128)
	/// Proof Skipped: Hrmp HrmpEgressChannelsIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpChannels (r:254 w:254)
	/// Proof Skipped: Hrmp HrmpChannels (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpAcceptedChannelRequestCount (r:0 w:1)
	/// Proof Skipped: Hrmp HrmpAcceptedChannelRequestCount (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpChannelContents (r:0 w:254)
	/// Proof Skipped: Hrmp HrmpChannelContents (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpOpenChannelRequestCount (r:0 w:1)
	/// Proof Skipped: Hrmp HrmpOpenChannelRequestCount (max_values: None, max_size: None, mode: Measured)
	/// The range of component `i` is `[0, 127]`.
	/// The range of component `e` is `[0, 127]`.
	fn force_clean_hrmp(i: u32, e: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `197 + i * (100 ±0) + e * (100 ±0)`
		//  Estimated: `9084 + i * (5550 ±0) + e * (5550 ±0)`
		// Minimum execution time: 1_074_045_000 picoseconds.
		Weight::from_parts(1_079_931_000, 0)
			.saturating_add(Weight::from_parts(0, 9084))
			// Standard Error: 96_252
			.saturating_add(Weight::from_parts(3_082_744, 0).saturating_mul(i.into()))
			// Standard Error: 96_252
			.saturating_add(Weight::from_parts(3_118_809, 0).saturating_mul(e.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(i.into())))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(e.into())))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(i.into())))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(e.into())))
			.saturating_add(Weight::from_parts(0, 5550).saturating_mul(i.into()))
			.saturating_add(Weight::from_parts(0, 5550).saturating_mul(e.into()))
	}
	/// Storage: Configuration ActiveConfig (r:1 w:0)
	/// Proof Skipped: Configuration ActiveConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpOpenChannelRequestsList (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpOpenChannelRequestsList (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpOpenChannelRequests (r:128 w:128)
	/// Proof Skipped: Hrmp HrmpOpenChannelRequests (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras ParaLifecycles (r:256 w:0)
	/// Proof Skipped: Paras ParaLifecycles (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpIngressChannelsIndex (r:128 w:128)
	/// Proof Skipped: Hrmp HrmpIngressChannelsIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpEgressChannelsIndex (r:128 w:128)
	/// Proof Skipped: Hrmp HrmpEgressChannelsIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpOpenChannelRequestCount (r:128 w:128)
	/// Proof Skipped: Hrmp HrmpOpenChannelRequestCount (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpAcceptedChannelRequestCount (r:128 w:128)
	/// Proof Skipped: Hrmp HrmpAcceptedChannelRequestCount (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpChannels (r:0 w:128)
	/// Proof Skipped: Hrmp HrmpChannels (max_values: None, max_size: None, mode: Measured)
	/// The range of component `c` is `[0, 128]`.
	fn force_process_hrmp_open(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `652 + c * (136 ±0)`
		//  Estimated: `14676 + c * (18549 ±0)`
		// Minimum execution time: 9_678_000 picoseconds.
		Weight::from_parts(6_837_820, 0)
			.saturating_add(Weight::from_parts(0, 14676))
			// Standard Error: 20_096
			.saturating_add(Weight::from_parts(18_573_440, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((7_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((6_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 18549).saturating_mul(c.into()))
	}
	/// Storage: Hrmp HrmpCloseChannelRequestsList (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpCloseChannelRequestsList (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpChannels (r:128 w:128)
	/// Proof Skipped: Hrmp HrmpChannels (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpEgressChannelsIndex (r:128 w:128)
	/// Proof Skipped: Hrmp HrmpEgressChannelsIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpIngressChannelsIndex (r:128 w:128)
	/// Proof Skipped: Hrmp HrmpIngressChannelsIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpCloseChannelRequests (r:0 w:128)
	/// Proof Skipped: Hrmp HrmpCloseChannelRequests (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpChannelContents (r:0 w:128)
	/// Proof Skipped: Hrmp HrmpChannelContents (max_values: None, max_size: None, mode: Measured)
	/// The range of component `c` is `[0, 128]`.
	fn force_process_hrmp_close(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `268 + c * (124 ±0)`
		//  Estimated: `5973 + c * (8175 ±0)`
		// Minimum execution time: 6_545_000 picoseconds.
		Weight::from_parts(3_839_847, 0)
			.saturating_add(Weight::from_parts(0, 5973))
			// Standard Error: 13_189
			.saturating_add(Weight::from_parts(11_397_155, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((5_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 8175).saturating_mul(c.into()))
	}
	/// Storage: Hrmp HrmpOpenChannelRequestsList (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpOpenChannelRequestsList (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpOpenChannelRequests (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpOpenChannelRequests (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpOpenChannelRequestCount (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpOpenChannelRequestCount (max_values: None, max_size: None, mode: Measured)
	/// The range of component `c` is `[0, 128]`.
	fn hrmp_cancel_open_request(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `959 + c * (13 ±0)`
		//  Estimated: `10704 + c * (45 ±0)`
		// Minimum execution time: 22_478_000 picoseconds.
		Weight::from_parts(27_026_265, 0)
			.saturating_add(Weight::from_parts(0, 10704))
			// Standard Error: 1_040
			.saturating_add(Weight::from_parts(53_928, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 45).saturating_mul(c.into()))
	}
	/// Storage: Hrmp HrmpOpenChannelRequestsList (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpOpenChannelRequestsList (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpOpenChannelRequests (r:128 w:128)
	/// Proof Skipped: Hrmp HrmpOpenChannelRequests (max_values: None, max_size: None, mode: Measured)
	/// The range of component `c` is `[0, 128]`.
	fn clean_open_channel_requests(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `176 + c * (63 ±0)`
		//  Estimated: `2821 + c * (2602 ±0)`
		// Minimum execution time: 5_297_000 picoseconds.
		Weight::from_parts(3_420_027, 0)
			.saturating_add(Weight::from_parts(0, 2821))
			// Standard Error: 4_062
			.saturating_add(Weight::from_parts(3_203_130, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 2602).saturating_mul(c.into()))
	}
	/// Storage: Paras ParaLifecycles (r:2 w:0)
	/// Proof Skipped: Paras ParaLifecycles (max_values: None, max_size: None, mode: Measured)
	/// Storage: Configuration ActiveConfig (r:1 w:0)
	/// Proof Skipped: Configuration ActiveConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpOpenChannelRequests (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpOpenChannelRequests (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpChannels (r:1 w:0)
	/// Proof Skipped: Hrmp HrmpChannels (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpEgressChannelsIndex (r:1 w:0)
	/// Proof Skipped: Hrmp HrmpEgressChannelsIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpOpenChannelRequestCount (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpOpenChannelRequestCount (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpOpenChannelRequestsList (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpOpenChannelRequestsList (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueues (r:2 w:2)
	/// Proof Skipped: Dmp DownwardMessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: Dmp DownwardMessageQueueHeads (r:2 w:2)
	/// Proof Skipped: Dmp DownwardMessageQueueHeads (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpIngressChannelsIndex (r:1 w:0)
	/// Proof Skipped: Hrmp HrmpIngressChannelsIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: Hrmp HrmpAcceptedChannelRequestCount (r:1 w:1)
	/// Proof Skipped: Hrmp HrmpAcceptedChannelRequestCount (max_values: None, max_size: None, mode: Measured)
	fn force_open_hrmp_channel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `577`
		//  Estimated: `47927`
		// Minimum execution time: 53_734_000 picoseconds.
		Weight::from_parts(54_206_000, 0)
			.saturating_add(Weight::from_parts(0, 47927))
			.saturating_add(T::DbWeight::get().reads(14))
			.saturating_add(T::DbWeight::get().writes(8))
	}
}
