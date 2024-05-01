//! Transparent address index UTXO queries.
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
    collections::{BTreeMap, BTreeSet, HashSet},
    ops::RangeInclusive,
};

use zebra_chain::{block::Height, parameters::Network, transaction, transparent};

use crate::{
    service::finalized_state::ZebraDb,
    OutputLocation, TransactionLocation,
};

/// The full range of address heights.
///
/// The genesis coinbase transactions are ignored by a consensus rule,
/// so they are not included in any address indexes.
pub const ADDRESS_HEIGHTS_FULL_RANGE: RangeInclusive<Height> = Height(1)..=Height::MAX;

/// A convenience wrapper that efficiently stores unspent transparent outputs,
/// and the corresponding transaction IDs.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct AddressUtxos {
    /// A set of unspent transparent outputs.
    utxos: BTreeMap<OutputLocation, transparent::Output>,

    /// The transaction IDs for each [`OutputLocation`] in `utxos`.
    tx_ids: BTreeMap<TransactionLocation, transaction::Hash>,

    /// The configured network for this state.
    network: Network,
}

impl AddressUtxos {
    /// Creates a new set of address UTXOs.
    pub fn new(
        network: &Network,
        utxos: BTreeMap<OutputLocation, transparent::Output>,
        tx_ids: BTreeMap<TransactionLocation, transaction::Hash>,
    ) -> Self {
        Self {
            utxos,
            tx_ids,
            network: network.clone(),
        }
    }

    /// Returns an iterator that provides the unspent output, its transaction hash,
    /// its location in the chain, and the address it was sent to.
    ///
    /// The UTXOs are returned in chain order, across all addresses.
    #[allow(dead_code)]
    pub fn utxos(
        &self,
    ) -> impl Iterator<
        Item = (
            transparent::Address,
            &transaction::Hash,
            &OutputLocation,
            &transparent::Output,
        ),
    > {
        self.utxos.iter().map(|(out_loc, output)| {
            (
                output
                    .address(&self.network)
                    .expect("address indexes only contain outputs with addresses"),
                self.tx_ids
                    .get(&out_loc.transaction_location())
                    .expect("address indexes are consistent"),
                out_loc,
                output,
            )
        })
    }
}

/// Returns the unspent transparent outputs (UTXOs) for `addresses` in the finalized chain,
/// and the finalized tip heights the UTXOs were queried at.
///
/// If the addresses do not exist in the finalized `db`, returns an empty list.
//
// TODO: turn the return type into a struct?
fn finalized_address_utxos(
    db: &ZebraDb,
    addresses: &HashSet<transparent::Address>,
) -> (
    BTreeMap<OutputLocation, transparent::Output>,
    Option<RangeInclusive<Height>>,
) {
    // # Correctness
    //
    // The StateService can commit additional blocks while we are querying address UTXOs.

    // Check if the finalized state changed while we were querying it
    let start_finalized_tip = db.finalized_tip_height();

    let finalized_utxos = db.partial_finalized_address_utxos(addresses);

    let end_finalized_tip = db.finalized_tip_height();

    let finalized_tip_range = if let (Some(start_finalized_tip), Some(end_finalized_tip)) =
        (start_finalized_tip, end_finalized_tip)
    {
        Some(start_finalized_tip..=end_finalized_tip)
    } else {
        // State is empty
        None
    };

    (finalized_utxos, finalized_tip_range)
}

/// Combines the supplied finalized and non-finalized UTXOs,
/// removes the spent UTXOs, and returns the result.
fn apply_utxo_changes(
    finalized_utxos: BTreeMap<OutputLocation, transparent::Output>,
    created_chain_utxos: BTreeMap<OutputLocation, transparent::Output>,
    spent_chain_utxos: BTreeSet<OutputLocation>,
) -> BTreeMap<OutputLocation, transparent::Output> {
    // Correctness: combine the created UTXOs, then remove spent UTXOs,
    // to compensate for overlapping finalized and non-finalized blocks.
    finalized_utxos
        .into_iter()
        .chain(created_chain_utxos)
        .filter(|(utxo_location, _output)| !spent_chain_utxos.contains(utxo_location))
        .collect()
}
