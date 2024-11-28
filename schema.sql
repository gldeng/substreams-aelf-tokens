CREATE TABLE IF NOT EXISTS balances (
   "contract" TEXT NOT NULL,
   "symbol" TEXT NOT NULL,
   "owner" TEXT NOT NULL,
   "balance" BIGINT NOT NULL,
   "transaction" TEXT NOT NULL,
   "call_path" TEXT NOT NULL,
   "block_num" INT NOT NULL,
   "timestamp" TEXT NOT NULL,
   PRIMARY KEY ("contract", "symbol", "owner")
);

CREATE TABLE IF NOT EXISTS balance_updates (
   "contract" TEXT NOT NULL,
   "symbol" TEXT NOT NULL,
   "owner" TEXT NOT NULL,
   "balance" BIGINT NOT NULL,
   "transaction" TEXT NOT NULL,
   "call_path" TEXT NOT NULL,
   "block_num" INT NOT NULL,
   "timestamp" TEXT NOT NULL,
   "ordinal" INT NOT NULL,
   PRIMARY KEY ("contract", "symbol", "owner", "block_num", "ordinal")
);