use super::*;

enum Command {
    Builtin(BuiltinCommand),
    Wasm(WasmCommand),
}

enum BuiltinCommand {}

struct WasmCommand {
    executable: ExecutableId,
    args: Json,
}

struct ExecutableId;

struct Json;

struct Wasmtime;

// --- State transition ---

struct Init {
    wasmtime: Wasmtime,
}

struct ToRead {
    wasmtime: Wasmtime,
    request: ReadSet,
}

struct Reading {
    wasmtime: Wasmtime,
    request: ReadSet,
}

struct HasRead {
    wasmtime: Wasmtime,
    result: StateView,
}

struct ToWrite {
    request: WriteSet,
}

struct Writing {
    request: WriteSet,
}

struct HasWritten;

// struct ToPay;

// struct Paying;

// struct HasPaid;

struct Record;
