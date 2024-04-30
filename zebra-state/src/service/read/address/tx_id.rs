//! Reading address transaction IDs.
//!
//! In the functions in this module:
//!
//! The block write task commits blocks to the finalized state before updating
//! `chain` with a cached copy of the best non-finalized chain from
//! `NonFinalizedState.chain_set`. Then the block commit task can commit additional blocks to
//! the finalized state after we've cloned the `chain`.
//!
//! This means that some blocks can be in both:
//! - the cached [`Chain`], and
//! - the shared finalized [`ZebraDb`] reference.

use std::{
    collections::{BTreeMap, HashSet},
    ops::RangeInclusive,
};

use zebra_chain::{block::Height, transaction, transparent};

use crate::{
    service::{
        finalized_state::ZebraDb, non_finalized_state::Chain, read::FINALIZED_STATE_QUERY_RETRIES,
    },
    BoxError, TransactionLocation,
};


/// Returns the combined finalized and non-finalized transaction IDs.
fn apply_tx_id_changes(
    finalized_tx_ids: BTreeMap<TransactionLocation, transaction::Hash>,
    chain_tx_ids: BTreeMap<TransactionLocation, transaction::Hash>,
) -> BTreeMap<TransactionLocation, transaction::Hash> {
    // Correctness: compensate for inconsistent tx IDs finalized blocks across multiple addresses,
    // by combining them with overlapping non-finalized block tx IDs.
    finalized_tx_ids.into_iter().chain(chain_tx_ids).collect()
}
