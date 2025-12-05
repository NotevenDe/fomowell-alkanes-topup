use base64;
use bitcoin::sighash::SighashCache;
use hex;

use bitcoin::psbt::Psbt;
use bitcoin::{Address, Amount, Network, TapSighash, TxOut, Txid};

use crate::ic;
use std::str::FromStr;

use super::{
    builder::PsbtBuilder,
    types::{InputUtxo, TransactionInput, TransactionOutput, TransactionResult},
};
use bitcoin::FeeRate;

pub async fn create_transaction_multi(
    network_type: &str,
    inputs: Vec<TransactionInput>,
    outputs: Vec<TransactionOutput>,
    btc_inputs: Vec<TransactionInput>,
    token_path: &str, 
) -> Result<TransactionResult, String> {
    let network = match network_type.to_lowercase().as_str() {
        "mainnet" => Network::Bitcoin,
        "testnet" => Network::Testnet,
        "signet" => Network::Signet,
        "regtest" => Network::Regtest,
        _ => return Err(format!("Invalid network type: {}", network_type)),
    };

    if inputs.is_empty() {
        return Err("No inputs provided".to_string());
    }
    if outputs.is_empty() {
        return Err("No outputs provided".to_string());
    }

    let mut builder = PsbtBuilder::new(network);

    for input in inputs.clone() {
        let tx_id =
            Txid::from_str(&input.txid).map_err(|e| format!("Invalid input txid: {}", e))?;

        let input_utxo = InputUtxo {
            tx_id,
            vout: input.vout,
            value: Amount::from_sat(input.amount),
        };

        let pubkey_bytes = if let Some(pk) = input.public_key {
            Some(hex::decode(&pk).map_err(|e| format!("Invalid public key hex: {}", e))?)
        } else {
            None
        };

        builder.add_input(
            input_utxo,
            &input.address,
            pubkey_bytes.as_deref(),
            input.signature_type,
        )?;
    }

    for output in outputs {
        if output.op_return.is_some() {
            if output.amount != 0 {
                return Err("OP_RETURN output must have zero amount".to_string());
            }
        }

        builder.add_output(&output.address, output.amount, output.op_return)?;
    }

    let mut psbt = builder.build()?;

    let sighash = {
        let tx = psbt.unsigned_tx.clone();
        let mut sihash_cache = SighashCache::new(tx);

        let mut tx_out_vec = vec![];

        for input in &inputs {
            let address = Address::from_str(&input.address)
                .map_err(|e| format!("Invalid input address: {}", e))?
                .require_network(network)
                .map_err(|_| "Address network mismatch".to_string())?;
            tx_out_vec.push(TxOut {
                value: Amount::from_sat(input.amount),
                script_pubkey: address.script_pubkey(),
            });
        }
        sihash_cache
            .taproot_key_spend_signature_hash(
                0,
                &bitcoin::sighash::Prevouts::All(&tx_out_vec.as_slice()),
                bitcoin::TapSighashType::All,
            )
            .unwrap()
    };

    let signature_blob = ic::schnorr_api::sign_with_schnorr(
        "test_key_1".to_string(),
        crate::get_derivation_path(token_path), 
        Some(vec![]),
        <TapSighash as AsRef<[u8; 32]>>::as_ref(&sighash).to_vec(),
    )
    .await;
    let token_signature_blob = ic::schnorr_api::sign_with_schnorr(
        "test_key_1".to_string(),
        crate::get_derivation_path("fomowell-btc-address"),
        Some(vec![]),
        <TapSighash as AsRef<[u8; 32]>>::as_ref(&sighash).to_vec(),
    )
    .await;

    let signature = bitcoin::taproot::Signature {
        signature: bitcoin::secp256k1::schnorr::Signature::from_slice(&signature_blob).unwrap(),
        sighash_type: bitcoin::TapSighashType::All,
    };
    psbt.inputs[0].final_script_witness = Some(bitcoin::Witness::from_slice(&[signature.to_vec()]));

    let token_signature = bitcoin::taproot::Signature {
        signature: bitcoin::secp256k1::schnorr::Signature::from_slice(&token_signature_blob)
            .unwrap(),
        sighash_type: bitcoin::TapSighashType::All,
    };

    for i in 1..(inputs.len() - btc_inputs.len()) {
        if let Some(witness_data) = &inputs[i].witness {
            if let Some(input) = psbt.inputs.get_mut(i) {
                input.final_script_witness =
                    Some(bitcoin::Witness::from_slice(&[witness_data.clone()]));
            }
        }
    }

    let start_index = inputs.len() - btc_inputs.len();
    for i in 0..btc_inputs.len() {
        psbt.inputs[start_index + i].final_script_witness = Some(bitcoin::Witness::from_slice(&[
            bitcoin::taproot::Signature {
                signature: bitcoin::secp256k1::schnorr::Signature::from_slice(
                    &token_signature_blob,
                )
                .map_err(|e| format!("Failed to create schnorr signature: {}", e))?,
                sighash_type: bitcoin::TapSighashType::AllPlusAnyoneCanPay,
            }
            .to_vec(),
        ]));
    }

    let tx: bitcoin::Transaction = psbt.clone().extract_tx().map_err(|e| e.to_string())?;
    let txid = tx.txid().to_string();

    let vsize: u64 = builder.estimate_vbytes()?;

    let serialized = psbt.serialize();
    let psbt_base64 = base64::encode(&serialized);
    let psbt_hex = hex::encode(&serialized);

    Ok(TransactionResult {
        txid,
        psbt_base64,
        psbt_hex,
        vsize,
    })
}

pub fn combine_psbt(psbt1: &str, psbt2: &str) -> Result<String, String> {
    let psbt1_bytes = if psbt1.chars().all(|c| c.is_ascii_hexdigit()) {
        hex::decode(psbt1).map_err(|e| format!("Failed to decode hex PSBT1: {}", e))?
    } else {
        base64::decode(psbt1).map_err(|e| format!("Failed to decode base64 PSBT1: {}", e))?
    };

    let psbt2_bytes = if psbt2.chars().all(|c| c.is_ascii_hexdigit()) {
        hex::decode(psbt2).map_err(|e| format!("Failed to decode hex PSBT2: {}", e))?
    } else {
        base64::decode(psbt2).map_err(|e| format!("Failed to decode base64 PSBT2: {}", e))?
    };

    let mut psbt1 = Psbt::deserialize(&psbt1_bytes)
        .map_err(|e| format!("Failed to deserialize PSBT1: {}", e))?;
    let psbt2 = Psbt::deserialize(&psbt2_bytes)
        .map_err(|e| format!("Failed to deserialize PSBT2: {}", e))?;

    psbt1
        .combine(psbt2)
        .map_err(|e| format!("Failed to combine PSBTs: {}", e))?;

    let combined_bytes = psbt1.serialize();
    Ok(base64::encode(&combined_bytes))
}
