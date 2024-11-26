use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams::scalar::BigInt;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_entity_change::pb::entity::EntityChanges;
use crate::pb::sf::substreams::aelf::token::v1::{BalanceUpdates, Transfers};
use std::str::FromStr;

#[substreams::handlers::map]
pub fn db_out(clock: Clock, balance_updates: BalanceUpdates) -> Result<DatabaseChanges, Error> {
    let mut tables = substreams_database_change::tables::Tables::new();
    let block_num = clock.clone().number.to_string();
    let timestamp = clock.clone().timestamp.unwrap().seconds.to_string();

    let mut ordinal = 0;
    for balance_update in balance_updates.balance_updates {
        tables.create_row("balance_updates", [
            ("contract", (&balance_update).contract.to_string()),
            ("symbol", (&balance_update).symbol.to_string()),
            ("owner", (&balance_update).owner.to_string()),
            ("block_num", block_num.to_string()),
            ("ordinal", ordinal.to_string()),
        ])
            .set("contract", balance_update.contract)
            .set("symbol", balance_update.symbol)
            .set("owner", balance_update.owner)
            .set("balance", balance_update.new_balance)
            .set("transaction", balance_update.transaction)
            .set("call_path", balance_update.call_path)
            .set("block_num", &block_num)
            .set("ordinal", ordinal.to_string())
            .set("timestamp", &timestamp);
        ordinal += 1;
    }

    Ok(tables.to_database_changes())
}

#[substreams::handlers::map]
pub fn graph_out(transfers: Transfers) -> Result<EntityChanges, Error> {
    let mut tables = substreams_entity_change::tables::Tables::new();
    let block_num = transfers.clock.clone().unwrap_or_default().number;
    let block_hash = transfers.clock.clone().unwrap_or_default().id.to_string();
    let timestamp = transfers.clock.clone().unwrap_or_default().timestamp.unwrap_or_default();

    let mut ordinal = 0;
    for t in transfers.transfers {
        tables.create_row("Transfer", format!("{}:{}", block_hash, ordinal))
            .set("contract", t.contract)
            .set("symbol", t.symbol)
            .set("from", t.from)
            .set("to", t.to)
            .set("amount", BigInt::from_str(&t.amount).unwrap_or_else(|_| BigInt::one().neg()))
            .set("memo", t.memo)
            .set("blockNumber", BigInt::from(block_num))
            .set("ordinal", BigInt::from(ordinal))
            .set("transaction", t.tx_id)
            .set("callPath", t.call_path)
            .set("timestamp", timestamp.to_string());
        ordinal += 1;
    }

    Ok(tables.to_entity_changes())
}
