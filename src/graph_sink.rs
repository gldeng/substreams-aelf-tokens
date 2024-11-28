use crate::pb::sf::substreams::aelf::token::v1::{BalanceUpdates, Transfers};
use std::str::FromStr;
use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams::prelude::{BigInt, StoreGetBigInt};
use substreams::store::StoreGet;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;

fn graph_write_tranfers(tables: &mut Tables, clock: &Clock, transfers: Transfers) {
    let Clock {
        id: block_hash,
        number: block_num,
        timestamp,
    } = clock.clone();
    let timestamp = timestamp.unwrap_or_default();
    let mut ordinal = 0;
    for t in transfers.transfers {
        tables
            .create_row("Transfer", format!("{}:{}", block_hash, ordinal))
            .set("contract", t.contract)
            .set("symbol", t.symbol)
            .set("from", t.from)
            .set("to", t.to)
            .set(
                "amount",
                BigInt::from_str(&t.amount).unwrap_or_else(|_| BigInt::one().neg()),
            )
            .set("memo", t.memo)
            .set("blockNumber", BigInt::from(block_num))
            .set("blockHash", block_hash.to_string())
            .set("ordinal", BigInt::from(ordinal))
            .set("transaction", t.tx_id)
            .set("callPath", t.call_path)
            .set("timestamp", timestamp.to_string());
        ordinal += 1;
    }
}

fn graph_write_balances(
    tables: &mut Tables,
    clock: &Clock,
    balance_updates: BalanceUpdates,
    balances_store: &StoreGetBigInt,
) {
    let Clock {
        id: block_hash,
        number: block_num,
        timestamp,
    } = clock.clone();
    let timestamp = timestamp.unwrap_or_default();

    let mut ordinal = 0;
    for balance_update in balance_updates.balance_updates {
        let key = crate::balance::get_balance_key(&balance_update);
        let balance_row = match balances_store.get_at(ordinal - 1, key.clone()) {
            Some(_) => tables.update_row("Balance", key),
            _ => tables.create_row("Balance", key),
        };

        balance_row
            .set("contract", balance_update.contract.clone())
            .set("symbol", balance_update.symbol.clone())
            .set("owner", balance_update.owner.clone())
            .set("balance", BigInt::from_str(&balance_update.new_balance).unwrap_or_else(|_|BigInt::one().neg()))
            .set("transaction", balance_update.transaction.clone())
            .set("callPath", balance_update.call_path.clone())
            .set("blockNumber", BigInt::from(block_num))
            .set("blockHash", block_hash.clone())
            .set("timestamp", timestamp.to_string());

        tables
            .create_row(
                "BalanceUpdate",
                format!("{}:{}",  block_hash, ordinal),
            )
            .set("contract", balance_update.contract)
            .set("symbol", balance_update.symbol)
            .set("owner", balance_update.owner)
            .set("balance", BigInt::from_str(&balance_update.new_balance).unwrap_or_else(|_|BigInt::one().neg()))
            .set("transaction", balance_update.transaction)
            .set("callPath", balance_update.call_path)
            .set("blockNumber", BigInt::from(block_num))
            .set("blockHash", block_hash.to_string())
            .set("ordinal", BigInt::from(ordinal))
            .set("timestamp", timestamp.to_string());
        ordinal += 1;
    }
}

#[substreams::handlers::map]
pub fn graph_out(
    clock: Clock,
    transfers: Transfers,
    balance_updates: BalanceUpdates,
    balances_store: StoreGetBigInt,
) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();
    graph_write_tranfers(&mut tables, &clock, transfers);
    graph_write_balances(&mut tables, &clock, balance_updates, &balances_store);
    Ok(tables.to_entity_changes())
}
