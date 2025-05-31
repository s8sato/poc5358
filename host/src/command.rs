use crate::bindings;
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
    universe: bindings::Universe,
    store: wasmtime::Store<HostState>,
}

pub struct HostState {
    args: String,
}

impl bindings::poc::wit::general::Host for HostState {}
impl bindings::poc::wit::read::Host for HostState {}
impl bindings::poc::wit::view::Host for HostState {}
impl bindings::poc::wit::write::Host for HostState {}

// --- State transition ---

pub fn initiate(command: WasmCommand, engine: &wasmtime::Engine) -> Init {
    let host_state = HostState { args: command.args };
    let mut linker = wasmtime::component::Linker::new(engine);

    bindings::Universe::add_to_linker(&mut linker, |state: &mut HostState| state)
        .expect("failed to add bindings to linker");

    let mut store = wasmtime::Store::new(engine, host_state);
    let universe = bindings::Universe::instantiate(&mut store, &command.component, &linker)
        .expect("failed to instantiate component");
    let wasmtime = Wasmtime { universe, store };

    Init { wasmtime }
}

pub struct Init {
    wasmtime: Wasmtime,
}

impl Init {
    pub fn read_request(self) -> ToRead {
        let args = self.wasmtime.store.data().args.clone();
        let Init { mut wasmtime } = self;
        let request = wasmtime
            .universe
            .call_read_request(&mut wasmtime.store, &args)
            .expect("failed to call read_request function");

        ToRead { wasmtime, request }
    }
}

pub struct ToRead {
    wasmtime: Wasmtime,
    request: bindings::ReadSet,
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
    request: bindings::ReadSet,
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
    result: bindings::ViewSet,
}

impl HasRead {
    pub fn write_request(self) -> ToWrite {
        let args = self.wasmtime.store.data().args.clone();
        let HasRead {
            mut wasmtime,
            result,
        } = self;
        let request = wasmtime
            .universe
            .call_write_request(&mut wasmtime.store, &result, &args)
            .expect("failed to call write_request function");

        ToWrite { request }
    }
}

pub struct ToWrite {
    request: bindings::WriteSet,
}

impl ToWrite {
    pub fn write_approval(self) -> Result<Writing, ()> {
        let ToWrite { request } = self;
        // TODO: seek approval from the authorizer
        Ok(Writing { request })
    }
}

pub struct Writing {
    request: bindings::WriteSet,
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
    result: bindings::WriteSet,
}

// pub struct ToPay;

// pub struct Paying;

// pub struct HasPaid;

pub struct Record;
