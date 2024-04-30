//! Reading address balances.
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

use std::{collections::HashSet, sync::Arc};

use zebra_chain::{
    amount::{self, Amount, NegativeAllowed, NonNegative},
    block::Height,
    transparent,
};

use crate::{
    service::{
        finalized_state::ZebraDb, non_finalized_state::Chain, read::FINALIZED_STATE_QUERY_RETRIES,
    },
    BoxError,
};

/// Add the supplied finalized and non-finalized balances together,
/// and return the result.
fn apply_balance_change(
    finalized_balance: Amount<NonNegative>,
    chain_balance_change: Amount<NegativeAllowed>,
) -> amount::Result<Amount<NonNegative>> {
    let balance = finalized_balance.constrain()? + chain_balance_change;

    balance?.constrain()
}
