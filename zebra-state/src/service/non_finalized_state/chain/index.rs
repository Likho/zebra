//! Transparent address indexes for non-finalized chains.

use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    ops::RangeInclusive,
};

use mset::MultiSet;

use zebra_chain::{
    amount::{Amount, NegativeAllowed},
    block::Height,
    transaction, transparent,
};

use crate::{OutputLocation, TransactionLocation, ValidateContextError};

use super::{RevertPosition, UpdateWith};

/// Returns the transaction location for an [`transparent::OrderedUtxo`].
pub fn transaction_location(ordered_utxo: &transparent::OrderedUtxo) -> TransactionLocation {
    TransactionLocation::from_usize(ordered_utxo.utxo.height, ordered_utxo.tx_index_in_block)
}
