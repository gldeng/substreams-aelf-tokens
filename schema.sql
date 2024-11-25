CREATE TABLE IF NOT EXISTS balances (
   "contract" TEXT NOT NULL,
   "symbol" TEXT NOT NULL,
   "owner" TEXT NOT NULL,
   "balance" NUMERIC NOT NULL,
   "transaction" TEXT NOT NULL,
   "block_num" INT NOT NULL,
   "timestamp" TEXT NOT NULL,
   PRIMARY KEY ("contract", "symbol", "owner")
);