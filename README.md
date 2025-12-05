# Fomowelll Canister

Rust canister that bridges “Alkanes”/Protostone inscriptions and Fomowell meme tokens while coordinating Bitcoin UTXOs on the Internet Computer.

## Project Layout
- `src/lib.rs`: canister entrypoint, timers, top-up/withdraw orchestration, HTTP calls, and address derivation.
- `src/alkanes/`: Protostone encoding helpers and in-canister storage for alkane records, UTXO ledger, and token whitelist.
- `src/psbt/`: PSBT builder, fee estimation, and transaction assembly utilities.
- `src/ic/`: wrappers around management canister APIs (Schnorr, Bitcoin, HTTP, etc.).
- `src/did/`: generated bindings for external canisters (Fomowell token ledger, fee-rate canister, BTC canisters).

## Public Methods (Candid)
- `topup_alkanes(txid: String)` (update): consume a recorded alkane deposit and transfer the mapped meme token to the caller.
- `get_address(address_type: String)` (query): return derived addresses for `alkanes_topup`, `alkanes_fund`, or `btc`.
- `get_btc_utxos(address: String)` (update): fetch BTC UTXOs for an address via the management canister.
- `set_owner_ic(new_owner: Principal)` (update): change the admin principal.
- `upload_alkanes(batch: Vec<AlkaneRecord>)` / `clear_alkanes()` (update): batch load or clear recorded alkane deposits and related state.
- `get_alkane(txid: String)` / `list_alkanes()` (query): read stored alkane records.
- `add_white_token_ic(token: String)` / `remove_white_token_ic(token: String)` (update) and `get_white_tokens_ic()` (query): manage the whitelist and mapped meme token ids.

Background tasks:
- Every hour: `gather_alkanes_utxo_timer` consolidates alkane UTXOs and builds Protostone transfer outputs.
- Every two hours: `check_withdraw_request` checks the last pending BTC tx and, when confirmed, dispatches queued withdraws.

## Build & Deploy
1) Install the wasm target: `rustup target add wasm32-unknown-unknown`.
2) Build with dfx: `dfx build`.
3) Deploy (local example): `dfx deploy fomowelll`.

`dfx.json` is configured to compile `src/lib.rs` as the candid interface. Management-canister HTTP/Bitcoin calls require cycles when running on the network.

## Development Notes
- The canister uses stable storage for alkane records, whitelist, token-id mappings, and the UTXO ledger (see `alkanes_storage.rs`).
- Constants such as `IC_BITCOIN_NETWORK` (Testnet), `SCHNORR_KEY_NAME`, and the external canister ids are defined in `src/lib.rs`; adjust them before deploying to a different environment.
- PSBT helpers expose `create_transaction_multi`, `calculate_fee_simple`, and related types for composing signed transactions with Schnorr/Taproot inputs.
