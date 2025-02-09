use crate::lib::environment::Environment;
use crate::lib::error::DfxResult;
use crate::lib::ledger_types::{AccountBalanceArgs, MAINNET_LEDGER_CANISTER_ID};
use crate::lib::nns_types::account_identifier::AccountIdentifier;
use crate::lib::nns_types::icpts::ICPTs;

use anyhow::anyhow;
use candid::{Decode, Encode};
use clap::Clap;
use std::str::FromStr;

const ACCOUNT_BALANCE_METHOD: &str = "account_balance_dfx";

/// Prints the account balance of the user
#[derive(Clap)]
pub struct BalanceOpts {
    /// Specifies an AccountIdentifier to get the balance of
    of: Option<String>,
}

pub async fn exec(env: &dyn Environment, opts: BalanceOpts) -> DfxResult {
    let sender = env
        .get_selected_identity_principal()
        .expect("Selected identity not instantiated.");
    let acc_id = opts
        .of
        .map_or_else(
            || Ok(AccountIdentifier::new(sender, None)),
            |v| AccountIdentifier::from_str(&v),
        )
        .map_err(|err| anyhow!(err))?;
    let agent = env
        .get_agent()
        .ok_or_else(|| anyhow!("Cannot get HTTP client from environment."))?;

    let result = agent
        .query(&MAINNET_LEDGER_CANISTER_ID, ACCOUNT_BALANCE_METHOD)
        .with_arg(Encode!(&AccountBalanceArgs {
            account: acc_id.to_string()
        })?)
        .call()
        .await?;

    let balance = Decode!(&result, ICPTs)?;

    println!("{}", balance);

    Ok(())
}
