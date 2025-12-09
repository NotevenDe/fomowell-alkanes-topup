// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Deserialize, Principal, Encode, Decode};
use ic_cdk::api::call::CallResult as Result;

#[derive(CandidType, Deserialize)]
pub enum BitcoinNetwork {
  #[serde(rename="mainnet")]
  Mainnet,
  #[serde(rename="regtest")]
  Regtest,
  #[serde(rename="testnet")]
  Testnet,
}

#[derive(CandidType, Deserialize)]
pub struct InitArgs {
  pub runes_indexer: Principal,
  pub network: BitcoinNetwork,
  pub ckbtc_minter: Principal,
  pub ckbtc_ledger: Principal,
  pub timer_for_reveal_txn: u32,
}

#[derive(CandidType, Deserialize)]
pub enum Result_ { Ok, Err(String) }

#[derive(CandidType, Deserialize)]
pub enum Result1 { Ok(u64), Err(String) }

#[derive(CandidType, Deserialize)]
pub enum DepositResult { Ok(candid::Nat), Err(String) }

#[derive(CandidType, Deserialize)]
pub struct DepositRunesArgs {
  pub pid: Principal,
  pub utxo: String,
  pub divisibility: u8,
  pub amount: candid::Nat,
  pub receiver: String,
  pub rune_id: String,
}

#[derive(CandidType, Deserialize)]
pub enum Result2 { Ok(String), Err(String) }

#[derive(CandidType, Deserialize)]
pub struct BurnRuneArg {
  pub pid: Principal,
  pub value: candid::Nat,
  pub memo: String,
  pub rune_name: String,
  pub subaccount: Option<serde_bytes::ByteBuf>,
  pub address: String,
}

#[derive(CandidType, Deserialize)]
pub struct StatusRequest {
  pub memory_size: bool,
  pub cycles: bool,
  pub heap_memory_size: bool,
}

#[derive(CandidType, Deserialize)]
pub struct StatusResponse {
  pub memory_size: Option<u64>,
  pub cycles: Option<u64>,
  pub heap_memory_size: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct DepositUtxoArgs {
  pub pid: Principal,
  pub value: candid::Nat,
  pub block_hash: String,
  pub block_time: u64,
  pub txid: String,
  pub vout: u32,
  pub address: String,
  pub confirmed: bool,
  pub block_height: u32,
}

#[derive(CandidType, Deserialize)]
pub struct EtchingArgs {
  pub cap: candid::Nat,
  pub height: Option<(u64,u64,)>,
  pub turbo: bool,
  pub premine: candid::Nat,
  pub rune: String,
  pub divisibility: u8,
  pub offset: Option<(u64,u64,)>,
  pub fee_rate: Option<u64>,
  pub amount: candid::Nat,
  pub symbol: u32,
}

#[derive(CandidType, Deserialize)]
pub struct OrdinalsTerms {
  pub cap: candid::Nat,
  pub height: (Option<u64>,Option<u64>,),
  pub offset: (Option<u64>,Option<u64>,),
  pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct LogoParams { pub content_type: String, pub content_base64: String }

#[derive(CandidType, Deserialize)]
pub struct EtchingArgs1 {
  pub terms: Option<OrdinalsTerms>,
  pub turbo: bool,
  pub premine: Option<candid::Nat>,
  pub logo: Option<LogoParams>,
  pub rune_name: String,
  pub divisibility: Option<u8>,
  pub premine_receiver: String,
  pub symbol: Option<String>,
}

#[derive(CandidType, Deserialize)]
pub enum Result3 { Ok(u64,String,), Err(String) }

#[derive(CandidType, Deserialize)]
pub struct EtchingArgs2 {
  pub terms: Option<OrdinalsTerms>,
  pub turbo: bool,
  pub premine: Option<candid::Nat>,
  pub logo: Option<LogoParams>,
  pub rune_name: String,
  pub divisibility: Option<u8>,
  pub symbol: Option<String>,
}

#[derive(CandidType, Deserialize)]
pub struct Account {
  pub owner: Principal,
  pub subaccount: Option<serde_bytes::ByteBuf>,
}

#[derive(CandidType, Deserialize)]
pub struct Addresses {
  pub icrc1_string: String,
  pub account_identifier: serde_bytes::ByteBuf,
  pub icrc1: Account,
  pub ckbtc_mint: String,
  pub bitcoin: String,
  pub account_identifier_string: String,
}

#[derive(CandidType, Deserialize)]
pub struct Terms {
  pub cap: Option<candid::Nat>,
  pub height: (Option<u64>,Option<u64>,),
  pub offset: (Option<u64>,Option<u64>,),
  pub amount: Option<candid::Nat>,
}

#[derive(CandidType, Deserialize)]
pub struct RuneInfoWrapper {
  pub id: u64,
  pub confirmations: u32,
  pub mints: candid::Nat,
  pub terms: Option<Terms>,
  pub etching: String,
  pub turbo: bool,
  pub token_id: Option<String>,
  pub premine: candid::Nat,
  pub rune_utxo_vout: u32,
  pub rune_utxo: Option<String>,
  pub divisibility: u8,
  pub rune_balance: candid::Nat,
  pub spaced_rune: String,
  pub number: u64,
  pub timestamp: u64,
  pub block: u64,
  pub ledger_canister: Option<Principal>,
  pub burned: candid::Nat,
  pub rune_id: String,
  pub mint_address: String,
  pub symbol: Option<String>,
}

#[derive(CandidType, Deserialize)]
pub struct PagedResponse {
  pub total: u64,
  pub offset: u64,
  pub limit: u64,
  pub items: Vec<RuneInfoWrapper>,
}

#[derive(CandidType, Deserialize)]
pub struct RuneId { pub tx: u32, pub block: u64 }

#[derive(CandidType, Deserialize)]
pub enum TokenType { Rune(RuneId), Bitcoin }

#[derive(CandidType, Deserialize)]
pub struct Outpoint { pub txid: serde_bytes::ByteBuf, pub vout: u32 }

#[derive(CandidType, Deserialize)]
pub struct Utxo { pub height: u32, pub value: u64, pub outpoint: Outpoint }

#[derive(CandidType, Deserialize)]
pub struct GetUtxosResponse {
  pub next_page: Option<serde_bytes::ByteBuf>,
  pub tip_height: u32,
  pub tip_block_hash: serde_bytes::ByteBuf,
  pub utxos: Vec<Utxo>,
}

#[derive(CandidType, Deserialize)]
pub struct ImportRuneArg {
  pub confirmations: u32,
  pub mints: candid::Nat,
  pub terms: Option<Terms>,
  pub etching: String,
  pub turbo: bool,
  pub token_id: Option<String>,
  pub premine: candid::Nat,
  pub rune_utxo_vout: u32,
  pub rune_utxo: Option<String>,
  pub divisibility: u8,
  pub rune_balance: candid::Nat,
  pub spaced_rune: String,
  pub number: u64,
  pub timestamp: u64,
  pub block: u64,
  pub ledger_canister: Option<Principal>,
  pub burned: candid::Nat,
  pub rune_id: String,
  pub mint_address: String,
  pub symbol: Option<String>,
}

#[derive(CandidType, Deserialize)]
pub struct DepositWrapper {
  pub id: u64,
  pub pid: Principal,
  pub deposit_at: Option<u64>,
  pub value: candid::Nat,
  pub block_hash: String,
  pub block_time: u64,
  pub txid: String,
  pub vout: u32,
  pub mint_at: Option<u64>,
  pub collection_at: Option<u64>,
  pub created_at: u64,
  pub address: String,
  pub confirmed: bool,
  pub block_height: u32,
}

#[derive(CandidType, Deserialize)]
pub struct DepositExtWrapper {
  pub id: u64,
  pub pid: Principal,
  pub value: candid::Nat,
  pub txid: String,
  pub mint_at: Option<u64>,
  pub created_at: u64,
  pub confirmed: bool,
}

#[derive(CandidType, Deserialize)]
pub struct DepositRunesWrapper {
  pub pid: Principal,
  pub confirmations: u64,
  pub deposit_at: u64,
  pub utxo: String,
  pub collection_at: Option<u64>,
  pub divisibility: u8,
  pub address: String,
  pub amount: candid::Nat,
  pub rune_id: String,
}

#[derive(CandidType, Deserialize)]
pub struct WithdrawRuneWrapper {
  pub id: u64,
  pub fee: candid::Nat,
  pub pid: Principal,
  pub value: candid::Nat,
  pub block_hash: Option<String>,
  pub block_time: Option<u64>,
  pub memo: String,
  pub txid: Option<String>,
  pub created_at: u64,
  pub broadcast_at: Option<u64>,
  pub address: String,
  pub confirmed: bool,
  pub completed_at: Option<u64>,
  pub block_height: Option<u32>,
  pub rune_id: String,
}

#[derive(CandidType, Deserialize)]
pub struct MintAddress {
  pub mint_btc_address: String,
  pub platform_icrc1_subaccount: Option<serde_bytes::ByteBuf>,
  pub index: u64,
  pub mint_icrc1_subaccount: Option<serde_bytes::ByteBuf>,
}

#[derive(CandidType, Deserialize)]
pub struct RunicUtxo { pub balance: candid::Nat, pub utxo: Utxo }

#[derive(CandidType, Deserialize)]
pub enum UtxoStatus {
  ValueTooSmall(Utxo),
  Tainted(Utxo),
  Minted{ minted_amount: u64, block_index: u64, utxo: Utxo },
  Checked(Utxo),
}

#[derive(CandidType, Deserialize)]
pub struct RuneEntry {
  pub confirmations: u32,
  pub mints: candid::Nat,
  pub terms: Option<Terms>,
  pub etching: String,
  pub turbo: bool,
  pub premine: candid::Nat,
  pub divisibility: u8,
  pub spaced_rune: String,
  pub number: u64,
  pub timestamp: u64,
  pub block: u64,
  pub burned: candid::Nat,
  pub rune_id: String,
  pub symbol: Option<String>,
}

#[derive(CandidType, Deserialize)]
pub struct WalletReceiveResult { pub accepted: u64 }

#[derive(CandidType, Deserialize)]
pub enum WithdrawalType {
  Rune{
    to: String,
    fee_per_vbytes: Option<u64>,
    runeid: RuneId,
    amount: candid::Nat,
  },
  Bitcoin{ to: String, fee_per_vbytes: Option<u64>, amount: u64 },
}

#[derive(CandidType, Deserialize)]
pub enum SubmittedTxidType { Bitcoin{ txid: String } }

#[derive(CandidType, Deserialize)]
pub struct WithdrawRuneArgs {
  pub pid: Principal,
  pub value: candid::Nat,
  pub memo: String,
  pub rune_name: String,
  pub address: String,
}

pub struct Service(pub Principal);
impl Service {
  pub async fn get_candid_interface_tmp_hack(&self) -> Result<(String,)> {
    ic_cdk::call(self.0, "__get_candid_interface_tmp_hack", ()).await
  }
  pub async fn add_admin(&self, arg0: Principal) -> Result<(Result_,)> {
    ic_cdk::call(self.0, "add_admin", (arg0,)).await
  }
  pub async fn add_rune(&self, arg0: String) -> Result<(Result1,)> {
    ic_cdk::call(self.0, "add_rune", (arg0,)).await
  }
  pub async fn add_used_tx_id(&self, arg0: String, arg1: u64) -> Result<
    (Result_,)
  > { ic_cdk::call(self.0, "add_used_tx_id", (arg0,arg1,)).await }
  pub async fn admin_delete_rune(&self, arg0: u64) -> Result<(Result_,)> {
    ic_cdk::call(self.0, "admin_delete_rune", (arg0,)).await
  }
  pub async fn admin_deposit_rune(
    &self,
    arg0: Principal,
    arg1: String,
    arg2: candid::Nat,
    arg3: String,
  ) -> Result<(DepositResult,)> {
    ic_cdk::call(self.0, "admin_deposit_rune", (arg0,arg1,arg2,arg3,)).await
  }
  pub async fn admin_deposit_runes(&self, arg0: DepositRunesArgs) -> Result<
    (Result2,)
  > { ic_cdk::call(self.0, "admin_deposit_runes", (arg0,)).await }
  pub async fn burn_rune(&self, arg0: BurnRuneArg) -> Result<(Result1,)> {
    ic_cdk::call(self.0, "burn_rune", (arg0,)).await
  }
  pub async fn canister_get_status(&self, arg0: StatusRequest) -> Result<
    (StatusResponse,)
  > { ic_cdk::call(self.0, "canister_get_status", (arg0,)).await }
  pub async fn collect(&self, arg0: DepositUtxoArgs) -> Result<(Result1,)> {
    ic_cdk::call(self.0, "collect", (arg0,)).await
  }
  pub async fn collect_runes(&self, arg0: String) -> Result<(Result2,)> {
    ic_cdk::call(self.0, "collect_runes", (arg0,)).await
  }
  pub async fn confirm_and_convert_ckbtc(&self) -> Result<(u64,)> {
    ic_cdk::call(self.0, "confirm_and_convert_ckbtc", ()).await
  }
  pub async fn deposit(&self, arg0: DepositUtxoArgs) -> Result<(Result1,)> {
    ic_cdk::call(self.0, "deposit", (arg0,)).await
  }
  pub async fn deposit_by_admin(&self, arg0: DepositUtxoArgs) -> Result<
    (Result1,)
  > { ic_cdk::call(self.0, "deposit_by_admin", (arg0,)).await }
  pub async fn deposit_by_ext(
    &self,
    arg0: Principal,
    arg1: String,
    arg2: candid::Nat,
  ) -> Result<(Result1,)> {
    ic_cdk::call(self.0, "deposit_by_ext", (arg0,arg1,arg2,)).await
  }
  pub async fn deposit_runes(&self, arg0: DepositRunesArgs) -> Result<
    (Result2,)
  > { ic_cdk::call(self.0, "deposit_runes", (arg0,)).await }
  pub async fn etch_rune(&self, arg0: EtchingArgs) -> Result<(String,String,)> {
    ic_cdk::call(self.0, "etch_rune", (arg0,)).await
  }
  pub async fn etch_rune_v_1(&self, arg0: EtchingArgs1) -> Result<(Result3,)> {
    ic_cdk::call(self.0, "etch_rune_v1", (arg0,)).await
  }
  pub async fn etch_rune_v_2(&self, arg0: EtchingArgs2) -> Result<(Result3,)> {
    ic_cdk::call(self.0, "etch_rune_v2", (arg0,)).await
  }
  pub async fn etch_rune_with_logo(
    &self,
    arg0: EtchingArgs,
    arg1: LogoParams,
  ) -> Result<(String,String,)> {
    ic_cdk::call(self.0, "etch_rune_with_logo", (arg0,arg1,)).await
  }
  pub async fn generate_transfer_ticket_v_2(
    &self,
    arg0: String,
    arg1: String,
    arg2: candid::Nat,
    arg3: Principal,
  ) -> Result<(Result2,)> {
    ic_cdk::call(self.0, "generate_transfer_ticket_v2", (
      arg0,arg1,arg2,arg3,
    )).await
  }
  pub async fn get_btc_balance(&self) -> Result<(u64,)> {
    ic_cdk::call(self.0, "get_btc_balance", ()).await
  }
  pub async fn get_deposit_address_for_bitcoin(&self) -> Result<(String,)> {
    ic_cdk::call(self.0, "get_deposit_address_for_bitcoin", ()).await
  }
  pub async fn get_deposit_address_for_ckbtc(&self) -> Result<(String,)> {
    ic_cdk::call(self.0, "get_deposit_address_for_ckbtc", ()).await
  }
  pub async fn get_deposit_addresses(&self) -> Result<(Addresses,)> {
    ic_cdk::call(self.0, "get_deposit_addresses", ()).await
  }
  pub async fn get_deposit_runes_address(&self) -> Result<(Result2,)> {
    ic_cdk::call(self.0, "get_deposit_runes_address", ()).await
  }
  pub async fn get_deposit_runes_address_by_pid(
    &self,
    arg0: Principal,
  ) -> Result<(Result2,)> {
    ic_cdk::call(self.0, "get_deposit_runes_address_by_pid", (arg0,)).await
  }
  pub async fn get_estimated_cbktc_conversion_fee(&self) -> Result<(u64,)> {
    ic_cdk::call(self.0, "get_estimated_cbktc_conversion_fee", ()).await
  }
  pub async fn get_fast_btc_address(
    &self,
    arg0: Option<serde_bytes::ByteBuf>,
  ) -> Result<(Result2,)> {
    ic_cdk::call(self.0, "get_fast_btc_address", (arg0,)).await
  }
  pub async fn get_mint_rune_by_id(&self, arg0: u64) -> Result<
    (Option<RuneInfoWrapper>,)
  > { ic_cdk::call(self.0, "get_mint_rune_by_id", (arg0,)).await }
  pub async fn get_mint_rune_by_name(&self, arg0: String) -> Result<
    (Option<RuneInfoWrapper>,)
  > { ic_cdk::call(self.0, "get_mint_rune_by_name", (arg0,)).await }
  pub async fn get_rune_list(&self, arg0: u64, arg1: u64) -> Result<
    (PagedResponse,)
  > { ic_cdk::call(self.0, "get_rune_list", (arg0,arg1,)).await }
  pub async fn get_user_balances(&self) -> Result<
    (Vec<(TokenType,candid::Nat,)>,)
  > { ic_cdk::call(self.0, "get_user_balances", ()).await }
  pub async fn get_utxos_of_update(&self, arg0: String) -> Result<
    (GetUtxosResponse,)
  > { ic_cdk::call(self.0, "get_utxos_of_update", (arg0,)).await }
  pub async fn import_rune(&self, arg0: ImportRuneArg) -> Result<(Result1,)> {
    ic_cdk::call(self.0, "import_rune", (arg0,)).await
  }
  pub async fn query_conversion_status(&self, arg0: u64) -> Result<(String,)> {
    ic_cdk::call(self.0, "query_conversion_status", (arg0,)).await
  }
  pub async fn query_list_deposits_paginated(
    &self,
    arg0: u64,
    arg1: u64,
  ) -> Result<(Vec<DepositWrapper>,)> {
    ic_cdk::call(self.0, "query_list_deposits_paginated", (arg0,arg1,)).await
  }
  pub async fn query_list_ext_deposits_paginated(
    &self,
    arg0: u64,
    arg1: u64,
  ) -> Result<(Vec<DepositExtWrapper>,)> {
    ic_cdk::call(self.0, "query_list_ext_deposits_paginated", (
      arg0,arg1,
    )).await
  }
  pub async fn query_list_runes_deposits_paginated(
    &self,
    arg0: u64,
    arg1: u64,
  ) -> Result<(Vec<DepositRunesWrapper>,)> {
    ic_cdk::call(self.0, "query_list_runes_deposits_paginated", (
      arg0,arg1,
    )).await
  }
  pub async fn query_list_withdraws_paginated(
    &self,
    arg0: u64,
    arg1: u64,
  ) -> Result<(Vec<WithdrawRuneWrapper>,)> {
    ic_cdk::call(self.0, "query_list_withdraws_paginated", (arg0,arg1,)).await
  }
  pub async fn query_runes_deposit_address_paginated(
    &self,
    arg0: u64,
    arg1: u64,
  ) -> Result<(Vec<(Principal,u64,String,)>,)> {
    ic_cdk::call(self.0, "query_runes_deposit_address_paginated", (
      arg0,arg1,
    )).await
  }
  pub async fn query_user_bitcoin_utxos(&self, arg0: String) -> Result<
    (Vec<Utxo>,)
  > { ic_cdk::call(self.0, "query_user_bitcoin_utxos", (arg0,)).await }
  pub async fn query_user_ckbtc_address_paginated(
    &self,
    arg0: u64,
    arg1: u64,
  ) -> Result<(Vec<(Principal,MintAddress,)>,u64,Option<u64>,)> {
    ic_cdk::call(self.0, "query_user_ckbtc_address_paginated", (
      arg0,arg1,
    )).await
  }
  pub async fn query_user_runic_utxos(
    &self,
    arg0: String,
    arg1: RuneId,
  ) -> Result<(Vec<RunicUtxo>,)> {
    ic_cdk::call(self.0, "query_user_runic_utxos", (arg0,arg1,)).await
  }
  pub async fn record_user_runic_utxos(
    &self,
    arg0: String,
    arg1: RuneId,
    arg2: Vec<RunicUtxo>,
  ) -> Result<(Result_,)> {
    ic_cdk::call(self.0, "record_user_runic_utxos", (arg0,arg1,arg2,)).await
  }
  pub async fn remove_admin(&self, arg0: Principal) -> Result<(Result_,)> {
    ic_cdk::call(self.0, "remove_admin", (arg0,)).await
  }
  pub async fn reset_user_runes_addresses(&self) -> Result<(Result_,)> {
    ic_cdk::call(self.0, "reset_user_runes_addresses", ()).await
  }
  pub async fn retry_deposit(&self, arg0: u64) -> Result<(Result1,)> {
    ic_cdk::call(self.0, "retry_deposit", (arg0,)).await
  }
  pub async fn transfer_token(
    &self,
    arg0: String,
    arg1: Principal,
    arg2: candid::Nat,
  ) -> Result<(Result2,)> {
    ic_cdk::call(self.0, "transfer_token", (arg0,arg1,arg2,)).await
  }
  pub async fn update_fast_btc_balance(&self) -> Result<(Vec<UtxoStatus>,)> {
    ic_cdk::call(self.0, "update_fast_btc_balance", ()).await
  }
  pub async fn update_rune_balance(
    &self,
    arg0: u64,
    arg1: String,
    arg2: u32,
    arg3: candid::Nat,
  ) -> Result<(Result_,)> {
    ic_cdk::call(self.0, "update_rune_balance", (arg0,arg1,arg2,arg3,)).await
  }
  pub async fn update_rune_from_indexer(&self, arg0: String) -> Result<
    (Option<RuneEntry>,)
  > { ic_cdk::call(self.0, "update_rune_from_indexer", (arg0,)).await }
  pub async fn update_rune_ledger_from_indexer(&self, arg0: String) -> Result<
    (Option<Principal>,)
  > { ic_cdk::call(self.0, "update_rune_ledger_from_indexer", (arg0,)).await }
  pub async fn wallet_balance(&self) -> Result<(candid::Nat,)> {
    ic_cdk::call(self.0, "wallet_balance", ()).await
  }
  pub async fn wallet_receive(&self) -> Result<(WalletReceiveResult,)> {
    ic_cdk::call(self.0, "wallet_receive", ()).await
  }
  pub async fn withdraw(&self, arg0: WithdrawalType) -> Result<
    (SubmittedTxidType,)
  > { ic_cdk::call(self.0, "withdraw", (arg0,)).await }
  pub async fn withdraw_rune(&self, arg0: WithdrawRuneArgs) -> Result<
    (Result1,)
  > { ic_cdk::call(self.0, "withdraw_rune", (arg0,)).await }
  pub async fn withdraw_rune_v_2(&self, arg0: WithdrawRuneArgs) -> Result<
    (Result1,)
  > { ic_cdk::call(self.0, "withdraw_rune_v2", (arg0,)).await }
}
