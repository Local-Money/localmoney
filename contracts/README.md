# Local Money Smart Contracts
All the contracts from the LocalMoney protocol.

### Commands

To build the protocol, you will need to use a specific version of cargo: `cargo 1.67.0-nightly (16b097879 2022-11-14)`. This version of cargo is necessary to ensure that the protocol is built correctly.
```bash
cargo +nightly build
```

To compile the protocol, you will need to have Docker installed on your computer. Once Docker is installed, you can use the script in the `/contracts` directory to generate `.wasm` files in the `/contracts/artifacts` path.
```bash
sh optimize.sh
```

