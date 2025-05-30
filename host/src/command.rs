use crate::conversion::*;
use crate::prelude as host;

pub enum CommandEnum {
    Builtin(BuiltinCommand),
    Wasm(WasmCommand),
}

pub enum BuiltinCommand {}

pub struct WasmCommand {
    // TODO #5147: Reference the component compiled and registered in advance.
    // component: WasmComponentId,
    pub component: WasmComponent,
    pub args: String,
}

pub type WasmComponent = wasmtime::component::Component;

pub struct Wasmtime {
    instance: wasmtime::component::Instance,
    store: wasmtime::Store<HostState>,
}

pub struct HostState {
    args: String,
}

// --- State transition ---

pub fn initiate(command: WasmCommand, engine: &wasmtime::Engine) -> Init {
    let host_state = HostState { args: command.args };
    let mut store = wasmtime::Store::new(engine, host_state);
    let linker = wasmtime::component::Linker::new(engine);
    let instance = linker
        .instantiate(&mut store, &command.component)
        .expect("failed to instantiate component");
    let wasmtime = Wasmtime { instance, store };

    Init { wasmtime }
}

pub struct Init {
    wasmtime: Wasmtime,
}

impl Init {
    pub fn read_request(self) -> ToRead {
        let args = self.wasmtime.store.data().args.clone();
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

pub struct ToRead {
    wasmtime: Wasmtime,
    request: ReadSet,
}

impl ToRead {
    pub fn read_approval(self) -> Result<Reading, ()> {
        let ToRead { wasmtime, request } = self;
        // TODO: seek approval from the authorizer
        Ok(Reading { wasmtime, request })
    }
}

pub struct Reading {
    wasmtime: Wasmtime,
    request: ReadSet,
}

impl Reading {
    pub fn read(self, state: &impl crate::state::WorldState) -> Result<HasRead, ()> {
        let Reading { wasmtime, request } = self;
        let request = host::ReadSet::from(request);
        let result = state.read(&request).into();

        Ok(HasRead { wasmtime, result })
    }
}

pub struct HasRead {
    wasmtime: Wasmtime,
    result: ViewSet,
}

impl HasRead {
    pub fn write_request(self) -> ToWrite {
        let args = self.wasmtime.store.data().args.clone();
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

pub struct ToWrite {
    request: WriteSet,
}

impl ToWrite {
    pub fn write_approval(self) -> Result<Writing, ()> {
        let ToWrite { request } = self;
        // TODO: seek approval from the authorizer
        Ok(Writing { request })
    }
}

pub struct Writing {
    request: WriteSet,
}

impl Writing {
    pub fn write(self, state: &mut impl crate::state::WorldState) -> Result<HasWritten, ()> {
        let Writing { request } = self;
        let request = host::WriteSet::from(request);
        state.write(&request);
        let result = request.into();

        Ok(HasWritten { result })
    }
}

pub struct HasWritten {
    result: WriteSet,
}

// pub struct ToPay;

// pub struct Paying;

// pub struct HasPaid;

pub struct Record;
