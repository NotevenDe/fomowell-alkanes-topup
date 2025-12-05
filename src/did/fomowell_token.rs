// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Deserialize, Principal, Encode, Decode};
use ic_cdk::api::call::CallResult as Result;

#[derive(CandidType, Deserialize)]
pub struct Account {
  pub owner: Principal,
  pub subaccount: Option<serde_bytes::ByteBuf>,
}

#[derive(CandidType, Deserialize)]
pub struct TokenAmount { pub token: Principal, pub amount: candid::Nat }

#[derive(CandidType, Deserialize)]
pub struct InitArchiveArg {
  pub maxRecordsToArchive: candid::Nat,
  pub maxArchivePages: candid::Nat,
  pub settleToRecords: candid::Nat,
  pub archiveCycles: candid::Nat,
  pub maxActiveRecords: candid::Nat,
  pub maxRecordsInArchiveInstance: candid::Nat,
  pub archiveControllers: Option<Option<Vec<Principal>>>,
}

#[derive(CandidType, Deserialize)]
pub struct InitArg {
  pub fee_receiver: Account,
  pub rune_fee_rate: Option<u64>,
  pub create_token_fee: Vec<TokenAmount>,
  pub archive_init: Option<InitArchiveArg>,
  pub withdraw_ckbtc_fee: candid::Nat,
  pub ckbtc_minter: Principal,
  pub swap_fee: candid::Nat,
  pub token_launch_threshold: Vec<TokenAmount>,
  pub ckbtc_ledger: Principal,
  pub maintenance: bool,
  pub fee_percentage: Option<f32>,
  pub swap_burn: candid::Nat,
  pub btc_custody_canister: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct LiquidityAddArg {
  pub id: u64,
  pub sats: candid::Nat,
  pub nonce: u64,
  pub runes: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub enum Result_ { Ok(candid::Nat), Err(String) }

#[derive(CandidType, Deserialize)]
pub enum LedgerType {
  MemeToken(u64),
  #[serde(rename="ICRCToken")]
  IcrcToken(Principal),
}

#[derive(CandidType, Deserialize)]
pub struct AirdropArg {
  pub tos: Vec<Principal>,
  pub ledger: LedgerType,
  pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub enum Result1 { Ok, Err(String) }

#[derive(CandidType, Deserialize)]
pub struct QueryMemeTokenBalanceTokenArg {
  pub principal: Principal,
  pub meme_token_id: u64,
}

#[derive(CandidType, Deserialize)]
pub struct QueryMemeTokenBalanceResp {
  pub principal: Principal,
  pub balance: candid::Nat,
  pub meme_token_id: u64,
}

#[derive(CandidType, Deserialize)]
pub struct QueryLpArg { pub principal: Principal, pub meme_token_id: u64 }

#[derive(CandidType, Deserialize)]
pub struct QueryLpResp {
  pub principal: Principal,
  pub balance: candid::Nat,
  pub meme_token_id: u64,
}

#[derive(CandidType, Deserialize)]
pub struct BurnInitArg {
  pub subaccount: Option<serde_bytes::ByteBuf>,
  pub address: String,
  pub meme_token_id: u64,
  pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct BuyArgs {
  pub amount_out_min: Option<candid::Nat>,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub subaccount: Option<serde_bytes::ByteBuf>,
  pub amount_in: candid::Nat,
  pub meme_token_id: u64,
}

#[derive(CandidType, Deserialize)]
pub struct BuyResponse { pub amount_out: candid::Nat, pub is_completed: bool }

#[derive(CandidType, Deserialize)]
pub enum Result2 { Ok(BuyResponse), Err(String) }

#[derive(CandidType, Deserialize)]
pub struct StableToken {
  pub fee: candid::Nat,
  pub decimals: u8,
  pub name: String,
  pub canister_id: Principal,
  pub symbol: String,
}

#[derive(CandidType, Deserialize)]
pub struct ClaimArg { pub token: StableToken, pub claimer: Option<Principal> }

#[derive(CandidType, Deserialize)]
pub struct Logo { pub content_type: String, pub content_base64: String }

#[derive(CandidType, Deserialize)]
pub enum MemeTokenType {
  #[serde(rename="BRC2")]
  Brc2(String),
  Icrc(Principal),
  Rune(String),
  #[serde(rename="BRC20")]
  Brc20(String),
}

#[derive(CandidType, Deserialize)]
pub enum SwapFeeRate { H1, H2, T3, M100, M500 }

#[derive(CandidType, Deserialize)]
pub struct CreateMemeTokenArg {
  pub creator: Option<Principal>,
  pub ticker: String,
  pub logo_base64: Option<Logo>,
  pub twitter: Option<String>,
  pub logo: String,
  pub name: String,
  pub description: String,
  pub website: Option<String>,
  pub meme_token_type: MemeTokenType,
  pub swap_fee_rate: Option<SwapFeeRate>,
  pub dev_buy: Option<candid::Nat>,
  pub telegram: Option<String>,
}

#[derive(CandidType, Deserialize)]
pub struct SwapRatio { pub numer: candid::Nat, pub denom: candid::Nat }

#[derive(CandidType, Deserialize)]
pub struct FeesStorage {
  pub swap_fee: SwapRatio,
  pub swap_burn: SwapRatio,
  pub swap_creator_fee: Option<SwapRatio>,
}

#[derive(CandidType, Deserialize)]
pub struct Curve { pub sold: candid::Nat }

#[derive(CandidType, Deserialize)]
pub struct MemeToken {
  pub id: u64,
  pub creator: String,
  pub decimals: u8,
  pub ticker: String,
  pub available_token: candid::Nat,
  pub twitter: Option<String>,
  pub fees: FeesStorage,
  pub logo: String,
  pub name: String,
  pub market_cap_token: candid::Nat,
  pub curve: Curve,
  pub completed: bool,
  pub description: String,
  pub created_at: u64,
  pub website: Option<String>,
  pub meme_token_type: MemeTokenType,
  pub frozen: Option<bool>,
  pub price: f64,
  pub telegram: Option<String>,
  pub total_supply: candid::Nat,
  pub process: f64,
  pub is_etch: bool,
}

#[derive(CandidType, Deserialize)]
pub enum Result3 { Ok(MemeToken), Err(String) }

#[derive(CandidType, Deserialize)]
pub struct DepositArgs {
  pub to: Option<Account>,
  pub token: StableToken,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub subaccount: Option<serde_bytes::ByteBuf>,
  pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct DepositRuneArgs {
  pub to: Principal,
  pub memo: String,
  pub rune_name: String,
  pub subaccount: Option<serde_bytes::ByteBuf>,
  pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct StatusRequest {
  pub memory_size: bool,
  pub cycles: bool,
  pub heap_memory_size: bool,
}

#[derive(CandidType, Deserialize)]
pub enum MetricsGranularity {
  #[serde(rename="hourly")]
  Hourly,
  #[serde(rename="daily")]
  Daily,
}

#[derive(CandidType, Deserialize)]
pub struct GetMetricsParameters {
  pub dateToMillis: candid::Nat,
  pub granularity: MetricsGranularity,
  pub dateFromMillis: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct MetricsRequest { pub parameters: GetMetricsParameters }

#[derive(CandidType, Deserialize)]
pub struct GetLogMessagesFilter {
  pub analyzeCount: u32,
  pub messageRegex: Option<String>,
  pub messageContains: Option<String>,
}

#[derive(CandidType, Deserialize)]
pub struct GetLogMessagesParameters {
  pub count: u32,
  pub filter: Option<GetLogMessagesFilter>,
  pub fromTimeNanos: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct GetLatestLogMessagesParameters {
  pub upToTimeNanos: Option<u64>,
  pub count: u32,
  pub filter: Option<GetLogMessagesFilter>,
}

#[derive(CandidType, Deserialize)]
pub enum CanisterLogRequest {
  #[serde(rename="getMessagesInfo")]
  GetMessagesInfo,
  #[serde(rename="getMessages")]
  GetMessages(GetLogMessagesParameters),
  #[serde(rename="getLatestMessages")]
  GetLatestMessages(GetLatestLogMessagesParameters),
}

#[derive(CandidType, Deserialize)]
pub struct GetInformationRequest {
  pub status: Option<StatusRequest>,
  pub metrics: Option<MetricsRequest>,
  pub logs: Option<CanisterLogRequest>,
  pub version: bool,
}

#[derive(CandidType, Deserialize)]
pub struct StatusResponse {
  pub memory_size: Option<u64>,
  pub cycles: Option<u64>,
  pub heap_memory_size: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct HourlyMetricsData {
  pub updateCalls: Vec<u64>,
  pub canisterHeapMemorySize: Vec<u64>,
  pub canisterCycles: Vec<u64>,
  pub canisterMemorySize: Vec<u64>,
  pub timeMillis: candid::Int,
}

#[derive(CandidType, Deserialize)]
pub struct NumericEntity {
  pub avg: u64,
  pub max: u64,
  pub min: u64,
  pub first: u64,
  pub last: u64,
}

#[derive(CandidType, Deserialize)]
pub struct DailyMetricsData {
  pub updateCalls: u64,
  pub canisterHeapMemorySize: NumericEntity,
  pub canisterCycles: NumericEntity,
  pub canisterMemorySize: NumericEntity,
  pub timeMillis: candid::Int,
}

#[derive(CandidType, Deserialize)]
pub enum CanisterMetricsData {
  #[serde(rename="hourly")]
  Hourly(Vec<HourlyMetricsData>),
  #[serde(rename="daily")]
  Daily(Vec<DailyMetricsData>),
}

#[derive(CandidType, Deserialize)]
pub struct CanisterMetrics { pub data: CanisterMetricsData }

#[derive(CandidType, Deserialize)]
pub struct MetricsResponse { pub metrics: Option<CanisterMetrics> }

#[derive(CandidType, Deserialize)]
pub enum CanisterLogFeature {
  #[serde(rename="filterMessageByContains")]
  FilterMessageByContains,
  #[serde(rename="filterMessageByRegex")]
  FilterMessageByRegex,
}

#[derive(CandidType, Deserialize)]
pub struct CanisterLogMessagesInfo {
  pub features: Vec<Option<CanisterLogFeature>>,
  pub lastTimeNanos: Option<u64>,
  pub count: u32,
  pub firstTimeNanos: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct LogMessageData { pub timeNanos: u64, pub message: String }

#[derive(CandidType, Deserialize)]
pub struct CanisterLogMessages {
  pub data: Vec<LogMessageData>,
  pub lastAnalyzedMessageTimeNanos: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub enum CanisterLogResponse {
  #[serde(rename="messagesInfo")]
  MessagesInfo(CanisterLogMessagesInfo),
  #[serde(rename="messages")]
  Messages(CanisterLogMessages),
}

#[derive(CandidType, Deserialize)]
pub struct GetInformationResponse {
  pub status: Option<StatusResponse>,
  pub metrics: Option<MetricsResponse>,
  pub logs: Option<CanisterLogResponse>,
  pub version: Option<candid::Nat>,
}

#[derive(CandidType, Deserialize)]
pub struct TransactionRange { pub start: candid::Nat, pub length: candid::Nat }

#[derive(CandidType, Deserialize)]
pub struct Buy {
  pub fee: Option<candid::Nat>,
  pub token: Principal,
  pub from: Account,
  pub amount_out: candid::Nat,
  pub reserve_out: candid::Nat,
  pub amount_in: candid::Nat,
  pub reserve_in: candid::Nat,
  pub meme_token_id: u64,
}

#[derive(CandidType, Deserialize)]
pub struct InternalTransfer {
  pub to: Account,
  pub token: Option<Principal>,
  pub from: Account,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub meme_token_id: Option<u64>,
  pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub enum SwapType {
  #[serde(rename="rune_to_btc")]
  RuneToBtc,
  #[serde(rename="btc_to_rune")]
  BtcToRune,
}

#[derive(CandidType, Deserialize)]
pub struct InnerSwap {
  pub fee: Option<candid::Nat>,
  pub token: Principal,
  pub from: Account,
  pub sats: candid::Nat,
  pub sold: candid::Nat,
  pub meme_token_id: u64,
  pub amount: candid::Nat,
  pub swap_type: SwapType,
}

#[derive(CandidType, Deserialize)]
pub struct WithdrawRune {
  pub to: String,
  pub from: Principal,
  pub memo: String,
  pub rune_name: String,
  pub meme_token_id: u64,
  pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct AddLiquidity {
  pub token: Principal,
  pub from: Account,
  pub reserve_runes: candid::Nat,
  pub input_runes: candid::Nat,
  pub reserve_sats: candid::Nat,
  pub meme_token_id: u64,
  pub input_sats: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct WithdrawRewards {
  pub to: Account,
  pub token: Principal,
  pub from: Account,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub enum LinearType {
  Days360Times12,
  Days270Times9,
  Days180Times6,
  Days90Times3,
}

#[derive(CandidType, Deserialize)]
pub enum FixedType { Days30, Days90, Days180, Days360, Days720 }

#[derive(CandidType, Deserialize)]
pub enum LockType { Linear(LinearType), Fixed(FixedType) }

#[derive(CandidType, Deserialize)]
pub struct LockToken {
  pub id: u64,
  pub unlock_time: u64,
  pub lock_token_type: String,
  pub start_time: u64,
  pub account: Account,
  pub lock_type: LockType,
  pub meme_token_id: u64,
  pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct Deposit {
  pub to: Account,
  pub height: candid::Nat,
  pub token: Principal,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub amount: candid::Nat,
  pub spender: Account,
}

#[derive(CandidType, Deserialize)]
pub struct DepositRune {
  pub to: Principal,
  pub memo: String,
  pub subaccount: Option<serde_bytes::ByteBuf>,
  pub meme_token_id: u64,
  pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct BurnTx {
  pub account: Account,
  pub meme_token_id: u64,
  pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub enum BTreeMapItem1 {
  Int(candid::Int),
  Map(Box<BTreeMap>),
  Nat(candid::Nat),
  Nat64(u64),
  Blob(serde_bytes::ByteBuf),
  Text(String),
  Array(Vec<Box<Value>>),
}

#[derive(CandidType, Deserialize)]
pub struct BTreeMap(Vec<(String,BTreeMapItem1,)>);

#[derive(CandidType, Deserialize)]
pub enum Value {
  Int(candid::Int),
  Map(Box<BTreeMap>),
  Nat(candid::Nat),
  Nat64(u64),
  Blob(serde_bytes::ByteBuf),
  Text(String),
  Array(Vec<Box<Value>>),
}

#[derive(CandidType, Deserialize)]
pub struct Mint {
  pub import: bool,
  pub meme_token0: u64,
  pub metadata: Vec<(String,Box<Value>,)>,
  pub from: Account,
  pub reserve0: candid::Nat,
  pub reserve1: candid::Nat,
  pub token1: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct WithdrawCkbtc {
  pub to: String,
  pub token: Principal,
  pub block_index: u64,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub amount: candid::Nat,
  pub spender: Account,
}

#[derive(CandidType, Deserialize)]
pub struct UnlockToken {
  pub id: u64,
  pub unlock_time: u64,
  pub lock_token_type: String,
  pub account: Account,
  pub meme_token_id: u64,
  pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct WithdrawIncome {
  pub to: Account,
  pub token: Principal,
  pub meme_token_id: u64,
  pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct WithdrawLiquidity {
  pub to: Account,
  pub token: Principal,
  pub out_put_liquidity: candid::Nat,
  pub reserve_runes: candid::Nat,
  pub output_runes: candid::Nat,
  pub reserve_sats: candid::Nat,
  pub output_sats: candid::Nat,
  pub meme_token_id: u64,
}

#[derive(CandidType, Deserialize)]
pub struct OuterSwap {
  pub fee: candid::Nat,
  pub token: Principal,
  pub burn: candid::Nat,
  pub from: Account,
  pub sats: candid::Nat,
  pub reserve_runes: candid::Nat,
  pub nonce: u64,
  pub reserve_sats: candid::Nat,
  pub meme_token_id: u64,
  pub swap_type: SwapType,
  pub runes: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct Transfer {
  pub to: Account,
  pub from: Account,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub ledger: LedgerType,
  pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct InternalTransferLp {
  pub to: Account,
  pub from: Account,
  pub sats: candid::Nat,
  pub meme_token_id: u64,
  pub amount: candid::Nat,
  pub runes: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct Transaction {
  pub buy: Option<Buy>,
  pub internal_transfer: Option<InternalTransfer>,
  pub inner_swap: Option<InnerSwap>,
  pub withdraw_rune: Option<WithdrawRune>,
  pub add_liquidity: Option<AddLiquidity>,
  pub withdraw_rewards: Option<WithdrawRewards>,
  pub lock_token: Option<LockToken>,
  pub withdraw: Option<Deposit>,
  pub deposit_rune: Option<DepositRune>,
  pub burn: Option<BurnTx>,
  pub kind: String,
  pub mint: Option<Mint>,
  pub sell: Option<Buy>,
  pub deposit: Option<Deposit>,
  pub withdraw_ckbtc: Option<WithdrawCkbtc>,
  pub unlock_token: Option<UnlockToken>,
  pub withdraw_lp_earning: Option<WithdrawIncome>,
  pub withdraw_liquidity: Option<WithdrawLiquidity>,
  pub timestamp: u64,
  pub outer_swap: Option<OuterSwap>,
  pub index: candid::Nat,
  pub transfer: Option<Transfer>,
  pub withdraw_income: Option<WithdrawIncome>,
  pub internal_transfer_lp: Option<InternalTransferLp>,
}

#[derive(CandidType, Deserialize)]
pub struct Burn {
  pub from: Account,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub created_at_time: Option<u64>,
  pub amount: candid::Nat,
  pub spender: Option<Account>,
}

#[derive(CandidType, Deserialize)]
pub struct Mint1 {
  pub to: Account,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub created_at_time: Option<u64>,
  pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct Approve {
  pub fee: Option<candid::Nat>,
  pub from: Account,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub created_at_time: Option<u64>,
  pub amount: candid::Nat,
  pub expected_allowance: Option<candid::Nat>,
  pub expires_at: Option<u64>,
  pub spender: Account,
}

#[derive(CandidType, Deserialize)]
pub struct Transfer1 {
  pub to: Account,
  pub fee: Option<candid::Nat>,
  pub from: Account,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub created_at_time: Option<u64>,
  pub amount: candid::Nat,
  pub spender: Option<Account>,
}

#[derive(CandidType, Deserialize)]
pub struct Transaction1 {
  pub burn: Option<Burn>,
  pub kind: String,
  pub mint: Option<Mint1>,
  pub approve: Option<Approve>,
  pub timestamp: u64,
  pub transfer: Option<Transfer1>,
}

#[derive(CandidType, Deserialize)]
pub struct TransactionRange1 { pub transactions: Vec<Transaction1> }

candid::define_function!(pub ArchivedRangeCallback : (TransactionRange) -> (
    TransactionRange1,
  ) query);
#[derive(CandidType, Deserialize)]
pub struct ArchivedRange {
  pub callback: ArchivedRangeCallback,
  pub start: candid::Nat,
  pub length: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct GetTransactionsResponse {
  pub first_index: candid::Nat,
  pub log_length: candid::Nat,
  pub transactions: Vec<Transaction>,
  pub archived_transactions: Vec<ArchivedRange>,
}

#[derive(CandidType, Deserialize)]
pub struct SupportedStandard { pub url: String, pub name: String }

#[derive(CandidType, Deserialize)]
pub struct ConsentMessageMetadata {
  pub utc_offset_minutes: Option<i16>,
  pub language: String,
}

#[derive(CandidType, Deserialize)]
pub enum DisplayMessageType {
  GenericDisplay,
  LineDisplay{ characters_per_line: u16, lines_per_page: u16 },
}

#[derive(CandidType, Deserialize)]
pub struct ConsentMessageSpec {
  pub metadata: ConsentMessageMetadata,
  pub device_spec: Option<DisplayMessageType>,
}

#[derive(CandidType, Deserialize)]
pub struct ConsentMessageRequest {
  pub arg: serde_bytes::ByteBuf,
  pub method: String,
  pub user_preferences: ConsentMessageSpec,
}

#[derive(CandidType, Deserialize)]
pub struct LineDisplayPage { pub lines: Vec<String> }

#[derive(CandidType, Deserialize)]
pub enum ConsentMessage {
  LineDisplayMessage{ pages: Vec<LineDisplayPage> },
  GenericDisplayMessage(String),
}

#[derive(CandidType, Deserialize)]
pub struct ConsentInfo {
  pub metadata: ConsentMessageMetadata,
  pub consent_message: ConsentMessage,
}

#[derive(CandidType, Deserialize)]
pub struct ErrorInfo { pub description: String }

#[derive(CandidType, Deserialize)]
pub enum Icrc21Error {
  GenericError{ description: String, error_code: candid::Nat },
  InsufficientPayment(ErrorInfo),
  UnsupportedCanisterCall(ErrorInfo),
  ConsentMessageUnavailable(ErrorInfo),
}

#[derive(CandidType, Deserialize)]
pub enum Result4 { Ok(ConsentInfo), Err(Icrc21Error) }

#[derive(CandidType, Deserialize)]
pub struct Icrc28TrustedOrigins { pub trusted_origins: Vec<String> }

#[derive(CandidType, Deserialize)]
pub struct ImportTokenArg {
  pub creator: Option<Principal>,
  pub decimals: u8,
  pub ticker: String,
  pub twitter: Option<String>,
  pub logo: String,
  pub name: String,
  pub sats: candid::Nat,
  pub rune_name: String,
  pub description: String,
  pub website: Option<String>,
  pub meme_token_type: MemeTokenType,
  pub swap_fee_rate: Option<SwapFeeRate>,
  pub telegram: Option<String>,
  pub total_supply: candid::Nat,
  pub runes: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub enum Result5 { Ok(u64), Err(String) }

#[derive(CandidType, Deserialize)]
pub struct InitLpTokenLock {
  pub lock_type: LockType,
  pub meme_token_id: u64,
  pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct LockTokenRecord {
  pub id: u64,
  pub unlock_time: u64,
  pub start_time: u64,
  pub account: Account,
  pub lock_type: LockType,
  pub meme_token_id: u64,
  pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub enum Result6 { Ok(Vec<LockTokenRecord>), Err(String) }

#[derive(CandidType, Deserialize)]
pub struct InitTokenLock {
  pub subaccount: Option<serde_bytes::ByteBuf>,
  pub account: Account,
  pub lock_type: LockType,
  pub meme_token_id: u64,
  pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct InternalTransferArg {
  pub to: Account,
  pub lock_id: Option<u64>,
  pub subaccount: Option<serde_bytes::ByteBuf>,
  pub ledger_type: LedgerType,
  pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct InternalTransferLpArg {
  pub lp: candid::Nat,
  pub to: Account,
  pub lock_id: Option<u64>,
  pub meme_token_id: u64,
}

#[derive(CandidType, Deserialize)]
pub struct InternalTransferLpResponse {
  pub lp: candid::Nat,
  pub decimals: u8,
  pub sats: candid::Nat,
  pub runes: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub enum Result7 { Ok(InternalTransferLpResponse), Err(String) }

#[derive(CandidType, Deserialize)]
pub struct MintLiquidity {
  pub id: u64,
  pub sats: candid::Nat,
  pub lp_provider: Option<Account>,
  pub runes: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct PoolView {
  pub k: candid::Nat,
  pub id: u64,
  pub sats: candid::Nat,
  pub runes: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub enum Result8 { Ok(PoolView), Err(String) }

#[derive(CandidType, Deserialize)]
pub struct DepositBrcArgs {
  pub pid: Principal,
  pub ticker: String,
  pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct PreLiquidityAddArg {
  pub id: u64,
  pub sats: Option<candid::Nat>,
  pub runes: Option<candid::Nat>,
}

#[derive(CandidType, Deserialize)]
pub enum Result9 { Ok(LiquidityAddArg), Err(String) }

#[derive(CandidType, Deserialize)]
pub struct PreRunesSwapSatsArg { pub id: u64, pub runes: candid::Nat }

#[derive(CandidType, Deserialize)]
pub struct PreRunesSwapSatsResponse { pub sats: candid::Nat, pub nonce: u64 }

#[derive(CandidType, Deserialize)]
pub enum Result10 { Ok(PreRunesSwapSatsResponse), Err(String) }

#[derive(CandidType, Deserialize)]
pub struct PreSatsSwapRunesArg { pub id: u64, pub sats: candid::Nat }

#[derive(CandidType, Deserialize)]
pub struct PreSatsSwapRunesResponse { pub nonce: u64, pub runes: candid::Nat }

#[derive(CandidType, Deserialize)]
pub enum Result11 { Ok(PreSatsSwapRunesResponse), Err(String) }

#[derive(CandidType, Deserialize)]
pub struct PreLiquidityRemoveArg { pub id: u64, pub liquidity: candid::Nat }

#[derive(CandidType, Deserialize)]
pub struct LockLpRecordView {
  pub id: u64,
  pub decimals: u8,
  pub unlock_time: u64,
  pub sats: candid::Nat,
  pub start_time: u64,
  pub account: Account,
  pub lock_type: LockType,
  pub meme_token_id: u64,
  pub amount: candid::Nat,
  pub runes: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct BrcMetadata {
  pub max: candid::Nat,
  pub tick: String,
  pub meme_token_id: u64,
  pub decimal: u8,
  pub standard: String,
}

#[derive(CandidType, Deserialize)]
pub struct FreezeState {
  pub maintainers: Vec<Principal>,
  pub maintainer: Account,
  pub maintenance: bool,
  pub maintenance_withdraw: bool,
}

#[derive(CandidType, Deserialize)]
pub struct MemeTokenView {
  pub id: u64,
  pub creator: String,
  pub decimals: u8,
  pub ticker: String,
  pub available_token: candid::Nat,
  pub twitter: Option<String>,
  pub fees: FeesStorage,
  pub logo: String,
  pub name: String,
  pub pool: Option<PoolView>,
  pub market_cap_token: candid::Nat,
  pub curve: Curve,
  pub completed: bool,
  pub is_frozen: bool,
  pub description: String,
  pub created_at: u64,
  pub website: Option<String>,
  pub meme_token_type: String,
  pub price: f64,
  pub telegram: Option<String>,
  pub total_supply: candid::Nat,
  pub process: f64,
  pub is_etch: bool,
}

#[derive(CandidType, Deserialize)]
pub struct LockInfo {
  pub token_lock_value: candid::Nat,
  pub lp_lock_value: candid::Nat,
  pub lp_value: candid::Nat,
  pub token_value: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct Liquidity {
  pub lp: candid::Nat,
  pub owner: Principal,
  pub locked: bool,
  pub percentage: f64,
}

#[derive(CandidType, Deserialize)]
pub enum Result12 { Ok(f64), Err(String) }

#[derive(CandidType, Deserialize)]
pub enum Sort { CreateTimeDsc, MarketCapDsc }

#[derive(CandidType, Deserialize)]
pub struct QueryMemeTokenArgs {
  pub sort: Option<Sort>,
  pub start: u64,
  pub length: u64,
}

#[derive(CandidType, Deserialize)]
pub struct QueryMemeTokenResponse {
  pub meme_tokens: Vec<MemeTokenView>,
  pub count: u64,
}

#[derive(CandidType, Deserialize)]
pub struct ArchiveSetting {
  pub max_records_in_archive_instance: candid::Nat,
  pub archive_cycles: candid::Nat,
  pub settle_to_records: candid::Nat,
  pub archive_controllers: Option<Option<Vec<Principal>>>,
  pub max_active_records: candid::Nat,
  pub max_records_to_archive: candid::Nat,
  pub max_archive_pages: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct ArchiveLedgerInfo {
  pub setting: ArchiveSetting,
  pub last_index: candid::Nat,
  pub first_index: candid::Nat,
  pub local_ledger_size: candid::Nat,
  pub txn_count: candid::Nat,
  pub archive_txn_count: candid::Nat,
  pub is_cleaning: bool,
  pub archives: Vec<(Principal,TransactionRange,)>,
}

#[derive(CandidType, Deserialize)]
pub struct State {
  pub archive_ledger_info: ArchiveLedgerInfo,
  pub fee_receiver: Account,
  pub rune_fee_rate: Option<u64>,
  pub create_token_fee: Vec<TokenAmount>,
  pub withdraw_ckbtc_fee: candid::Nat,
  pub ckbtc_minter: Principal,
  pub swap_fee: candid::Nat,
  pub token_launch_threshold: Vec<TokenAmount>,
  pub ckbtc_ledger: Principal,
  pub maintainer: Account,
  pub maintenance: bool,
  pub fee_percentage: Option<f32>,
  pub swap_burn: candid::Nat,
  pub btc_custody_canister: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct Holder { pub balance: candid::Nat, pub account: Account }

#[derive(CandidType, Deserialize)]
pub struct HolderView {
  pub balance: candid::Nat,
  pub locked: bool,
  pub account: Account,
}

#[derive(CandidType, Deserialize)]
pub struct LiquidityProviderView {
  pub id: u64,
  pub lp: candid::Nat,
  pub decimals: u8,
  pub ticker: String,
  pub lp_sats_value: candid::Nat,
  pub logo: String,
  pub lp_runes_value: candid::Nat,
  pub lp_earning: candid::Nat,
  pub percentage: f64,
}

#[derive(CandidType, Deserialize)]
pub struct LpV2View {
  pub id: Option<u64>,
  pub decimals: u8,
  pub sats: candid::Nat,
  pub meme_token_id: u64,
  pub amount: candid::Nat,
  pub runes: candid::Nat,
  pub is_locked: bool,
}

#[derive(CandidType, Deserialize)]
pub struct LiquidityProviderV2View {
  pub lp: candid::Nat,
  pub decimals: u8,
  pub ticker: String,
  pub lp_sats_value: candid::Nat,
  pub logo: String,
  pub lp_view: Vec<LpV2View>,
  pub lp_runes_value: candid::Nat,
  pub meme_token_id: u64,
}

#[derive(CandidType, Deserialize)]
pub struct MemeTokenBalance {
  pub decimals: u8,
  pub token: MemeToken,
  pub balance: candid::Nat,
  pub lock_id: Option<u64>,
  pub locked: bool,
}

#[derive(CandidType, Deserialize)]
pub enum MemeTType {
  #[serde(rename="BRC2")]
  Brc2,
  Icrc,
  Rune,
  #[serde(rename="BRC20")]
  Brc20,
}

#[derive(CandidType, Deserialize)]
pub struct MemeTokenBalanceV2 {
  pub decimals: u8,
  pub token: MemeToken,
  pub balance: candid::Nat,
  pub locked: bool,
  pub lock_info: Option<LockTokenRecord>,
}

#[derive(CandidType, Deserialize)]
pub struct WithdrawBrcInfo {
  pub id: u64,
  pub pid: Principal,
  pub tick: String,
  pub address: String,
  pub meme_token_id: u64,
  pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct RunesSwapSatsArg {
  pub id: u64,
  pub sats_min: Option<candid::Nat>,
  pub nonce: u64,
  pub runes: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct SatsSwapRunesArg {
  pub id: u64,
  pub sats: candid::Nat,
  pub nonce: u64,
  pub runes_min: Option<candid::Nat>,
}

#[derive(CandidType, Deserialize)]
pub enum Result13 { Ok(LockTokenRecord), Err(String) }

#[derive(CandidType, Deserialize)]
pub struct UpdateMemeTokenInfoArg {
  pub id: u64,
  pub twitter: Option<String>,
  pub website: Option<String>,
  pub telegram: Option<String>,
}

#[derive(CandidType, Deserialize)]
pub struct WithdrawArgs {
  pub to: Account,
  pub token: StableToken,
  pub from: Account,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub subaccount: Option<serde_bytes::ByteBuf>,
  pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct WithdrawBrcArg {
  pub address: String,
  pub meme_token_id: u64,
  pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct WithdrawByCkbtcArgs {
  pub to: String,
  pub token: StableToken,
  pub from: Account,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub subaccount: Option<serde_bytes::ByteBuf>,
  pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct LiquidityRemoveArg {
  pub id: u64,
  pub liquidity: candid::Nat,
  pub nonce: u64,
}

#[derive(CandidType, Deserialize)]
pub struct WithdrawRewardsArgs {
  pub to: Account,
  pub token: Principal,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct WithdrawRuneArgs {
  pub value: candid::Nat,
  pub memo: String,
  pub address: String,
  pub meme_token_id: u64,
}

pub struct Service(pub Principal);
impl Service {
  pub async fn get_candid_interface_tmp_hack(&self) -> Result<(String,)> {
    ic_cdk::call(self.0, "__get_candid_interface_tmp_hack", ()).await
  }
  pub async fn account_available(&self, arg0: Account) -> Result<(bool,)> {
    ic_cdk::call(self.0, "account_available", (arg0,)).await
  }
  pub async fn add_liquidity(&self, arg0: LiquidityAddArg) -> Result<
    (Result_,)
  > { ic_cdk::call(self.0, "add_liquidity", (arg0,)).await }
  pub async fn airdrop(&self, arg0: AirdropArg) -> Result<(Result1,)> {
    ic_cdk::call(self.0, "airdrop", (arg0,)).await
  }
  pub async fn batch_icrc_1_balance_of(
    &self,
    arg0: Vec<QueryMemeTokenBalanceTokenArg>,
  ) -> Result<(Vec<QueryMemeTokenBalanceResp>,)> {
    ic_cdk::call(self.0, "batch_icrc1_balance_of", (arg0,)).await
  }
  pub async fn batch_query_user_meme_token_lp(
    &self,
    arg0: Vec<QueryLpArg>,
  ) -> Result<(Vec<QueryLpResp>,)> {
    ic_cdk::call(self.0, "batch_query_user_meme_token_lp", (arg0,)).await
  }
  pub async fn burn(&self, arg0: BurnInitArg) -> Result<(Result_,)> {
    ic_cdk::call(self.0, "burn", (arg0,)).await
  }
  pub async fn buy(&self, arg0: BuyArgs) -> Result<(Result2,)> {
    ic_cdk::call(self.0, "buy", (arg0,)).await
  }
  pub async fn calculate_buy(&self, arg0: u64, arg1: candid::Nat) -> Result<
    (Result_,)
  > { ic_cdk::call(self.0, "calculate_buy", (arg0,arg1,)).await }
  pub async fn calculate_sell(&self, arg0: u64, arg1: candid::Nat) -> Result<
    (Result_,)
  > { ic_cdk::call(self.0, "calculate_sell", (arg0,arg1,)).await }
  pub async fn claim(&self, arg0: ClaimArg) -> Result<(Result_,)> {
    ic_cdk::call(self.0, "claim", (arg0,)).await
  }
  pub async fn create_token(&self, arg0: CreateMemeTokenArg) -> Result<
    (Result3,)
  > { ic_cdk::call(self.0, "create_token", (arg0,)).await }
  pub async fn deposit(&self, arg0: DepositArgs) -> Result<(Result_,)> {
    ic_cdk::call(self.0, "deposit", (arg0,)).await
  }
  pub async fn deposit_rune(&self, arg0: DepositRuneArgs) -> Result<
    (Result1,)
  > { ic_cdk::call(self.0, "deposit_rune", (arg0,)).await }
  pub async fn generate_random(&self) -> Result<(u64,)> {
    ic_cdk::call(self.0, "generate_random", ()).await
  }
  pub async fn get_canistergeek_information(
    &self,
    arg0: GetInformationRequest,
  ) -> Result<(GetInformationResponse,)> {
    ic_cdk::call(self.0, "getCanistergeekInformation", (arg0,)).await
  }
  pub async fn get_transactions(&self, arg0: TransactionRange) -> Result<
    (GetTransactionsResponse,)
  > { ic_cdk::call(self.0, "get_transactions", (arg0,)).await }
  pub async fn icrc_10_supported_standards(&self) -> Result<
    (Vec<SupportedStandard>,)
  > { ic_cdk::call(self.0, "icrc10_supported_standards", ()).await }
  pub async fn icrc_1_balance_of(
    &self,
    arg0: LedgerType,
    arg1: Account,
  ) -> Result<(candid::Nat,)> {
    ic_cdk::call(self.0, "icrc1_balance_of", (arg0,arg1,)).await
  }
  pub async fn icrc_21_canister_call_consent_message(
    &self,
    arg0: ConsentMessageRequest,
  ) -> Result<(Result4,)> {
    ic_cdk::call(self.0, "icrc21_canister_call_consent_message", (arg0,)).await
  }
  pub async fn icrc_28_trusted_origins(&self) -> Result<
    (Icrc28TrustedOrigins,)
  > { ic_cdk::call(self.0, "icrc28_trusted_origins", ()).await }
  pub async fn import_token(&self, arg0: ImportTokenArg) -> Result<(Result5,)> {
    ic_cdk::call(self.0, "import_token", (arg0,)).await
  }
  pub async fn init_lock_lp(&self, arg0: InitLpTokenLock) -> Result<
    (Result6,)
  > { ic_cdk::call(self.0, "init_lock_lp", (arg0,)).await }
  pub async fn init_lock_token(&self, arg0: InitTokenLock) -> Result<
    (Result6,)
  > { ic_cdk::call(self.0, "init_lock_token", (arg0,)).await }
  pub async fn internal_transfer(&self, arg0: InternalTransferArg) -> Result<
    (Result1,)
  > { ic_cdk::call(self.0, "internal_transfer", (arg0,)).await }
  pub async fn internal_transfer_lp(
    &self,
    arg0: InternalTransferLpArg,
  ) -> Result<(Result7,)> {
    ic_cdk::call(self.0, "internal_transfer_lp", (arg0,)).await
  }
  pub async fn migrate(&self) -> Result<()> {
    ic_cdk::call(self.0, "migrate", ()).await
  }
  pub async fn mint_liquidity(&self, arg0: MintLiquidity) -> Result<
    (Result8,)
  > { ic_cdk::call(self.0, "mint_liquidity", (arg0,)).await }
  pub async fn multi_balance_of(
    &self,
    arg0: Vec<LedgerType>,
    arg1: Account,
  ) -> Result<(Vec<candid::Nat>,)> {
    ic_cdk::call(self.0, "multi_balance_of", (arg0,arg1,)).await
  }
  pub async fn notify_deployed_brc(&self, arg0: u64) -> Result<()> {
    ic_cdk::call(self.0, "notify_deployed_brc", (arg0,)).await
  }
  pub async fn notify_deposit_brc(&self, arg0: DepositBrcArgs) -> Result<
    (Result1,)
  > { ic_cdk::call(self.0, "notify_deposit_brc", (arg0,)).await }
  pub async fn notify_import_brc(&self, arg0: u64) -> Result<()> {
    ic_cdk::call(self.0, "notify_import_brc", (arg0,)).await
  }
  pub async fn notify_withdraw_brc(&self, arg0: u64) -> Result<(Result1,)> {
    ic_cdk::call(self.0, "notify_withdraw_brc", (arg0,)).await
  }
  pub async fn pre_add_liquidity(&self, arg0: PreLiquidityAddArg) -> Result<
    (Result9,)
  > { ic_cdk::call(self.0, "pre_add_liquidity", (arg0,)).await }
  pub async fn pre_runes_swap_sats(&self, arg0: PreRunesSwapSatsArg) -> Result<
    (Result10,)
  > { ic_cdk::call(self.0, "pre_runes_swap_sats", (arg0,)).await }
  pub async fn pre_sats_swap_runes(&self, arg0: PreSatsSwapRunesArg) -> Result<
    (Result11,)
  > { ic_cdk::call(self.0, "pre_sats_swap_runes", (arg0,)).await }
  pub async fn pre_withdraw_liquidity(
    &self,
    arg0: PreLiquidityRemoveArg,
  ) -> Result<(Result9,)> {
    ic_cdk::call(self.0, "pre_withdraw_liquidity", (arg0,)).await
  }
  pub async fn query_account_lock_lp(&self, arg0: Option<Account>) -> Result<
    (Vec<LockTokenRecord>,)
  > { ic_cdk::call(self.0, "query_account_lock_lp", (arg0,)).await }
  pub async fn query_account_lock_tokens(
    &self,
    arg0: Option<Account>,
  ) -> Result<(Vec<LockTokenRecord>,)> {
    ic_cdk::call(self.0, "query_account_lock_tokens", (arg0,)).await
  }
  pub async fn query_account_meme_token_lock_lp(
    &self,
    arg0: u64,
    arg1: Option<Account>,
  ) -> Result<(Vec<LockLpRecordView>,)> {
    ic_cdk::call(self.0, "query_account_meme_token_lock_lp", (arg0,arg1,)).await
  }
  pub async fn query_account_meme_token_lock_tokens(
    &self,
    arg0: u64,
    arg1: Option<Account>,
  ) -> Result<(Vec<LockTokenRecord>,)> {
    ic_cdk::call(self.0, "query_account_meme_token_lock_tokens", (
      arg0,arg1,
    )).await
  }
  pub async fn query_deploying_brc(&self) -> Result<(Vec<BrcMetadata>,)> {
    ic_cdk::call(self.0, "query_deploying_brc", ()).await
  }
  pub async fn query_freeze(&self) -> Result<(FreezeState,)> {
    ic_cdk::call(self.0, "query_freeze", ()).await
  }
  pub async fn query_import_brc(&self) -> Result<(Vec<BrcMetadata>,)> {
    ic_cdk::call(self.0, "query_import_brc", ()).await
  }
  pub async fn query_meme_token(&self, arg0: u64) -> Result<
    (Option<MemeTokenView>,)
  > { ic_cdk::call(self.0, "query_meme_token", (arg0,)).await }
  pub async fn query_meme_token_lock_info(&self, arg0: u64) -> Result<
    (LockInfo,)
  > { ic_cdk::call(self.0, "query_meme_token_lock_info", (arg0,)).await }
  pub async fn query_meme_token_lock_lp(&self, arg0: u64) -> Result<
    (Vec<LockTokenRecord>,)
  > { ic_cdk::call(self.0, "query_meme_token_lock_lp", (arg0,)).await }
  pub async fn query_meme_token_lock_tokens(&self, arg0: u64) -> Result<
    (Vec<LockTokenRecord>,)
  > { ic_cdk::call(self.0, "query_meme_token_lock_tokens", (arg0,)).await }
  pub async fn query_meme_token_lp(&self, arg0: u64) -> Result<
    (Vec<Liquidity>,)
  > { ic_cdk::call(self.0, "query_meme_token_lp", (arg0,)).await }
  pub async fn query_meme_token_lp_v_2(&self, arg0: u64) -> Result<
    (Vec<Liquidity>,)
  > { ic_cdk::call(self.0, "query_meme_token_lp_v2", (arg0,)).await }
  pub async fn query_meme_token_pool(&self, arg0: u64) -> Result<
    (Option<PoolView>,)
  > { ic_cdk::call(self.0, "query_meme_token_pool", (arg0,)).await }
  pub async fn query_meme_token_price(&self, arg0: u64) -> Result<(Result12,)> {
    ic_cdk::call(self.0, "query_meme_token_price", (arg0,)).await
  }
  pub async fn query_meme_tokens(&self, arg0: QueryMemeTokenArgs) -> Result<
    (QueryMemeTokenResponse,)
  > { ic_cdk::call(self.0, "query_meme_tokens", (arg0,)).await }
  pub async fn query_pool_income(&self, arg0: u64) -> Result<(candid::Nat,)> {
    ic_cdk::call(self.0, "query_pool_income", (arg0,)).await
  }
  pub async fn query_state(&self) -> Result<(State,)> {
    ic_cdk::call(self.0, "query_state", ()).await
  }
  pub async fn query_token_holders(
    &self,
    arg0: u64,
    arg1: u64,
    arg2: u64,
  ) -> Result<(Vec<Holder>,u64,)> {
    ic_cdk::call(self.0, "query_token_holders", (arg0,arg1,arg2,)).await
  }
  pub async fn query_token_holders_v_2(
    &self,
    arg0: u64,
    arg1: u64,
    arg2: u64,
  ) -> Result<(Vec<HolderView>,u64,)> {
    ic_cdk::call(self.0, "query_token_holders_v2", (arg0,arg1,arg2,)).await
  }
  pub async fn query_user_by_random(&self, arg0: u64) -> Result<
    (Option<Principal>,)
  > { ic_cdk::call(self.0, "query_user_by_random", (arg0,)).await }
  pub async fn query_user_create_meme_tokens(
    &self,
    arg0: Option<Principal>,
  ) -> Result<(Vec<MemeToken>,)> {
    ic_cdk::call(self.0, "query_user_create_meme_tokens", (arg0,)).await
  }
  pub async fn query_user_lp(&self, arg0: Option<Principal>) -> Result<
    (Vec<LiquidityProviderView>,)
  > { ic_cdk::call(self.0, "query_user_lp", (arg0,)).await }
  pub async fn query_user_lp_earning(
    &self,
    arg0: Option<Principal>,
    arg1: u64,
  ) -> Result<(candid::Nat,)> {
    ic_cdk::call(self.0, "query_user_lp_earning", (arg0,arg1,)).await
  }
  pub async fn query_user_lp_v_2(&self, arg0: Option<Principal>) -> Result<
    (Vec<LiquidityProviderV2View>,)
  > { ic_cdk::call(self.0, "query_user_lp_v2", (arg0,)).await }
  pub async fn query_user_meme_token_lp(
    &self,
    arg0: Option<Principal>,
    arg1: u64,
  ) -> Result<(candid::Nat,)> {
    ic_cdk::call(self.0, "query_user_meme_token_lp", (arg0,arg1,)).await
  }
  pub async fn query_user_meme_token_lp_v_2(
    &self,
    arg0: Option<Principal>,
    arg1: u64,
  ) -> Result<(Liquidity,)> {
    ic_cdk::call(self.0, "query_user_meme_token_lp_v2", (arg0,arg1,)).await
  }
  pub async fn query_user_tokens(&self, arg0: Option<Account>) -> Result<
    (Vec<MemeTokenBalance>,)
  > { ic_cdk::call(self.0, "query_user_tokens", (arg0,)).await }
  pub async fn query_user_tokens_v_2(
    &self,
    arg0: Option<Account>,
    arg1: Option<MemeTType>,
  ) -> Result<(Vec<MemeTokenBalanceV2>,)> {
    ic_cdk::call(self.0, "query_user_tokens_v2", (arg0,arg1,)).await
  }
  pub async fn query_withdrawing_brc(&self) -> Result<(Vec<WithdrawBrcInfo>,)> {
    ic_cdk::call(self.0, "query_withdrawing_brc", ()).await
  }
  pub async fn runes_swap_sats(&self, arg0: RunesSwapSatsArg) -> Result<
    (Result_,)
  > { ic_cdk::call(self.0, "runes_swap_sats", (arg0,)).await }
  pub async fn sats_swap_runes(&self, arg0: SatsSwapRunesArg) -> Result<
    (Result_,)
  > { ic_cdk::call(self.0, "sats_swap_runes", (arg0,)).await }
  pub async fn sell(&self, arg0: BuyArgs) -> Result<(Result_,)> {
    ic_cdk::call(self.0, "sell", (arg0,)).await
  }
  pub async fn token_balance(&self) -> Result<(candid::Nat,)> {
    ic_cdk::call(self.0, "token_balance", ()).await
  }
  pub async fn token_balance_2(
    &self,
    arg0: u64,
    arg1: u64,
    arg2: Option<Principal>,
  ) -> Result<(Vec<(Account,candid::Nat,)>,)> {
    ic_cdk::call(self.0, "token_balance2", (arg0,arg1,arg2,)).await
  }
  pub async fn unlock_lp(&self, arg0: Vec<u64>) -> Result<(Vec<Result13>,)> {
    ic_cdk::call(self.0, "unlock_lp", (arg0,)).await
  }
  pub async fn unlock_tokens(&self, arg0: Vec<u64>) -> Result<
    (Vec<Result13>,)
  > { ic_cdk::call(self.0, "unlock_tokens", (arg0,)).await }
  pub async fn update_maintainers(&self, arg0: Vec<Principal>) -> Result<
    (Result1,)
  > { ic_cdk::call(self.0, "update_maintainers", (arg0,)).await }
  pub async fn update_maintenance(&self, arg0: bool, arg1: bool) -> Result<
    (Result1,)
  > { ic_cdk::call(self.0, "update_maintenance", (arg0,arg1,)).await }
  pub async fn update_meme_token_info(
    &self,
    arg0: UpdateMemeTokenInfoArg,
  ) -> Result<(Result1,)> {
    ic_cdk::call(self.0, "update_meme_token_info", (arg0,)).await
  }
  pub async fn withdraw(&self, arg0: WithdrawArgs) -> Result<(Result_,)> {
    ic_cdk::call(self.0, "withdraw", (arg0,)).await
  }
  pub async fn withdraw_brc(&self, arg0: WithdrawBrcArg) -> Result<(Result1,)> {
    ic_cdk::call(self.0, "withdraw_brc", (arg0,)).await
  }
  pub async fn withdraw_ckbtc(&self, arg0: WithdrawByCkbtcArgs) -> Result<
    (Result5,)
  > { ic_cdk::call(self.0, "withdraw_ckbtc", (arg0,)).await }
  pub async fn withdraw_liquidity(&self, arg0: LiquidityRemoveArg) -> Result<
    (Result8,)
  > { ic_cdk::call(self.0, "withdraw_liquidity", (arg0,)).await }
  pub async fn withdraw_pool_income(&self, arg0: u64) -> Result<(Result_,)> {
    ic_cdk::call(self.0, "withdraw_pool_income", (arg0,)).await
  }
  pub async fn withdraw_pool_lp_earning(&self, arg0: u64) -> Result<
    (Result_,)
  > { ic_cdk::call(self.0, "withdraw_pool_lp_earning", (arg0,)).await }
  pub async fn withdraw_rewards(&self, arg0: WithdrawRewardsArgs) -> Result<
    (Result_,)
  > { ic_cdk::call(self.0, "withdraw_rewards", (arg0,)).await }
  pub async fn withdraw_rune(&self, arg0: WithdrawRuneArgs) -> Result<
    (Result1,)
  > { ic_cdk::call(self.0, "withdraw_rune", (arg0,)).await }
}
