//!
//! UTXO record representation used by wallet transactions.
//!

use crate::imports::*;
use kaspa_addresses::Address;
use serde::{Deserialize, Serialize};

pub use kaspa_consensus_core::tx::TransactionId;

#[cfg(target_arch = "wasm32")]
use kaspa_consensus_client::TransactionOutpoint as ClientOutpoint;

#[cfg(not(target_arch = "wasm32"))]
use kaspa_consensus_core::tx::TransactionOutpoint as CoreOutpoint;

#[cfg(not(target_arch = "wasm32"))]
fn get_outpoint_index(outpoint: &kaspa_consensus_client::TransactionOutpoint) -> u32 {
    // ClientOutpoint usually has a getter or public field
    outpoint.index() 
}

#[cfg(target_arch = "wasm32")]
fn get_outpoint_index(outpoint: &kaspa_consensus_client::TransactionOutpoint) -> u32 {
    outpoint.get_index()
}

/// [`UtxoRecord`] represents an incoming transaction UTXO entry
/// stored within [`TransactionRecord`].
#[derive(Clone, Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct UtxoRecord {
    pub address: Option<Address>,
    pub index: TransactionIndexType,
    pub amount: u64,
    #[serde(rename = "scriptPubKey")]
    pub script_public_key: ScriptPublicKey,
    #[serde(rename = "isCoinbase")]
    pub is_coinbase: bool,
}

impl From<&UtxoEntryReference> for UtxoRecord {
    fn from(utxo_ref: &UtxoEntryReference) -> Self {
        let utxo = &utxo_ref.utxo;
        
        UtxoRecord {
            index: get_outpoint_index(&utxo.outpoint),
            address: utxo.address.clone(),
            amount: utxo.amount,
            script_public_key: utxo.script_public_key.clone(),
            is_coinbase: utxo.is_coinbase,
        }
    }
}
