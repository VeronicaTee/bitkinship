use bitcoin::{Address, Amount, OutPoint, TxIn};

pub struct Utxo {
    pub outpoint: OutPoint,
    pub address: Address,
    pub amount: Amount,
    pub block_height: Option<u32>,
    pub inputs: Vec<TxIn>,
}


pub fn relatedness(utxo1: Utxo, utxo2: Utxo) -> f64 {
    let w_address = 0.5;
    let w_amount = 0.3;
    let w_block_height = 0.1;
    let w_input = 0.1;

    let address_similarity = if utxo1.address == utxo2.address { 1.0 } else { 0.0 };
    // let amount_similarity = 1.0 - ((utxo1.amount - utxo2.amount).abs() / (utxo1.amount + utxo2.amount));
    let amount_similarity = 1.0 - ((utxo1.amount.as_btc() - utxo2.amount.as_btc()).abs() / (utxo1.amount.as_btc() + utxo2.amount.as_btc()));
    let block_height_similarity = match (utxo1.block_height, utxo2.block_height) {
        (Some(h1), Some(h2)) => 1.0 - ((h1 as f32 - h2 as f32).abs() / (h1 as f32 + h2 as f32)),
        _ => 0.0,
    };
    // let input_similarity = if utxo1.inputs.iter().any(|input1| utxo2.inputs.iter().any(|input2| input1 == input2)) { 1.0 } else { 0.0 };
    let input_similarity = if utxo1.inputs.iter().any(|input1| utxo2.inputs.iter().any(|input2| input1.previous_output == input2.previous_output)) { 1.0 } else { 0.0 };
    let numerator = w_address * address_similarity + w_amount * amount_similarity + w_block_height * (block_height_similarity as f64) + w_input * input_similarity;
    let denominator = w_address + w_amount + w_block_height + w_input;

    numerator / denominator
}

