use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::caller;
use ic_cdk::api::management_canister::http_request::{http_request, CanisterHttpRequestArgument};
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};
use serde_json::Value;
use ic_cdk::api::time;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use ic_cdk_timers::set_timer_interval;
use std::time::Duration;

mod ic;
use ic_cdk::api::management_canister::bitcoin::{
    bitcoin_get_balance, bitcoin_get_current_fee_percentiles, bitcoin_get_utxos,
    bitcoin_send_transaction, BitcoinNetwork, GetBalanceRequest, GetCurrentFeePercentilesRequest,
    GetUtxosRequest, GetUtxosResponse, MillisatoshiPerByte, SendTransactionRequest,
};
use ic_cdk::api::management_canister::schnorr::{
    SchnorrAlgorithm, SchnorrKeyId, SignWithSchnorrArgument,
};
use ic::bitcoin_api::send_transaction;
use bitcoin::{
    key::{PublicKey, Secp256k1},
    secp256k1::{schnorr, XOnlyPublicKey},
    Address, Amount, Network, ScriptBuf, Sequence, TapSighashType, TxOut,
};


mod psbt;
mod alkanes;
mod did;

pub use psbt::{
    builder::PsbtBuilder,
    transaction::{combine_psbt, create_transaction_multi},
    types::{InputSignatureType, TransactionInput, TransactionOutput, TransactionResult},
    gas::{calculate_fee_simple, calculate_fee_with_opreturn},
};
use crate::alkanes::alkanes_data::alkanes_protostone::{Protostone, Edict, RuneId, build_alkanes_transfer_script};

use crate::alkanes::alkanes_storage::{
    add_white_token, batch_upload, clear, get_all, get_white_tokens,
    init as storage_init, is_white_token, post_upgrade as storage_post_upgrade,
    pre_upgrade as storage_pre_upgrade, remove_white_token, alkanes_query, set_owner, AlkaneRecord,
    set_token_id_mapping, get_token_id_by_alkaneid, get_alkane_fund_utxo, get_all_utxos, 
    AlkaneUtxoRecord, get_utxos_by_address, set_utxo, remove_utxo, get_utxos_by_alkaneid, utxo_count
};

use crate::did::fomowell_token::{CreateMemeTokenArg, MemeTokenType, Service, InternalTransferArg, Account, LedgerType};
use crate::did::user_canister_did::Service as UserCanisterService;


const IC_BITCOIN_NETWORK: ic_cdk::api::management_canister::bitcoin::BitcoinNetwork =
    ic_cdk::api::management_canister::bitcoin::BitcoinNetwork::Testnet;

const SCHNORR_KEY_NAME: &str = "test_key_1";
const FOMOWELLL_MAINNET_CANISTER_ID: &str = "fw4iq-diaaa-aaaah-arela-cai";
const FEE_RATE_CANISTER_ID: &str = "kqs64-paaaa-aaaar-qamza-cai";

const fomowell_alkanes_topup_address: &str = "fomowell-alkanes-topup-address";
const fomowell_alkanes_fund_address: &str = "fomowell-alkanes-fund-address";
const fomowell_btc_address: &str = "fomowell-btc-address";


thread_local! {
    static ADDRESSES: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
    static PROCESSED_TRANSACTIONS: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
    static PENDING_TX_ID: RefCell<Option<String>> = RefCell::new(None); //假设一笔
    static WITHDRAW_REQUESTS: RefCell<HashMap<Principal, Vec<WithdrawRequest>>> = RefCell::new(HashMap::new());
    static LOGS: RefCell<Vec<LogEntry>> = RefCell::new(Vec::new());
}

#[init]
async fn init() {
    let owner: Principal = Principal::from_text("tvz33-ke3fp-pkev4-7zlcz-e6la2-nuoxp-ogkve-udz64-65zrs-rr34c-5qe").unwrap();
    storage_init(owner);

    let in_fomowell_alkanes_topup_address = generate_address(fomowell_alkanes_topup_address.to_string()).await;
    let in_fomowell_alkans_fund_address = generate_address(fomowell_alkanes_fund_address.to_string()).await;
    let in_fomowell_btc_address = generate_address(fomowell_btc_address.to_string()).await;

    ADDRESSES.with(|addrs| {
        let mut map = addrs.borrow_mut();
        map.insert("alkanes_topup".to_string(), in_fomowell_alkanes_topup_address);
        map.insert("alkanes_fund".to_string(), in_fomowell_alkans_fund_address);
        map.insert("btc".to_string(), in_fomowell_btc_address);
    });
    set_timer_interval(Duration::from_secs(36000), || {
        ic_cdk::spawn(gather_alkanes_utxo_timer());
    });
    set_timer_interval(Duration::from_secs(7200), || {
        ic_cdk::spawn(check_withdraw_request());
    });
}


#[pre_upgrade]
fn pre_upgrade_hook() {
    storage_pre_upgrade();
}

#[post_upgrade]
async fn post_upgrade_hook() {
    storage_post_upgrade();
    let in_fomowell_alkanes_topup_address = generate_address(fomowell_alkanes_topup_address.to_string()).await;
    let in_fomowell_alkanes_fund_address = generate_address(fomowell_alkanes_fund_address.to_string()).await;
    let in_fomowell_btc_address = generate_address(fomowell_btc_address.to_string()).await;
    ADDRESSES.with(|addrs| {
        let mut map = addrs.borrow_mut();
        map.insert("alkanes_topup".to_string(), in_fomowell_alkanes_topup_address);
        map.insert("alkanes_fund".to_string(), in_fomowell_alkanes_fund_address);
        map.insert("btc".to_string(), in_fomowell_btc_address);
    });
}
#[derive(CandidType, Deserialize, Clone)]
pub struct EdictInput {
    pub block: u64,
    pub tx: u32,
    pub amount: u128,
    pub output: u32,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct LogEntry {
    pub timestamp_nanos: u64,
    pub event: String,
}

fn append_log(event: impl Into<String>) {
    const MAX_LOGS: usize = 500;
    LOGS.with(|logs| {
        let mut vec = logs.borrow_mut();
        vec.push(LogEntry {
            timestamp_nanos: time(),
            event: event.into(),
        });
        if vec.len() > MAX_LOGS {
            let overflow = vec.len() - MAX_LOGS;
            vec.drain(0..overflow);
        }
    });
}

pub fn generate_protostone(edicts: Vec<EdictInput>) -> (ScriptBuf) {
    let edicts: Vec<Edict> = edicts
        .into_iter()
        .map(|e| Edict {
            id: RuneId {
                block: e.block,
                tx: e.tx,
            },
            amount: e.amount,
            output: e.output,
        })
        .collect();

    // 默认utxo放在0位置，0位置是保险设计
    let proto = Protostone {
        subprotocol_id: 1, 
        edicts,
        pointer: Some(0),
        refund_pointer: Some(0),
        burn: None,
        message: None,
        from: None,
    };

    let script = build_alkanes_transfer_script(&proto);

    script
}

#[update]
async fn topup_alkanes(txid: String) -> Result<String, String> {
    if PROCESSED_TRANSACTIONS.with(|set| set.borrow().contains(&txid)) {
        return Ok("Transaction already processed".into());
    }// 防止重放攻击
    match get_alkane(txid.clone()) {
        Ok(record) => {
            let user_canister_id = Principal::from_text("a7ady-jiaaa-aaaah-arexa-cai")
                .map_err(|e| format!("Invalid canister ID: {}", e))?;
            
            let user_service = UserCanisterService(user_canister_id);
            
            let mut all_deposits = Vec::new();
            let page_size = 500u64;
            let mut offset = 0u64;
            
            loop {
                match user_service.query_list_deposits_paginated(offset, offset + page_size).await {
                    Ok((deposits,)) => {
                        if deposits.is_empty() {
                            break;
                        }
                        all_deposits.extend(deposits);
                        offset += page_size;
                    }
                    Err((code, msg)) => {
                        return Err(format!("Failed to query deposits: {:?}, {}", code, msg));
                    }
                }
            }
            
            let caller_pid = caller();
            let found_deposit = all_deposits.iter().find(|d| d.pid == caller_pid);
            
            match found_deposit {
                Some(deposit) => {
                    if deposit.address != record.send_address {
                        return Err(format!("BTC address mismatch: expected {}, got {}", record.send_address, deposit.address));
                    }
                    append_log(format!("[topup] BTC address  txid={} caller_pid={} address={}", 
                        txid, caller_pid, deposit.address));
                }
                None => {
                    append_log(format!("[topup] Caller deposit not found txid={} caller_pid={}", txid, caller_pid));
                    return Err(format!("Caller deposit not found for principal {}", caller_pid));
                }
            }

            let meme_token_id = get_token_id_by_alkaneid(record.alkaneid.clone())
                .map_err(|e| format!("Failed to get meme_token_id: {}", e))?;
            let token_canister_id = Principal::from_text(FOMOWELLL_MAINNET_CANISTER_ID)
                .map_err(|e| format!("Invalid canister ID: {}", e))?;
            let service = Service(token_canister_id);
            
            let to_account = Account {
                owner: caller(),
                subaccount: None,
            };
            
            let transfer_arg = InternalTransferArg {
                to: to_account,
                lock_id: None,
                subaccount: None,
                ledger_type: LedgerType::MemeToken(meme_token_id),
                amount: candid::Nat::from(record.amount),
            };
            
            match service.internal_transfer(transfer_arg).await {
                Ok((result,)) => {
                    match result {
                        crate::did::fomowell_token::Result1::Ok => {
                            PROCESSED_TRANSACTIONS.with(|set| set.borrow_mut().insert(txid.clone()));
                            append_log(format!("[topup] success txid={} amount={} alkaneid={}", txid, record.amount, record.alkaneid));
                            Ok(format!("Topup successful: {} tokens transferred for txid {}", record.amount, txid))
                        }
                        crate::did::fomowell_token::Result1::Err(e) => {
                            append_log(format!("[topup] internal_transfer failed txid={} error={}", txid, e));
                            Err(format!("Internal transfer failed: {}", e))
                        }
                    }
                }
                Err(e) => {
                    append_log(format!("[topup] call internal_transfer error txid={} err={:?}", txid, e));
                    Err(format!("Failed to call internal_transfer: {:?}", e))
                },
            }
        }
        Err(e) => {
            append_log(format!("[topup] record not found txid={} err={}", txid, e));
            Err(e)
        },
    }
}



#[derive(CandidType, Deserialize, Clone)]
pub struct WithdrawRequest {
    ic_txid: String,
    token_type: String,
    token_id: String,
    token_amount: u64,
    withdraw_address: String,
}
#[update]
async fn withdraw_alkanes(withdraw_request: WithdrawRequest) -> Result<String, String> {
    // if PROCESSED_TRANSACTIONS.with(|set| set.borrow().contains(&withdraw_request.ic_txid)) {
    //     return Ok("Transaction already processed".into());
    // }

    let pid = caller();
    let allowed_canister = Principal::from_text("fw4iq-diaaa-aaaah-arela-cai")
        .map_err(|e| format!("Invalid allowed canister ID: {}", e))?;
    if pid != allowed_canister {
        return Err("Unauthorized: Only fomowell canister can call this function".to_string());
    }

    let is_full = WITHDRAW_REQUESTS.with(|requests| {
        requests.borrow().len() >= 10
    });
    if is_full {
        append_log("[withdraw-queue] queue full");
        return Err("Withdraw request queue is full".to_string());
    }
    
    WITHDRAW_REQUESTS.with(|requests| {
        let mut requests_map = requests.borrow_mut();
        requests_map
            .entry(pid)
            .or_insert_with(Vec::new)
            .push(withdraw_request.clone());
    });
    append_log(format!("[withdraw-queue] queued ic_txid={} for caller={}", withdraw_request.ic_txid, pid));
    Ok("Withdraw request submitted".into())
}

async fn check_withdraw_request()  {
    let PREVIOUS_TX_ID = match PENDING_TX_ID.with(|id| id.borrow().clone()) {
        Some(txid) => txid,
        None => {
            return;
        }
    };    
    append_log(format!("[withdraw-check] checking txid={}", PREVIOUS_TX_ID));
    match check_tx_confirmed(PREVIOUS_TX_ID.clone()).await {
        Ok(true) => {
            append_log(format!("[withdraw-check] confirmed txid={}", PREVIOUS_TX_ID));
            let withdraw_requests = WITHDRAW_REQUESTS.with(|requests| {
                requests.borrow().clone()
            });   
            if let Err(e) = send_withdraw_request(withdraw_requests.clone()).await {
                append_log(format!("send_withdraw_request error={}", e));
            }
        }
        Ok(false) => {
            append_log(format!("[withdraw-check] not confirmed txid={}", PREVIOUS_TX_ID));
            return;
        }
        Err(e) => {
            append_log(format!("[withdraw-check] check_tx_confirmed error txid={} err={}", PREVIOUS_TX_ID, e));
            return;
        }
    }
}


async fn send_withdraw_request(withdraw_alkanes: HashMap<Principal, Vec<WithdrawRequest>>) -> Result<String, String> {
    let keys = ["alkanes_topup", "alkanes_fund", "btc"];
    let [alkanes_topup_address, alkanes_fund_address, alkanes_btc_address] =
        keys.map(|k| get_address(k.to_string()).unwrap());
    
    let required_alkanes: HashMap<String, u64> = withdraw_alkanes
        .values()
        .flatten()
        .fold(HashMap::new(), |mut acc, req| {
            *acc.entry(req.token_id.clone()).or_insert(0) += req.token_amount;
            acc
        });
    let required_alkane_ids: HashSet<String> = required_alkanes.keys().cloned().collect();
    let alkanes_fund_address = get_address("alkanes_fund".to_string()).unwrap();
    
    let all_utxos: Vec<(String, String, AlkaneUtxoRecord)> = 
        get_utxos_by_address(alkanes_fund_address.clone())
            .into_iter()
            .filter(|(_, alkaneid, _)| required_alkane_ids.contains(alkaneid))
            .collect();

    // 检查总余额是否足够
    let total_alkane_amounts: HashMap<String, u64> = all_utxos
        .iter()
        .fold(HashMap::new(), |mut acc, (_, alkaneid, record)| {
            *acc.entry(alkaneid.clone()).or_insert(0) += record.amount;
            acc
        });
    
    if let Some((alkaneid, required_amount)) = required_alkanes
        .iter()
        .find(|(alkaneid, required)| { 
            total_alkane_amounts.get(*alkaneid).copied().unwrap_or(0) < **required
        })
    {
        let available = total_alkane_amounts.get(alkaneid).copied().unwrap_or(0);
        return Err(format!(
            "Insufficient alkanes for {}: required {}, available {}",
            alkaneid, required_amount, available
        ));
    }

    let mut selected_utxos: Vec<(String, String, AlkaneUtxoRecord)> = Vec::new();
    let mut selected_alkane_amounts: HashMap<String, u64> = HashMap::new();
    
    for (alkaneid, required_amount) in &required_alkanes {
        let mut alkane_utxos: Vec<(String, String, AlkaneUtxoRecord)> = all_utxos
            .iter()
            .filter(|(_, aid, _)| aid == alkaneid)
            .map(|(a, aid, r)| (a.clone(), aid.clone(), r.clone()))
            .collect();
        alkane_utxos.sort_by(|(_, _, r1), (_, _, r2)| r2.amount.cmp(&r1.amount));
        
        let mut selected_amount = 0u64;
        for utxo in alkane_utxos {
            if selected_amount >= *required_amount {
                break;
            }
            selected_utxos.push(utxo.clone());
            selected_amount += utxo.2.amount;
        }
        
        selected_alkane_amounts.insert(alkaneid.clone(), selected_amount);
        
        if selected_amount < *required_amount {
            return Err(format!(
                "Failed to select sufficient UTXOs for {}: required {}, selected {}",
                alkaneid, required_amount, selected_amount
            ));
        }
    }

    // 使用选中的UTXO金额计算找零
    let fund_summary: Vec<(String, u64)> = selected_alkane_amounts
        .iter()
        .map(|(alkaneid, amount)| (alkaneid.clone(), *amount))
        .collect();

    let mut edict_inputs: Vec<EdictInput> = withdraw_alkanes
        .values()
        .flatten()  
        .enumerate()
        .filter_map(|(idx, request)| {
            let (block_str, tx_str) = request.token_id.split_once(':')?;
            let block = block_str.parse::<u64>().ok()?;
            let tx = tx_str.parse::<u32>().ok()?;
            
            Some(EdictInput {
                block,
                tx,
                amount: request.token_amount as u128,
                output: (idx + 1) as u32,
            })
        })
        .collect();

    // 构建找零数据：使用选中的UTXO金额计算找零
    let change_edicts: Vec<EdictInput> = selected_alkane_amounts
        .iter()
        .filter_map(|(alkaneid, &selected_amount)| {
            let required_amount = required_alkanes.get(alkaneid).copied().unwrap_or(0);
            let change_amount = selected_amount.saturating_sub(required_amount);
            
            if change_amount > 0 {
                let (block_str, tx_str) = alkaneid.split_once(':')?;
                let block = block_str.parse::<u64>().ok()?;
                let tx = tx_str.parse::<u32>().ok()?;
                
                Some(EdictInput {
                    block,
                    tx,
                    amount: change_amount as u128,
                    output: 0,
                })
            } else {
                None
            }
        })
        .collect();

    edict_inputs.extend(change_edicts);
    
    let protostone_script = generate_protostone(edict_inputs);

    let fund_public_key = ic::schnorr_api::schnorr_public_key(
        SCHNORR_KEY_NAME.to_string(),
        get_derivation_path(fomowell_alkanes_fund_address),
    )
    .await;
    let fund_public_key = hex::encode(fund_public_key);

    let mut inputs: Vec<TransactionInput> = Vec::new();
    for (_, _, utxo_record) in &selected_utxos {
        inputs.push(TransactionInput {
            txid: utxo_record.txid.clone(),
            vout: utxo_record.vout as u32, 
            amount: utxo_record.satoshi, 
            address: alkanes_fund_address.clone(),
            public_key: Some(fund_public_key.clone()),
            signature_type: Some(InputSignatureType::from_str("taproot_default").unwrap()),
            witness: None,
        });
    }
    
    
    let mut outputs: Vec<TransactionOutput> = Vec::new();
    outputs.push(TransactionOutput {
        address: alkanes_fund_address.clone(),
        amount: 330,
        op_return: None,
    });
    let withdraw_outputs: Vec<TransactionOutput> = withdraw_alkanes
        .values()
        .flatten()
        .map(|request| {
            TransactionOutput {
                address: request.withdraw_address.clone(),
                amount: 330,
                op_return: None,
            }
        })
        .collect();
    outputs.extend(withdraw_outputs);

    outputs.push(TransactionOutput {
        address: alkanes_fund_address.clone(),
        amount: 0,
        op_return: Some(protostone_script.as_bytes().to_vec()),
    });

    let fee_rate = get_feerate().await.unwrap();

    let total_input: u64 = inputs.iter().map(|input| input.amount).sum();
    let total_output: u64 = outputs.iter().map(|output| output.amount).sum();
    
    let fee = calculate_fee_with_opreturn(inputs.len(), outputs.len() + 1 , protostone_script.as_bytes().len(), fee_rate.parse::<f64>().unwrap());

    let btc_utxos = get_btc_utxos(alkanes_btc_address.clone()).await.unwrap();

    let btc_public_key = ic::schnorr_api::schnorr_public_key(
        SCHNORR_KEY_NAME.to_string(),
        get_derivation_path(fomowell_btc_address),
    )
    .await;
    let btc_public_key_hex = hex::encode(btc_public_key);
    let mut total_value = 0u64;
    let mut btc_inputs: Vec<TransactionInput> = Vec::new();
    for utxo in &btc_utxos {
        btc_inputs.push(TransactionInput {
            txid: utxo.txid.clone(),
            vout: utxo.vout,
            amount: utxo.value,
            address: alkanes_btc_address.clone(),
            public_key: Some(btc_public_key_hex.clone()),
            signature_type: Some(InputSignatureType::from_str("taproot_default").unwrap()),
            witness: None,
        });
        total_value += utxo.value;
        if total_value  + total_input >= fee + total_output + 2000 {
            break;
        }
    }
    inputs.extend(btc_inputs.clone());
    let new_fee = calculate_fee_with_opreturn(inputs.len(), outputs.len() + 1 , protostone_script.as_bytes().len(), fee_rate.parse::<f64>().unwrap());
    let change = inputs.iter().map(|input| input.amount).sum::<u64>()
    .saturating_sub(outputs.iter().map(|output| output.amount).sum::<u64>())
    .saturating_sub(new_fee);
    if  change > 330 {
        outputs.push(TransactionOutput {
            address: alkanes_btc_address.clone(),
            amount: change,
            op_return: None,
        });
    }
    match create_transaction_multi(
        "testnet",
        inputs,
        outputs,
        btc_inputs,
        fomowell_alkanes_fund_address,
    )
    .await
    {
        Ok(final_psbt) => {
            let final_hex = final_psbt.psbt_hex.clone();
            let transaction_bytes = hex::decode(&final_hex)
            .map_err(|e| format!("Failed to decode transaction hex: {}", e))?;
            send_transaction(IC_BITCOIN_NETWORK, transaction_bytes).await;
            let txid = final_psbt.txid.clone();
            PENDING_TX_ID.with(|id| {
                *id.borrow_mut() = Some(txid.clone());
            });
            append_log(format!("[withdraw-send] broadcast txid={} ", txid));
            Ok(txid)
        }
        Err(err) => {
            append_log(format!("[withdraw-send] create tx failed err={:?}", err));
            return Err(format!("cant create final tx !"));
        }
    }
}

async fn gather_alkanes_utxo() -> Result<String, String> {
        let keys = ["alkanes_topup", "alkanes_fund", "btc"];
        let [alkanes_topup_address, alkanes_fund_address, alkanes_btc_address] =
            keys.map(|k| get_address(k.to_string()).unwrap());
    
        let alkanes_topup_utxo = get_utxos_by_address(alkanes_topup_address.clone());
        let mut alkane_amounts: HashMap<String, u64> = HashMap::new();

        for (_, alkaneid, utxo_record) in &alkanes_topup_utxo {
            *alkane_amounts.entry(alkaneid.clone()).or_insert(0) += utxo_record.amount;
        }

        let alkanes_map: HashMap<String, u64> = alkane_amounts.clone();

        // 按照alkanes id构建alkanes data
        // 后续优化设计，可以将utxo只生成一个
        let edict_inputs: Vec<EdictInput> = alkanes_map
        .iter()
        .enumerate()
        .filter_map(|(idx, (alkaneid, amount))| {
            let (block_str, tx_str) = alkaneid.split_once(':')?;
            let block = block_str.parse::<u64>().ok()?;
            let tx = tx_str.parse::<u32>().ok()?;
            
            Some(EdictInput {
                block,
                tx,
                amount: *amount as u128,
                output: (idx + 1) as u32, // output 从 1 开始,0是保险设计
            })
        })
        .collect();
        
        let protostone_script = generate_protostone(edict_inputs);
    
        let topup_public_key = ic::schnorr_api::schnorr_public_key(
            SCHNORR_KEY_NAME.to_string(),
            get_derivation_path(fomowell_alkanes_topup_address),
        )
        .await;
        let topup_public_key = hex::encode(topup_public_key);

        let mut inputs: Vec<TransactionInput> = Vec::new();
        for (address, _alkaneid, utxo_record) in &alkanes_topup_utxo {
            inputs.push(TransactionInput {
                txid: utxo_record.txid.clone(),
                vout: utxo_record.vout as u32, 
                amount: utxo_record.satoshi, 
                address: alkanes_topup_address.clone(),
                public_key: Some(topup_public_key.clone()),
                signature_type: Some(InputSignatureType::from_str("taproot_default").unwrap()),
                witness: None,
            });
        }
        
        // 构建outputs，
        let mut outputs: Vec<TransactionOutput> = Vec::new();
        // 根据alkanes_map中alkanesid的数量，构建n+1个output，地址是alkanes_fund_address
        for (alkaneid, amount) in &alkanes_map {
            outputs.push(TransactionOutput {
                address: alkanes_fund_address.clone(),
                amount: 330,
                op_return: None,
            });
        }
        outputs.push(TransactionOutput {
            address: alkanes_fund_address.clone(),
            amount: 0,
            op_return: Some(protostone_script.as_bytes().to_vec()),
        });

        let fee_rate = get_feerate().await.unwrap();
        let fee = calculate_fee_with_opreturn(inputs.len(), outputs.len() +1, protostone_script.as_bytes().len(), fee_rate.parse::<f64>().unwrap());

        let btc_utxos = get_btc_utxos(alkanes_btc_address.clone()).await.unwrap();

        let btc_public_key = ic::schnorr_api::schnorr_public_key(
            SCHNORR_KEY_NAME.to_string(),
            get_derivation_path(fomowell_btc_address),
        )
        .await;
        let btc_public_key_hex = hex::encode(btc_public_key);
        let mut total_value = 0u64;
        let mut btc_inputs: Vec<TransactionInput> = Vec::new();
        for utxo in &btc_utxos {
            btc_inputs.push(TransactionInput {
                txid: utxo.txid.clone(),
                vout: utxo.vout,
                amount: utxo.value,
                address: fomowell_btc_address.to_string(),
                public_key: Some(btc_public_key_hex.clone()),
                signature_type: Some(InputSignatureType::from_str("taproot_default").unwrap()),
                witness: None,
            });
            total_value += utxo.value;
            if total_value >= fee {
                break;
            }
        }
        inputs.extend(btc_inputs.clone());
        let new_fee = calculate_fee_with_opreturn(inputs.len(), outputs.len() + 1 , protostone_script.as_bytes().len(), fee_rate.parse::<f64>().unwrap());
        let change = inputs.iter().map(|input| input.amount).sum::<u64>()
            .saturating_sub(outputs.iter().map(|output| output.amount).sum::<u64>())
            .saturating_sub(new_fee);
        if  change > 330 {
            outputs.push(TransactionOutput {
                address: alkanes_btc_address.clone(),
                amount: change,
                op_return: None,
            });
        }
        let final_hex: String;
        match create_transaction_multi(
            "testnet",
            inputs,
            outputs,
            btc_inputs,
            fomowell_alkanes_fund_address,
        )
        .await
        {
            Ok(final_psbt) => {
                final_hex = final_psbt.psbt_hex.clone();
                let transaction_bytes = hex::decode(&final_hex)
                .map_err(|e| format!("Failed to decode transaction hex: {}", e))?;
                send_transaction(IC_BITCOIN_NETWORK, transaction_bytes).await;
                let txid = final_psbt.txid.clone();
                PENDING_TX_ID.with(|id| {
                    *id.borrow_mut() = Some(txid.clone());
                });
                WITHDRAW_REQUESTS.with(|requests| {
                    requests.borrow_mut().clear();
                });
                append_log(format!("[gather] broadcast txid={} ",txid));
                Ok(txid)
            }
            Err(err) => {
                append_log(format!("[gather] create tx failed err={:?}", err));
                return Err(format!("cant create final tx !"));
            }
        }
}


async fn gather_alkanes_utxo_timer() {
    if let Err(e) = gather_alkanes_utxo().await {
        append_log(format!("gather_alkanes_utxo error={}", e));
    }
}

async fn check_tx_confirmed(txid: String) -> Result<bool, String> {
    let url = format!("https://mempool.space/api/tx/{}/status", txid);
    let request = CanisterHttpRequestArgument {
        url: url.clone(),
        max_response_bytes: Some(10_000_000),
        method: ic_cdk::api::management_canister::http_request::HttpMethod::GET,
        headers: vec![],
        body: None,
        transform: None,
    };

    let cycles: u128 = 2_000_000_000;

    match http_request(request, cycles).await {
        Ok((response,)) => {
            let body_str = String::from_utf8(response.body)
                .map_err(|e| format!("Failed to parse response body: {}", e))?;
            
            let json: Value = serde_json::from_str(&body_str)
                .map_err(|e| format!("Failed to parse JSON: {}", e))?;

            json["confirmed"]
                .as_bool()
                .ok_or_else(|| {
                    format!(
                        "Missing or invalid confirmed field in response: {}",
                        body_str
                    )
                })
                .map(|confirmed| {
                    append_log(format!("[check-tx] txid={} confirmed={}", txid, confirmed));
                    confirmed
                })
        }
        Err((code, msg)) => {
            append_log(format!("[check-tx] http error txid={} code={:?} msg={}", txid, code, msg));
            Err(format!("HTTP request failed: {:?}, {}", code, msg))
        }
    }
}

#[derive(CandidType, Deserialize, Clone)]
pub struct UtxoInfo {
    value: u64,
    txid: String,
    vout: u32,
}

#[update]
async fn get_btc_utxos(address: String) -> Result<Vec<UtxoInfo>, String> {
    let network = IC_BITCOIN_NETWORK;
    let filter = None;

    let query_btc_res: Result<(GetUtxosResponse,), (ic_cdk::api::call::RejectionCode, String)> =
        bitcoin_get_utxos(GetUtxosRequest {
            address: address.clone(),
            network,
            filter,
        })
        .await;

    match query_btc_res {
        Ok((get_utxos_response,)) => {
            let utxos = get_utxos_response.utxos;
            let mut utxo_info_list: Vec<UtxoInfo> = Vec::new();

            for utxo in utxos {
                let value = utxo.value;
                let txid = hex::encode(utxo.outpoint.txid);
                let vout = utxo.outpoint.vout;
                let utxo_info = UtxoInfo { value, txid, vout };
                utxo_info_list.push(utxo_info);
            }
            
            utxo_info_list.sort_by(|a, b| b.value.cmp(&a.value));
            
            Ok(utxo_info_list)
        }
        Err((code, msg)) => {
            Err(format!("Failed to get UTXOs for address {}: {:?}, {}", address, code, msg))
        }
    }
}

// Tools
fn bitcoin_network_to_ic_bitcoin_network(
    network: Network,
) -> ic_cdk::api::management_canister::bitcoin::BitcoinNetwork {
    match network {
        Network::Bitcoin => ic_cdk::api::management_canister::bitcoin::BitcoinNetwork::Mainnet,
        Network::Testnet => ic_cdk::api::management_canister::bitcoin::BitcoinNetwork::Testnet,
        Network::Testnet4 => ic_cdk::api::management_canister::bitcoin::BitcoinNetwork::Testnet,
        Network::Signet => ic_cdk::api::management_canister::bitcoin::BitcoinNetwork::Testnet,
        Network::Regtest => ic_cdk::api::management_canister::bitcoin::BitcoinNetwork::Regtest,
        _ => ic_cdk::api::management_canister::bitcoin::BitcoinNetwork::Testnet,
    }
}

async fn generate_address(input: String) -> String {
    let address = ic::p2tr_key_only::get_address(
        IC_BITCOIN_NETWORK,
        SCHNORR_KEY_NAME.to_string(),
        get_derivation_path(&input),
    )
    .await;
    address.to_string()
}


fn get_derivation_path(input: &str) -> Vec<Vec<u8>> {
    vec![input.as_bytes().to_vec()]
}


/// 地址类型：`"alkanes_topup"`, `"alkanes_fund"`, 或 `"btc"`
#[query]
fn get_address(address_type: String) -> Result<String, String> {
    ADDRESSES.with(|addrs| {
        addrs.borrow()
            .get(&address_type)
            .cloned()
            .ok_or_else(|| format!("Address type '{}' not initialized", address_type))
    })
}


async fn get_feerate() -> Result<String, String> {
    let fee_canister =
        crate::did::fee_rate_canister_did::Service(Principal::from_text(FEE_RATE_CANISTER_ID).unwrap());
    let query_fee_rate__res = fee_canister.get_mempool_tx_fee_rate().await;
    let mut current_fee_rate = 0.0;

    match query_fee_rate__res {
        Ok(result_tuple) => {
            let fee_rate_view = result_tuple.0;
            current_fee_rate = (fee_rate_view.high) as f64;
        }
        Err((code, msg)) => {
            return Err(format!("Error get mempool fee_rate: {:?}, {}", code, msg));
        }
    }
    Ok(current_fee_rate.to_string())
}

// Query

#[update]
async fn set_owner_ic(new_owner: Principal) -> Result<String, String> {
    set_owner(new_owner)
}

#[update]
async fn upload_alkanes(batch: Vec<AlkaneRecord>) -> Result<String, String> {
    batch_upload(batch)
}

#[query]
fn get_alkane(txid: String) -> Result<AlkaneRecord, String> {
    alkanes_query(txid)
}

#[query]
fn list_alkanes() -> Vec<AlkaneRecord> {
    get_all()
}


#[update]
async fn clear_alkanes() -> Result<String, String> {
    clear()
}



#[update]
async fn add_white_token_ic(token: String) -> Result<String, String> {
    if is_white_token(token.clone()) {
        return Err("Token already in whitelist".into());
    }

    let url = format!("https://open-api.unisat.io/v1/indexer/alkanes/{}/info", token);
    let request = CanisterHttpRequestArgument {
        url: url.clone(),
        max_response_bytes: Some(10_000_000),
        method: ic_cdk::api::management_canister::http_request::HttpMethod::GET,
        headers: vec![],
        body: None,
        transform: None,
    };

    let cycles: u128 = 2_000_000_000;

    match http_request(request, cycles).await {
        Ok((response,)) => {
            let body_str = String::from_utf8(response.body)
                .map_err(|e| format!("Failed to parse response body: {}", e))?;
            
            let json: Value = serde_json::from_str(&body_str)
                .map_err(|e| format!("Failed to parse JSON: {}", e))?;

            if json["code"].as_u64() != Some(0) {
                return Err(format!("API error: {}", json["msg"].as_str().unwrap_or("Unknown error")));
            }

            let data = json["data"].as_object()
                .ok_or_else(|| "Missing data field".to_string())?;
            let token_data = data["tokenData"].as_object()
                .ok_or_else(|| "Missing tokenData field".to_string())?;

            let symbol = token_data["symbol"].as_str()
                .ok_or_else(|| "Missing symbol field".to_string())?
                .to_string();
            let name = token_data["name"].as_str()
                .ok_or_else(|| "Missing name field".to_string())?
                .to_string();

            let create_arg = CreateMemeTokenArg {
                creator: Some(caller()),
                ticker: symbol.clone(),
                logo_base64: None,
                twitter: None,
                logo: "".to_string(),
                name: name.clone(),
                description: format!("Alkanes token: {}", name),
                website: None,
                meme_token_type: MemeTokenType::Brc20(token.clone()),
                swap_fee_rate: None,
                dev_buy: Some(candid::Nat::from(0u64)),
                telegram: None,
            };

            let token_canister_id = Principal::from_text(FOMOWELLL_MAINNET_CANISTER_ID)
                .map_err(|e| format!("Invalid canister ID: {}", e))?;
            let service = Service(token_canister_id);
            
            match service.create_token(create_arg).await {
                Ok((result,)) => {
                    match result {
                        crate::did::fomowell_token::Result3::Ok(meme_token) => {
                            let meme_token_id = meme_token.id;
                            if let Err(e) = set_token_id_mapping(token.clone(), meme_token_id) {
                                return Err(format!("Failed to store token ID mapping: {}", e));
                            }
                            
                            match add_white_token(token.clone()) {
                                Ok(_) => {
                                    append_log(format!("[token-create] success token={} meme_token_id={}", token, meme_token_id));
                                    Ok(format!("Token {} created successfully with meme_token_id {}", token, meme_token_id))
                                },
                                Err(e) => {
                                    append_log(format!("[token-create] add whitelist failed token={} err={}", token, e));
                                    Err(format!("Token failed to add to whitelist: {}", e))
                                },
                            }
                        }
                        crate::did::fomowell_token::Result3::Err(e) => {
                            append_log(format!("[token-create] service returned error token={} err={}", token, e));
                            Err(format!("Failed to create token: {}", e))
                        }
                    }
                }
                Err(e) => {
                    append_log(format!("[token-create] call create_token failed token={} err={:?}", token, e));
                    Err(format!("Failed to call create_token: {:?}", e))
                },
            }
        }
        Err((code, message)) => {
            append_log(format!("[token-create] http error token={} code={:?} message={}", token, code, message));
            Err(format!("HTTP request failed: code={:?}, message={}", code, message))
        }
    }
}

#[query]
fn get_logs(offset: u64, limit: u64) -> Vec<LogEntry> {
    LOGS.with(|logs| {
        let vec = logs.borrow();
        let start = offset as usize;
        let end = (offset + limit).min(vec.len() as u64) as usize;
        if start >= vec.len() {
            Vec::new()
        } else {
            vec[start..end].to_vec()
        }
    })
}

#[update]
async fn clear_logs() -> Result<String, String> {
    LOGS.with(|logs| logs.borrow_mut().clear());
    Ok("logs cleared".into())
}


#[update]
async fn batch_upload_utxos(utxos: Vec<(String, String, AlkaneUtxoRecord)>) -> Result<String, String> {
    let mut uploaded = 0;
    let mut errors = Vec::new();
    
    for (address, alkaneid, utxo) in utxos {
        match set_utxo(address.clone(), alkaneid.clone(), utxo) {
            Ok(_) => uploaded += 1,
            Err(e) => errors.push(format!("Failed to upload UTXO for address: {}, alkaneid: {}, error: {}", address, alkaneid, e)),
        }
    }
    
    if errors.is_empty() {
        Ok(format!("Batch upload successful: {} UTXOs saved", uploaded))
    } else {
        Err(format!("Batch upload completed with errors. Success: {}, Errors: {}", uploaded, errors.join("; ")))
    }
}
#[query]
fn get_utxos_by_address_and_alkaneid(address: String, alkaneid: String) -> Vec<AlkaneUtxoRecord> {
    get_alkane_fund_utxo(address, alkaneid)
}

#[query]
fn get_all_utxos_ic() -> Vec<(String, String, AlkaneUtxoRecord)> {
    get_all_utxos()
}
#[update]
async fn batch_remove_utxos(utxos: Vec<(String, String, String, u64)>) -> Result<String, String> {
    let mut removed = 0;
    let mut errors = Vec::new();
    
    for (address, alkaneid, txid, vout) in utxos {
        match remove_utxo(address.clone(), alkaneid.clone(), txid.clone(), vout) {
            Ok(_) => removed += 1,
            Err(e) => errors.push(format!("Failed to remove UTXO for address: {}, alkaneid: {}, txid: {}, vout: {}, error: {}", 
                address, alkaneid, txid, vout, e)),
        }
    }
    
    if errors.is_empty() {
        Ok(format!("Batch remove successful: {} UTXOs removed", removed))
    } else {
        Err(format!("Batch remove completed with errors. Success: {}, Errors: {}", removed, errors.join("; ")))
    }
}


#[update]
async fn remove_white_token_ic(token: String) -> Result<String, String> {
    remove_white_token(token)
}

#[query]
fn get_white_tokens_ic() -> Vec<String> {
    get_white_tokens()
}


ic_cdk::export_candid!();
