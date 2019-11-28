// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

//! Consensus for the Libra Core blockchain
//!
//! Encapsulates public consensus traits and any implementations of those traits.
//! Currently, the only consensus protocol supported is LibraBFT (based on
//! [HotStuff](https://arxiv.org/pdf/1803.05069.pdf)).

#![cfg_attr(feature = "fuzzing", allow(dead_code))]
#![recursion_limit = "512"]
extern crate failure;

#[allow(unused_imports)]
#[macro_use]
extern crate debug_interface;

pub mod chained_bft;

pub mod util;

#[cfg(feature = "fuzzing")]
pub use chained_bft::event_processor_fuzzing;

/// Defines the public consensus provider traits to implement for
/// use in the Libra Core blockchain.
pub mod consensus_provider;

mod counters;

pub mod state_computer;
pub mod state_replication;
pub mod txn_manager;
