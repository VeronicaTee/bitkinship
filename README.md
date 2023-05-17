# bitkinship

This project provides functionality for calculating the relatedness between Bitcoin Unspent Transaction Outputs (UTXOs). It includes methods to fetch UTXOs from addresses and calculate their relatedness value.

## Features

- Fetch UTXOs from addresses using the Blockstream API.
- Calculate the relatedness value between pairs of UTXOs.
- Support for BIP32 extended public keys (xpub) to derive addresses.

## Requirements

- Rust programming language (version 1.55 or higher)
- `bitcoin` crate (version 0.26 or higher)
- `reqwest` crate (version 0.11 or higher)

## Installation

1. Install Rust by following the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

2. Clone the repository:

```bash
git clone https://github.com/VeronicaTee/bitkinship.git
cd bitkinship

3. Run cargo build

