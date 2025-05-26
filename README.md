# Executor Decomposition & WasmInstruction I/O PoC

This repository is a proof-of-concept (PoC) implementation exploring the feasibility of decomposing the Executor component in Hyperledger Iroha, as proposed in the following issues:

* https://github.com/hyperledger-iroha/iroha/issues/5357
* https://github.com/hyperledger-iroha/iroha/issues/5358

## Purpose

The goal of this PoC is to validate whether the core transaction execution responsibilities can be cleanly separated into:

1. A WasmInstruction module that submits read/write requests without directly mutating host state
2. An Authorizer module that approves or rejects each request based on permission rules
3. A Host runtime that batches, authorizes, and applies state changes in its own memory/storage

By isolating these concerns, we aim to improve maintainability, extensibility, and testability of the execution pipeline.

## Project Structure

```text
my_poc_project/                   ← Workspace root
├── common/                       ← Shared ABI definitions and types
│   └── src/lib.rs                ← `RequestEntry`, `WriteEntry`, constants
├── host/                         ← Host runtime (Rust native)
│   ├── src/
│   │   ├── main.rs               ← Wasmtime setup & `_start` invocation
│   │   └── lib.rs                ← Host state (storage, memory buffer)
│   └── tests/
│       └── integration.rs        ← End-to-end scenarios
├── wasm_instruction/             ← WasmInstruction module (wasm32-unknown-unknown)
│   └── src/lib.rs                ← `generate_read_request`, `generate_write_request`, `finish()`
└── authorizer/                   ← Authorizer module (wasm32-unknown-unknown)
    └── src/lib.rs                ← `seek_read_approval`, `seek_write_approval`
```

## Building & Testing

Ensure you have Rust and wasm32 target installed:

```bash
rustup target add wasm32-unknown-unknown
```

Build all components:

```bash
cargo build --workspace --release
```

Run integration tests:

```bash
cargo test --package host
```

---

*This PoC is experimental and intended solely for feasibility exploration of the referenced Iroha issues.*
