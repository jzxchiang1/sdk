use crate::commands::ledger::{get_icpts_from_args, transfer};
use crate::lib::environment::Environment;
use crate::lib::error::DfxResult;
use crate::lib::ledger_types::Memo;
use crate::lib::nns_types::account_identifier::AccountIdentifier;
use crate::lib::nns_types::icpts::{ICPTs, TRANSACTION_FEE};
use crate::lib::root_key::fetch_root_key_if_needed;
use crate::util::clap::validators::{e8s_validator, icpts_amount_validator, memo_validator};

use anyhow::anyhow;
use clap::Clap;
use std::str::FromStr;

/// Transfer ICP from the user to the destination AccountIdentifier
#[derive(Clap)]
pub struct TransferOpts {
    /// AccountIdentifier of transfer destination.
    to: String,

    /// ICPs to transfer to the destination AccountIdentifier
    /// Can be specified as a Decimal with the fractional portion up to 8 decimal places
    /// i.e. 100.012
    #[clap(long, validator(icpts_amount_validator))]
    amount: Option<String>,

    /// Specify ICP as a whole number, helpful for use in conjunction with `--e8s`
    #[clap(long, validator(e8s_validator), conflicts_with("amount"))]
    icp: Option<String>,

    /// Specify e8s as a whole number, helpful for use in conjunction with `--icp`
    #[clap(long, validator(e8s_validator), conflicts_with("amount"))]
    e8s: Option<String>,

    /// Specify a numeric memo for this transaction.
    #[clap(long, validator(memo_validator))]
    memo: String,

    /// Transaction fee, default is 10000 e8s.
    #[clap(long, validator(icpts_amount_validator))]
    fee: Option<String>,
}

pub async fn exec(env: &dyn Environment, opts: TransferOpts) -> DfxResult {
    let amount = get_icpts_from_args(opts.amount, opts.icp, opts.e8s)?;

    let fee = opts.fee.map_or(Ok(TRANSACTION_FEE), |v| {
        ICPTs::from_str(&v).map_err(|err| anyhow!(err))
    })?;

    // validated by memo_validator
    let memo = Memo(opts.memo.parse::<u64>().unwrap());

    let to = AccountIdentifier::from_str(&opts.to)
        .map_err(|err| anyhow!(err))?
        .to_address();
    let agent = env
        .get_agent()
        .ok_or_else(|| anyhow!("Cannot get HTTP client from environment."))?;

    fetch_root_key_if_needed(env).await?;

    let block_height = transfer(agent, memo, amount, fee, to).await?;

    println!("Transfer sent at BlockHeight: {}", block_height);

    Ok(())
}
