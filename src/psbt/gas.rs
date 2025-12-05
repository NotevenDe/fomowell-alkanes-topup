use std::collections::HashMap;

pub const DUST_THRESHOLD: u64 = 330;

#[derive(Debug)]
pub struct Utxo {
    pub amount: u64,
    pub address: Option<String>,
}

#[derive(Debug)]
pub struct Output {
    pub amount: u64,
    pub script: Option<Script>,
    pub address: Option<String>,
}

#[derive(Debug)]
pub struct Script {
    pub hex: String,
}

#[derive(Debug)]
pub struct FeeCalculationResult {
    pub fee: u64,
    pub change_amount: u64,
    pub estimated_size: usize,
    pub total_input: u64,
    pub total_output: u64,
    pub include_change: bool,
}

#[derive(Debug, PartialEq)]
enum InputType {
    P2PKH,
    P2WPKH,
    P2TR,
}

#[derive(Debug, PartialEq)]
enum OutputType {
    P2PKH,
    P2WPKH,
    P2SH,
    P2TR,
}

fn get_input_type(address: &Option<String>) -> InputType {
    match address {
        Some(addr) if addr.starts_with("bc1q") || addr.starts_with("tb1q") => InputType::P2WPKH,
        Some(addr) if addr.starts_with("bc1p") || addr.starts_with("tb1p") => InputType::P2TR,
        Some(addr) if addr.starts_with('1') || addr.starts_with('m') || addr.starts_with('n') => {
            InputType::P2PKH
        }
        _ => InputType::P2TR, 
    }
}

fn get_output_type(address: &Option<String>) -> OutputType {
    match address {
        Some(addr) if addr.starts_with("bc1q") || addr.starts_with("tb1q") => OutputType::P2WPKH,
        Some(addr) if addr.starts_with("bc1p") || addr.starts_with("tb1p") => OutputType::P2TR,
        Some(addr) if addr.starts_with('3') || addr.starts_with('2') => OutputType::P2SH,
        Some(addr) if addr.starts_with('1') || addr.starts_with('m') || addr.starts_with('n') => {
            OutputType::P2PKH
        }
        _ => OutputType::P2TR, 
    }
}

pub fn calculate_fee_and_change(
    utxos: Vec<Utxo>,
    base_outputs: Vec<Output>,
    fee_rate: f64,
) -> FeeCalculationResult {
    let normal_outputs: Vec<&Output> = base_outputs
        .iter()
        .filter(|output| output.script.is_none())
        .collect();
    let script_outputs: Vec<&Output> = base_outputs
        .iter()
        .filter(|output| output.script.is_some() && !output.script.as_ref().unwrap().hex.is_empty())
        .collect();

    let mut overhead_size = 4; // nVersion

    let input_count = utxos.len();
    overhead_size += match input_count {
        0..=252 => 1,
        253..=65535 => 3,
        _ => 5,
    };

    let output_count = normal_outputs.len() + script_outputs.len();
    overhead_size += match output_count {
        0..=252 => 1,
        253..=65535 => 3,
        _ => 5,
    };

    overhead_size += 4;

    let has_segwit_input = true;
    if has_segwit_input {
        overhead_size += 0.5 as usize;
    }

    let mut input_size = 0.0;
    for utxo in &utxos {
        let input_type = get_input_type(&utxo.address);
        input_size += match input_type {
            InputType::P2PKH => 148.0,
            InputType::P2WPKH => 67.5,
            InputType::P2TR => 57.25,
        };
    }

    let mut normal_output_size = 0;
    for output in &normal_outputs {
        let output_type = get_output_type(&output.address);
        normal_output_size += match output_type {
            OutputType::P2PKH => 34,
            OutputType::P2WPKH => 31,
            OutputType::P2SH => 32,
            OutputType::P2TR => 43,
        };
    }

    let script_output_size = script_outputs.iter().fold(0, |sum, output| {
        let data_size = (output.script.as_ref().unwrap().hex.len() as f64 / 2.0).ceil() as usize;
        let script_pub_key_length_size = if data_size >= 253 && data_size < 65536 {
            3
        } else if data_size >= 65536 {
            5
        } else {
            1
        };
        sum + 8 + script_pub_key_length_size + data_size
    });

    let estimated_size =
        overhead_size + input_size as usize + normal_output_size + script_output_size;

    let fee = (estimated_size as f64 * fee_rate).ceil() as u64;

    let total_input = utxos.iter().fold(0, |sum, utxo| sum + utxo.amount);
    let total_output = base_outputs
        .iter()
        .fold(0, |sum, output| sum + output.amount);

    let change_amount = total_input - total_output - fee;

    let include_change = change_amount >= DUST_THRESHOLD;

    FeeCalculationResult {
        fee,
        change_amount: if include_change { change_amount } else { 0 },
        estimated_size,
        total_input,
        total_output,
        include_change,
    }
}

pub fn calculate_fee_simple(
    input_count: usize,
    output_count: usize,
    fee_rate: f64, 
) -> u64 {
    let mut overhead_size = 4; 

    overhead_size += match input_count {
        0..=252 => 1,
        253..=65535 => 3,
        _ => 5,
    };
    overhead_size += match output_count {
        0..=252 => 1,
        253..=65535 => 3,
        _ => 5,
    };
    overhead_size += 4; 
    overhead_size += 0.5 as usize;
    let input_size = input_count as f64 * 57.25;

    let output_size = output_count * 43;

    let estimated_size = overhead_size + input_size as usize + output_size;

    (estimated_size as f64 * fee_rate).ceil() as u64
}

pub fn calculate_fee_with_opreturn(
    input_count: usize,
    output_count: usize,
    runestone_bytes: usize,
    fee_rate: f64, 
) -> u64 {
    let mut overhead_size = 4; // nVersion

    overhead_size += match input_count {
        0..=252 => 1,
        253..=65535 => 3,
        _ => 5,
    };

    let total_output_count = output_count + 1; 

    overhead_size += match total_output_count {
        0..=252 => 1,
        253..=65535 => 3,
        _ => 5,
    };

    overhead_size += 4;

    let has_segwit_input = true;
    if has_segwit_input {
        overhead_size += 0.5 as usize; 
    }

    let input_size = input_count as f64 * 57.25;

    let regular_output_size = output_count * 43;


    let data_size = runestone_bytes;

    let script_pub_key_length_size = if data_size >= 253 && data_size < 65536 {
        3
    } else if data_size >= 65536 {
        5
    } else {
        1
    };


    let opreturn_output_size = 8 + script_pub_key_length_size + 2 + data_size;

    let estimated_size =
        overhead_size + input_size as usize + regular_output_size + opreturn_output_size;

    (estimated_size as f64 * fee_rate).ceil() as u64
}

pub fn calculate_fee_p2tr_with_opreturn(
    input_count: usize,
    output_count: usize,
    runestone_bytes: usize,
    fee_rate: f64, // 单位：聪/字节
) -> u64 {
    let mut overhead_size = 4; // nVersion

    overhead_size += match input_count {
        0..=252 => 1,
        253..=65535 => 3,
        _ => 5,
    };

    let total_output_count = output_count + 1; // +1 是为了OP_RETURN输出

    overhead_size += match total_output_count {
        0..=252 => 1,
        253..=65535 => 3,
        _ => 5,
    };

    overhead_size += 4; // nLockTime

    overhead_size += 0.5 as usize; 

    let input_size = input_count as f64 * 57.25;

    let output_size = output_count * 43;

    let data_size = runestone_bytes;
    let script_pub_key_length_size = if data_size >= 253 && data_size < 65536 {
        3
    } else if data_size >= 65536 {
        5
    } else {
        1
    };

    let opreturn_output_size = 8 + script_pub_key_length_size + 2 + data_size;

    let estimated_size = overhead_size + input_size as usize + output_size + opreturn_output_size;

    (estimated_size as f64 * fee_rate).ceil() as u64
}