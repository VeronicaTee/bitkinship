use bitcoin::{Address, Amount, OutPoint, TxIn, Txid};
use bitcoin::hashes::hex::FromHex;
use std::str::FromStr;
mod utxos;

fn main() {
    use utxos::*;

    let utxo1 = Utxo {
        outpoint: OutPoint {
            txid: Txid::from_hex("d37fc44f31fc16a8b2e14b1a8531bb651f666312033db135076c22a9ba926dc9").unwrap(),
            vout: 1,
        },
        address: Address::from_str("2MxE7ujjouqUrTiUZReYJ98avwwbxyEp48z").unwrap(),
        amount: Amount::from_sat(100_000_000),
        block_height: Some(710_000),
        inputs: vec![TxIn {
            previous_output: OutPoint {
                txid: Txid::from_hex("3d3a8107fb8e6e46f9f1d6d371b7dc8c729c03329d746f5aa5d5b5e5c5f93ec3").unwrap(),
                vout: 0,
            },
            sequence: 0,
            witness: vec![],
            script_sig: bitcoin::Script::new(),
        }],
    };

    let utxo2 = Utxo {
        outpoint: OutPoint {
            txid: Txid::from_hex("e6a790b6f1c6b8ed450d8b2bbf9a5a5c5d5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a").unwrap(),
            vout: 0,
        },
        address: Address::from_str("2MxE7ujjouqUrTiUZReYJ98avwwbxyEp48z").unwrap(),
        amount: Amount::from_sat(50_000_000),
        block_height: Some(720_000),
        inputs: vec![TxIn {
            previous_output: OutPoint {
                txid: Txid::from_hex("3d3a8107fb8e6e46f9f1d6d371b7dc8c729c03329d746f5aa5d5b5e5c5f93ec3").unwrap(),
                vout: 1,
            },
            sequence: 0,
            witness: vec![],
            script_sig: bitcoin::Script::new(),
        }],
    };

    println!("{}", relatedness(utxo1, utxo2));
}
