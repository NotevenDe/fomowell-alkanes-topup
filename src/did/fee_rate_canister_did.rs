// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Decode, Deserialize, Encode, Principal};
use ic_cdk::api::call::CallResult as Result;

#[derive(CandidType, Deserialize)]
pub enum Result_ {
    Ok,
    Err(String),
}

#[derive(CandidType, Deserialize)]
pub enum TxOutputType {
    #[serde(rename = "P2WPKH")]
    P2Wpkh,
    OpReturn(u64),
    #[serde(rename = "P2SH")]
    P2Sh,
    #[serde(rename = "P2TR")]
    P2Tr,
}

#[derive(CandidType, Deserialize)]
pub struct EstimateMinTxFeeArgs {
    pub input_types: Vec<TxOutputType>,
    pub pool_address: String,
    pub output_types: Vec<TxOutputType>,
}

#[derive(CandidType, Deserialize)]
pub enum Result1 {
    Ok(u64),
    Err(String),
}

#[derive(CandidType, Deserialize)]
pub struct FromUserRecord {
    pub user_id: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct FromCanisterRecord {
    pub canister_version: Option<u64>,
    pub canister_id: Principal,
}

#[derive(CandidType, Deserialize)]
pub enum CanisterChangeOrigin {
    #[serde(rename = "from_user")]
    FromUser(FromUserRecord),
    #[serde(rename = "from_canister")]
    FromCanister(FromCanisterRecord),
}

#[derive(CandidType, Deserialize)]
pub struct CreationRecord {
    pub controllers: Vec<Principal>,
}

#[derive(CandidType, Deserialize)]
pub enum CodeDeploymentMode {
    #[serde(rename = "reinstall")]
    Reinstall,
    #[serde(rename = "upgrade")]
    Upgrade,
    #[serde(rename = "install")]
    Install,
}

#[derive(CandidType, Deserialize)]
pub struct CodeDeploymentRecord {
    pub mode: CodeDeploymentMode,
    pub module_hash: serde_bytes::ByteBuf,
}

#[derive(CandidType, Deserialize)]
pub struct LoadSnapshotRecord {
    pub canister_version: u64,
    pub taken_at_timestamp: u64,
    pub snapshot_id: serde_bytes::ByteBuf,
}

#[derive(CandidType, Deserialize)]
pub enum CanisterChangeDetails {
    #[serde(rename = "creation")]
    Creation(CreationRecord),
    #[serde(rename = "code_deployment")]
    CodeDeployment(CodeDeploymentRecord),
    #[serde(rename = "load_snapshot")]
    LoadSnapshot(LoadSnapshotRecord),
    #[serde(rename = "controllers_change")]
    ControllersChange(CreationRecord),
    #[serde(rename = "code_uninstall")]
    CodeUninstall,
}

#[derive(CandidType, Deserialize)]
pub struct CanisterChange {
    pub timestamp_nanos: u64,
    pub canister_version: u64,
    pub origin: CanisterChangeOrigin,
    pub details: CanisterChangeDetails,
}

#[derive(CandidType, Deserialize)]
pub struct CanisterInfoResponse {
    pub controllers: Vec<Principal>,
    pub module_hash: Option<serde_bytes::ByteBuf>,
    pub recent_changes: Vec<CanisterChange>,
    pub total_num_changes: u64,
}

#[derive(CandidType, Deserialize)]
pub enum Result2 {
    Ok(CanisterInfoResponse),
    Err(String),
}

#[derive(CandidType, Deserialize)]
pub struct ExchangePool {
    pub exchange_id: String,
    pub pool_address: String,
    pub pool_key: String,
}

#[derive(CandidType, Deserialize)]
pub enum GetFailedInvokeLogArgs {
    All,
    ByTxid(String),
    ByAddress(String),
}

#[derive(CandidType, Deserialize)]
pub struct RollbackStepLogView {
    pub result: Result_,
    pub exchange_id: String,
    pub txid: String,
    pub rollback_time: String,
    pub maybe_return_time: Option<String>,
    pub pool_address: String,
}

#[derive(CandidType, Deserialize)]
pub enum Result3 {
    Ok(String),
    Err(String),
}

#[derive(CandidType, Deserialize)]
pub struct ExecutionStepLogView {
    pub result: Result3,
    pub exchange_id: String,
    pub maybe_return_time: Option<String>,
    pub calling_method: String,
    pub calling_args: String,
    pub pool_address: String,
    pub calling_time: String,
}

#[derive(CandidType, Deserialize)]
pub struct InvokeLogView {
    pub invoke_args: String,
    pub invoke_time: String,
    pub finalized_time: Option<String>,
    pub rollback_steps: Vec<RollbackStepLogView>,
    pub confirmed_time: Option<String>,
    pub execution_steps: Vec<ExecutionStepLogView>,
    pub processing_result: Result3,
    pub broadcasted_time: Option<String>,
}

#[derive(CandidType, Deserialize)]
pub struct CoinBalance {
    pub id: String,
    pub value: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct InputCoin {
    pub coin: CoinBalance,
    pub from: String,
}

#[derive(CandidType, Deserialize)]
pub struct OutputCoin {
    pub to: String,
    pub coin: CoinBalance,
}

#[derive(CandidType, Deserialize)]
pub struct Intention {
    pub input_coins: Vec<InputCoin>,
    pub output_coins: Vec<OutputCoin>,
    pub action: String,
    pub exchange_id: String,
    pub pool_utxo_spend: Vec<String>,
    pub action_params: String,
    pub nonce: u64,
    pub pool_utxo_receive: Vec<String>,
    pub pool_address: String,
}

#[derive(CandidType, Deserialize)]
pub struct IntentionSet {
    pub initiator_address: String,
    pub intentions: Vec<Intention>,
}

#[derive(CandidType, Deserialize)]
pub struct InvokeArgs {
    pub intention_set: IntentionSet,
    pub psbt_hex: String,
}

#[derive(CandidType, Deserialize)]
pub struct MempoolTxFeeRateView {
    pub low: u64,
    pub high: u64,
    pub update_time: String,
    pub medium: u64,
}

#[derive(CandidType, Deserialize)]
pub struct BlockBasic {
    pub block_hash: String,
    pub block_height: u32,
}

#[derive(CandidType, Deserialize)]
pub struct ReceivedBlockView {
    pub processing_results: Vec<String>,
    pub block_basic: BlockBasic,
    pub txids: Vec<String>,
    pub block_time: String,
    pub received_time: String,
}

#[derive(CandidType, Deserialize)]
pub enum ExchangeStatus {
    Active,
    Halted { txid: String },
}

#[derive(CandidType, Deserialize)]
pub struct ExchangeView {
    pub status: ExchangeStatus,
    pub exchange_id: String,
    pub name: String,
    pub canister_id: Principal,
    pub description: String,
}

#[derive(CandidType, Deserialize)]
pub struct RejectedTxView {
    pub rollback_results: Vec<String>,
    pub txid: String,
    pub received_time: String,
    pub reason: String,
}

#[derive(CandidType, Deserialize)]
pub enum BitcoinNetwork {
    #[serde(rename = "mainnet")]
    Mainnet,
    #[serde(rename = "regtest")]
    Regtest,
    #[serde(rename = "testnet")]
    Testnet,
}

#[derive(CandidType, Deserialize)]
pub struct OrchestratorSettings {
    pub max_input_count_of_psbt: u32,
    pub min_tx_confirmations: u32,
    pub mempool_connector_principal: Principal,
    pub max_unconfirmed_tx_count_in_pool: u32,
    pub min_btc_amount_for_utxo: u64,
    pub rune_indexer_principal: Principal,
    pub max_intentions_per_invoke: u32,
    pub bitcoin_network: BitcoinNetwork,
}

#[derive(CandidType, Deserialize)]
pub struct ExecuteTxArgs {
    pub zero_confirmed_tx_queue_length: u32,
    pub txid: String,
    pub intention_set: IntentionSet,
    pub intention_index: u32,
    pub psbt_hex: String,
}

#[derive(CandidType, Deserialize)]
pub enum TxStatus {
    Confirmed(u32),
    Rejected(String),
    Pending,
}

#[derive(CandidType, Deserialize)]
pub struct TxDetailView {
    pub status: Option<TxStatus>,
    pub invoke_log: InvokeLogView,
    pub included_block: Option<BlockBasic>,
    pub sent_tx_hex: String,
}

#[derive(CandidType, Deserialize)]
pub struct OutpointWithValue {
    pub maybe_rune: Option<CoinBalance>,
    pub value: u64,
    pub script_pubkey_hex: String,
    pub outpoint: String,
}

#[derive(CandidType, Deserialize)]
pub struct NewBlockDetectedArgs {
    pub block_hash: String,
    pub block_timestamp: u64,
    pub tx_ids: Vec<String>,
    pub block_height: u32,
}

#[derive(CandidType, Deserialize)]
pub struct ExchangeMetadata {
    pub principal: Principal,
    pub exchange_id: String,
    pub name: String,
    pub description: String,
}

#[derive(CandidType, Deserialize)]
pub struct SaveIncludedBlockForTxArgs {
    pub txid: String,
    pub timestamp: u64,
    pub block: BlockBasic,
}

#[derive(CandidType, Deserialize)]
pub struct SetTxFeePerVbyteArgs {
    pub low: u64,
    pub high: u64,
    pub medium: u64,
}

pub struct Service(pub Principal);
impl Service {
    pub async fn clean_failed_invoke_logs(
        &self,
        arg0: Option<u64>,
        arg1: Vec<String>,
    ) -> Result<(Result_,)> {
        ic_cdk::call(self.0, "clean_failed_invoke_logs", (arg0, arg1)).await
    }
    pub async fn estimate_min_tx_fee(&self, arg0: EstimateMinTxFeeArgs) -> Result<(Result1,)> {
        ic_cdk::call(self.0, "estimate_min_tx_fee", (arg0,)).await
    }
    pub async fn get_canister_info(&self, arg0: u64) -> Result<(Result2,)> {
        ic_cdk::call(self.0, "get_canister_info", (arg0,)).await
    }
    pub async fn get_exchange_pools(&self) -> Result<(Vec<ExchangePool>,)> {
        ic_cdk::call(self.0, "get_exchange_pools", ()).await
    }
    pub async fn get_failed_invoke_logs(
        &self,
        arg0: GetFailedInvokeLogArgs,
    ) -> Result<(Vec<(String, InvokeLogView)>,)> {
        ic_cdk::call(self.0, "get_failed_invoke_logs", (arg0,)).await
    }
    pub async fn get_invoke_args_of_failed_invoke(
        &self,
        arg0: String,
    ) -> Result<(Option<InvokeArgs>,)> {
        ic_cdk::call(self.0, "get_invoke_args_of_failed_invoke", (arg0,)).await
    }
    pub async fn get_last_sent_txs(
        &self,
        arg0: Option<u32>,
    ) -> Result<(Vec<(String, String, Option<u32>)>,)> {
        ic_cdk::call(self.0, "get_last_sent_txs", (arg0,)).await
    }
    pub async fn get_mempool_tx_fee_rate(&self) -> Result<(MempoolTxFeeRateView,)> {
        ic_cdk::call(self.0, "get_mempool_tx_fee_rate", ()).await
    }
    pub async fn get_received_blocks(
        &self,
        arg0: Option<u32>,
        arg1: Option<bool>,
    ) -> Result<(Vec<ReceivedBlockView>,)> {
        ic_cdk::call(self.0, "get_received_blocks", (arg0, arg1)).await
    }
    pub async fn get_registered_exchanges(&self) -> Result<(Vec<ExchangeView>,)> {
        ic_cdk::call(self.0, "get_registered_exchanges", ()).await
    }
    pub async fn get_rejected_txs(&self, arg0: Option<u32>) -> Result<(Vec<RejectedTxView>,)> {
        ic_cdk::call(self.0, "get_rejected_txs", (arg0,)).await
    }
    pub async fn get_settings(&self) -> Result<(OrchestratorSettings,)> {
        ic_cdk::call(self.0, "get_settings", ()).await
    }
    pub async fn get_sign_psbt_args_of_failed_invoke(
        &self,
        arg0: String,
        arg1: u64,
    ) -> Result<(Option<ExecuteTxArgs>,)> {
        ic_cdk::call(self.0, "get_sign_psbt_args_of_failed_invoke", (arg0, arg1)).await
    }
    pub async fn get_tx_for_outpoint(&self, arg0: String) -> Result<(Option<TxDetailView>,)> {
        ic_cdk::call(self.0, "get_tx_for_outpoint", (arg0,)).await
    }
    pub async fn get_tx_queue_of_pool(
        &self,
        arg0: String,
    ) -> Result<(Vec<(String, Option<u32>)>,)> {
        ic_cdk::call(self.0, "get_tx_queue_of_pool", (arg0,)).await
    }
    pub async fn get_tx_sent(&self, arg0: String) -> Result<(Option<TxDetailView>,)> {
        ic_cdk::call(self.0, "get_tx_sent", (arg0,)).await
    }
    pub async fn get_used_outpoints(
        &self,
        arg0: Option<String>,
    ) -> Result<(Vec<(String, String)>,)> {
        ic_cdk::call(self.0, "get_used_outpoints", (arg0,)).await
    }
    pub async fn get_zero_confirmed_tx_count_of_pool(&self, arg0: String) -> Result<(u32,)> {
        ic_cdk::call(self.0, "get_zero_confirmed_tx_count_of_pool", (arg0,)).await
    }
    pub async fn get_zero_confirmed_txs(&self, arg0: Option<String>) -> Result<(Vec<String>,)> {
        ic_cdk::call(self.0, "get_zero_confirmed_txs", (arg0,)).await
    }
    pub async fn get_zero_confirmed_utxos_of_address(
        &self,
        arg0: String,
    ) -> Result<(Vec<OutpointWithValue>,)> {
        ic_cdk::call(self.0, "get_zero_confirmed_utxos_of_address", (arg0,)).await
    }
    pub async fn invoke(&self, arg0: InvokeArgs) -> Result<(Result3,)> {
        ic_cdk::call(self.0, "invoke", (arg0,)).await
    }
    pub async fn new_block_detected(&self, arg0: NewBlockDetectedArgs) -> Result<(Result_,)> {
        ic_cdk::call(self.0, "new_block_detected", (arg0,)).await
    }
    pub async fn register_exchange(&self, arg0: ExchangeMetadata) -> Result<(Result_,)> {
        ic_cdk::call(self.0, "register_exchange", (arg0,)).await
    }
    pub async fn reject_tx(&self, arg0: String, arg1: String) -> Result<(Result_,)> {
        ic_cdk::call(self.0, "reject_tx", (arg0, arg1)).await
    }
    pub async fn save_included_block_for_tx(
        &self,
        arg0: SaveIncludedBlockForTxArgs,
    ) -> Result<(Result_,)> {
        ic_cdk::call(self.0, "save_included_block_for_tx", (arg0,)).await
    }
    pub async fn set_max_input_count_of_psbt(&self, arg0: u32) -> Result<(Result_,)> {
        ic_cdk::call(self.0, "set_max_input_count_of_psbt", (arg0,)).await
    }
    pub async fn set_max_intentions_per_invoke(&self, arg0: u32) -> Result<(Result_,)> {
        ic_cdk::call(self.0, "set_max_intentions_per_invoke", (arg0,)).await
    }
    pub async fn set_max_unconfirmed_tx_count_in_pool(&self, arg0: u32) -> Result<(Result_,)> {
        ic_cdk::call(self.0, "set_max_unconfirmed_tx_count_in_pool", (arg0,)).await
    }
    pub async fn set_min_btc_amount_for_utxo(&self, arg0: u64) -> Result<(Result_,)> {
        ic_cdk::call(self.0, "set_min_btc_amount_for_utxo", (arg0,)).await
    }
    pub async fn set_min_tx_confirmations(&self, arg0: u32) -> Result<(Result_,)> {
        ic_cdk::call(self.0, "set_min_tx_confirmations", (arg0,)).await
    }
    pub async fn set_tx_fee_per_vbyte(&self, arg0: SetTxFeePerVbyteArgs) -> Result<(Result_,)> {
        ic_cdk::call(self.0, "set_tx_fee_per_vbyte", (arg0,)).await
    }
    pub async fn version(&self) -> Result<(String,)> {
        ic_cdk::call(self.0, "version", ()).await
    }
}
