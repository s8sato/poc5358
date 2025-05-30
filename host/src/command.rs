use common::conversion::*;
use common::host;

enum CommandEnum {
    Builtin(BuiltinCommand),
    Wasm(WasmCommand),
}

enum BuiltinCommand {}

struct WasmCommand {
    // TODO #5147: Reference the component compiled and registered in advance.
    // component: WasmComponentId,
    component: WasmComponent,
    args: String,
}

type WasmComponent = wasmtime::component::Component;

struct Wasmtime {
    instance: wasmtime::component::Instance,
    store: wasmtime::Store<HostState>,
}

struct HostState {
    args: String,
}

// --- State transition ---

struct Init {
    wasmtime: Wasmtime,
}

impl Init {
    fn new(command: WasmCommand, engine: &wasmtime::Engine) -> Self {
        let host_state = HostState { args: command.args };
        let mut store = wasmtime::Store::new(engine, host_state);
        let linker = wasmtime::component::Linker::new(engine);
        let instance = linker
            .instantiate(&mut store, &command.component)
            .expect("failed to instantiate component");
        let wasmtime = Wasmtime { instance, store };

        Self { wasmtime }
    }

    fn read_request(self, args: String) -> ToRead {
        let Init { mut wasmtime } = self;
        let (request,) = wasmtime
            .instance
            .get_typed_func::<(String,), (ReadSet,)>(&mut wasmtime.store, "read_request")
            .expect("failed to get read_request function")
            .call(&mut wasmtime.store, (args,))
            .expect("failed to call read_request function");

        ToRead { wasmtime, request }
    }
}

struct ToRead {
    wasmtime: Wasmtime,
    request: ReadSet,
}

impl ToRead {
    fn read_approval(self) -> Result<Reading, ()> {
        let ToRead { wasmtime, request } = self;
        // TODO: seek approval from the authorizer
        Ok(Reading { wasmtime, request })
    }
}

struct Reading {
    wasmtime: Wasmtime,
    request: ReadSet,
}

impl Reading {
    fn read(self, state: &impl crate::state::WorldState) -> Result<HasRead, ()> {
        let Reading { wasmtime, request } = self;
        let request = host::ReadSet::from(request);
        let result = state.read(&request).into();

        Ok(HasRead { wasmtime, result })
    }
}

struct HasRead {
    wasmtime: Wasmtime,
    result: ViewSet,
}

impl HasRead {
    fn write_request(self, args: String) -> ToWrite {
        let HasRead {
            mut wasmtime,
            result,
        } = self;
        let (request,) = wasmtime
            .instance
            .get_typed_func::<(String, ViewSet), (WriteSet,)>(&mut wasmtime.store, "write_request")
            .expect("failed to get write_request function")
            .call(&mut wasmtime.store, (args, result))
            .expect("failed to call write_request function");

        ToWrite { request }
    }
}

struct ToWrite {
    request: WriteSet,
}

impl ToWrite {
    fn write_approval(self) -> Result<Writing, ()> {
        let ToWrite { request } = self;
        // TODO: seek approval from the authorizer
        Ok(Writing { request })
    }
}

struct Writing {
    request: WriteSet,
}

impl Writing {
    fn write(self, state: &mut impl crate::state::WorldState) -> Result<HasWritten, ()> {
        let Writing { request } = self;
        let request = host::WriteSet::from(request);
        state.write(&request);
        let result = request.into();

        Ok(HasWritten { result })
    }
}

struct HasWritten {
    result: WriteSet,
}

// struct ToPay;

// struct Paying;

// struct HasPaid;

struct Record;
