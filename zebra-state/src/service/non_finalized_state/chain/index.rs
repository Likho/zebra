//! Transparent address indexes for non-finalized chains.

use zebra_chain::transparent;

use crate::TransactionLocation;

/// Returns the transaction location for an [`transparent::OrderedUtxo`].
pub fn transaction_location(ordered_utxo: &transparent::OrderedUtxo) -> TransactionLocation {
    TransactionLocation::from_usize(ordered_utxo.utxo.height, ordered_utxo.clone().tx_index_in_block)
}
