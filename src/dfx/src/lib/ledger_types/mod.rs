// DISCLAIMER:
// Do not modify this file arbitrarily.
// The contents are borrowed from:
// https://gitlab.com/dfinity-lab/public/ic/-/blob/master/rs/rosetta-api/ledger_canister/src/lib.rs

use crate::lib::nns_types::account_identifier::Subaccount;
use crate::lib::nns_types::icpts::ICPTs;
use crate::lib::nns_types::{account_identifier, icpts};
use candid::CandidType;
use ic_types::principal::Principal;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Id of the ledger canister on the IC.
pub const MAINNET_LEDGER_CANISTER_ID: Principal =
    Principal::from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x01, 0x01]);

pub const MAINNET_CYCLE_MINTER_CANISTER_ID: Principal =
    Principal::from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x01, 0x01]);

pub type AccountIdBlob = [u8; 32];

/// Arguments for the `transfer` call.
#[derive(CandidType)]
pub struct TransferArgs {
    pub memo: Memo,
    pub amount: ICPTs,
    pub fee: ICPTs,
    pub from_subaccount: Option<Subaccount>,
    pub to: AccountIdBlob,
    pub created_at_time: Option<TimeStamp>,
}

/// Result of the `transfer` call.
pub type TransferResult = Result<BlockHeight, TransferError>;

/// Error of the `transfer` call.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum TransferError {
    BadFee { expected_fee: ICPTs },
    InsufficientFunds { balance: ICPTs },
    TxTooOld { allowed_window_nanos: u64 },
    TxCreatedInFuture,
    TxDuplicate { duplicate_of: BlockHeight },
}

impl fmt::Display for TransferError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadFee { expected_fee } => {
                write!(f, "transaction fee should be {}", expected_fee)
            }
            Self::InsufficientFunds { balance } => {
                write!(
                    f,
                    "the debit account doesn't have enough funds to complete the transaction, current balance: {}",
                    balance
                )
            }
            Self::TxTooOld {
                allowed_window_nanos,
            } => write!(
                f,
                "transaction is older than {} seconds",
                allowed_window_nanos / 1_000_000_000
            ),
            Self::TxCreatedInFuture => write!(f, "transaction's created_at_time is in future"),
            Self::TxDuplicate { duplicate_of } => write!(
                f,
                "transaction is a duplicate of another transaction in block {}",
                duplicate_of
            ),
        }
    }
}

#[derive(CandidType, Deserialize)]
pub enum CyclesResponse {
    CanisterCreated(Principal),
    ToppedUp(()),
    Refunded(String, Option<BlockHeight>),
}

/// Position of a block in the chain. The first block has position 0.
pub type BlockHeight = u64;

#[derive(
    Serialize, Deserialize, CandidType, Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct Memo(pub u64);

impl Default for Memo {
    fn default() -> Memo {
        Memo(0)
    }
}

#[derive(CandidType)]
pub struct AccountBalanceArgs {
    pub account: String,
}

#[derive(CandidType)]
pub struct TimeStamp {
    pub timestamp_nanos: u64,
}

#[derive(CandidType)]
pub struct NotifyCanisterArgs {
    pub block_height: BlockHeight,
    pub max_fee: icpts::ICPTs,
    pub from_subaccount: Option<account_identifier::Subaccount>,
    pub to_canister: Principal,
    pub to_subaccount: Option<account_identifier::Subaccount>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ledger_canister_id() {
        assert_eq!(
            MAINNET_LEDGER_CANISTER_ID,
            Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap()
        );
    }

    #[test]
    fn test_cycle_minter_canister_id() {
        assert_eq!(
            MAINNET_CYCLE_MINTER_CANISTER_ID,
            Principal::from_text("rkp4c-7iaaa-aaaaa-aaaca-cai").unwrap()
        );
    }
}
