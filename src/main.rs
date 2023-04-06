mod utxos;
// mod xpub;

fn main() {
    use utxos::*;
    // use xpub::relatedness_for_xpub;

    let utxo1 = Utxo {
        txid: "d37fc44f31fc16a8b2e14b1a8531bb651f666312033db135076c22a9ba926dc9".to_string(),
        vout: 1,
        address: "bc1q0rr0p8cgrvte9d3q3lt83qgr3y65xkvh0s4h0z".to_string(),
        amount: 100_000_000,
        block_height: Some(710_000),
        inputs: vec![("3d3a8107fb8e6e46f9f1d6d371b7dc8c729c03329d746f5aa5d5b5e5c5f93ec3".to_string(), 0)],
    };

    let utxo2 = Utxo {
    txid: "e6a790b6f1c6b8ed450d8b2bbf9a5a5c5d5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a".to_string(),
    vout: 0,
    address: "bc1q4rr0p8cgrvte9d3q3lt83qgr3y65xkvh0s4h0z".to_string(),
    amount: 50_000_000,
    block_height: Some(720_000),
    inputs: vec![("3d3a8107fb8e6e46f9f1d6d371b7dc8c729c03329d746f5aa5d5b5e5c5f93ec3".to_string(), 1)],
    };


    println!("{}", relatedness(utxo1, utxo2));

    // let xpub = "xpub6C7hLiJp61qV7zyuFeJe1zV7JukPAgTQtrF9Y9XKDD7GxajWJq3zV57sBYsRSw7Vd8W4vegJvmsbYjJUb7HUhSpspjdwZgVjK77aA2FETMj".to_string();
    // let related_pairs = relatedness_for_xpub(xpub);
    // println!("{:?}", related_pairs);

}
