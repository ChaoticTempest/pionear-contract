near create-account v1bookdb.chaotictempest.near --masterAccount chaotictempest.near --initialBalance 1
 near deploy --accountId v1bookdb.chaotictempest.near --wasmFile earth/feedr/target/wasm32-unknown-unknown/release/feedr.wasm --initFunction new --initArgs '{}'