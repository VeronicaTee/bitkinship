use bitcoin::util::bip32::{DerivationPath, ExtendedPubKey};
use bitcoin::{Address, Network, TxOut};
use reqwest::blocking::Client;
use crate::utxos::Utxo;
use crate::utxos::relatedness;

fn relatedness_for_xpub(xpub: String) -> Vec<(Utxo, Utxo, f32)> {
    // Parse the xpub and derive all the addresses
    let xpub_key = ExtendedPubKey::from_str(&xpub).unwrap();
    let mut addresses = Vec::new();
    for i in 0..1000 {
        let address = Address::from_pubkey(&xpub_key.derive_pub(&[i as u32]).public_key, Network::Bitcoin);
        addresses.push(address.to_string());
    }

    // Fetch all the UTXOs for the derived addresses
    let client = Client::new();
    let mut utxos = Vec::new();
    for address in addresses {
        let url = format!("https://blockstream.info/api/address/{}/utxo", address);
        let response = client.get(&url).send().unwrap();
        let txouts: Vec<TxOut> = response.json().unwrap();
        for txout in txouts {
            let utxo = Utxo {
                txid: txout.txid().to_string(),
                vout: txout.vout,
                address: address.clone(),
                amount: txout.value,
                block_height: Some(txout.height.unwrap_or(0)),
                inputs: vec![],
            };
            utxos.push(utxo);
        }
    }

    // Create all possible pairs of UTXOs and calculate their relatedness value
    let mut related_pairs = Vec::new();
    for i in 0..utxos.len() {
        for j in i+1..utxos.len() {
            let relatedness_value = relatedness(utxos[i].clone(), utxos[j].clone());
            related_pairs.push((utxos[i].clone(), utxos[j].clone(), relatedness_value));
        }
    }

    related_pairs
}
