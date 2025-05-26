# Executor Modularization & WasmInstruction I/O Simplification PoC

This proof-of-concept explores splitting Hyperledger Iroha’s _Executor_ into modular pieces and slimming down _WasmInstruction_ I/O. The inspiration comes from:

- <https://github.com/hyperledger-iroha/iroha/issues/5357>
- <https://github.com/hyperledger-iroha/iroha/issues/5358> (see state-transition diagram)

## Objective

Can we cleanly separate instruction execution into three roles?

1. _WasmInstruction_: collects read/write intents without directly mutating state
2. _Authorizer_: evaluates intents against permission rules (approve or reject)
3. _Host runtime_: initiates the instruction flow, batches intents for the authorizer, and applies them to state

Success means smaller, testable components, fewer FFI round-trips, and clearer extension points.

## Repository Structure

```text
.
├── guest/
│   ├── authorizer/          — Wasm component that enforces permissions
│   └── instruction/         — Wasm component that submits read/write intents
├── host/                    — Rust runtime and tests
├── wit/                     — Shared WIT interfaces
└── README.md
```

## Building & Testing

### Prerequisites

```bash
rustup target add wasm32-wasip2
```

```bash
cargo add wit-bindgen
```

### Guest components

```bash
cargo build --target wasm32-wasip2 --manifest-path guest/instruction/Cargo.toml
```

```bash
cargo build --target wasm32-wasip2 --manifest-path guest/authorizer/Cargo.toml
```

### Host tests

```bash
cargo test --package host --lib -- tests::instruction_flow --exact --show-output 
```

Compare the test steps to the #5358 state-transition diagram for clarity.

## Developer Notes

### Host vs. guest, imports vs. exports

- `wasmtime::component::bindgen!` is used on the _host_ side to implement _import_ functions.
- `wit_bindgen::generate!` is used on the _guest_ side to implement _export_ functions.

### Component model trade-offs

- Removes all `unsafe` blocks around FFI calls, making maintenance easier.
- Wasm _components_ typically produce larger binaries than classic _modules_—keep that in mind.

### Future developer experience

- Consider `guest/instruction/src/lib.rs` as a reference implementation of smart contracts and trigger executables. It’s intentionally verbose now; later we can introduce syntax sugars.

---

_This PoC is experimental and exists solely to test the feasibility of the referenced Iroha issues._
