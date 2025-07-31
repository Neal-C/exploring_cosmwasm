# Exploring CosmWasm

## Resources

- Sylvia , high-level smart contracts framework https://cosmwasm.cosmos.network/sylvia https://docs.rs/sylvia/latest/sylvia/
- https://cosmwasm.com/build
- https://cosmwasm.cosmos.network/
- https://tendermint.com/core/
- https://docs.cometbft.com/main/spec/consensus/bft-time
- https://informal.systems/blog/introducing-proposer-based-timestamps-pbts-in-cometbft

## Notes

- Tries to support reproducible builds

- Supports migrating the storage data schema

- Contract is a WebAssembly code stored in the blockchain’s state

- Contracts have no state except that which is contained in the binary code (e.g. static constants)

- State is stored in the key-value store on the blockchain’s state

- A reference to the contract’s binary, combined with a reference to the prefixed data store, uniquely identifies the smart contract

- The Cosmos SDK is based upon the Tendermint BFT Consensus Engine

- Tendermint Core is a Byzantine Fault Tolerant (BFT) middleware that takes a state transition machine - written in any programming language - and securely replicates it on many machines.

- CosmWasm is built around the actor model

- CosmWasm effectively forces you (with compile time checks) to follow the CEI pattern (Checks, Effects, Interactions) while other similar systems only have this as a “best practice”.

- IBC is a standard that defines how blockchains can send and receive messages to each other. This allows for the creation of a network of blockchains that can interact with each other


## Bugs

- Some documentation example (last updated may 28 2025) is outdated and does not compile. Namely, some of the ibc2 examples

- Default template doxxes your email address

- Default template CI unexpectedly triggers on every push


## Instructions

### With Docker

```bash
git clone https://github.com/Neal-C/exploring_cosmwasm.git
```

```bash
cd exploring_cosmwasm
```

```bash
docker build -t neal-c-cosmwasm:latest .
```

```bash
docker run --name neal-c-cosmwasm neal-c-cosmwasm:latest
```

### With local installation

Requirements: Rust >= 1.88.0

```bash
rustup target add wasm32-unknown-unknown
```

```bash
git clone https://github.com/Neal-C/exploring_cosmwasm.git
```

```bash
cd exploring_cosmwasm
```

```bash
cargo wasm
```

```bash
cargo unit-test
```

```bash
cargo integration-test  
```
