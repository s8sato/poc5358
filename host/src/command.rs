use crate::bindings;
use crate::prelude as host;

use wasmtime_wasi::p2;
use wasmtime_wasi::p2::bindings::cli::exit::LinkOptions;

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
    store: wasmtime::Store<CommandState>,
}

pub struct CommandState {
    pub host: HostState,
    pub wasi: p2::WasiCtx,
}

pub struct HostState {
    args: String,
}

impl p2::IoView for CommandState {
    fn table(&mut self) -> &mut wasmtime_wasi::ResourceTable {
        unimplemented!()
    }
}
impl p2::WasiView for CommandState {
    fn ctx(&mut self) -> &mut p2::WasiCtx {
        &mut self.wasi
    }
}

impl bindings::poc::wit::general::Host for CommandState {}
impl bindings::poc::wit::read::Host for CommandState {}
impl bindings::poc::wit::view::Host for CommandState {}
impl bindings::poc::wit::write::Host for CommandState {}
impl p2::bindings::cli::environment::Host for CommandState {
    fn get_environment(
        &mut self,
    ) -> wasmtime::Result<
        wasmtime::component::__internal::Vec<(
            wasmtime::component::__internal::String,
            wasmtime::component::__internal::String,
        )>,
    > {
        unimplemented!()
    }
    fn get_arguments(
        &mut self,
    ) -> wasmtime::Result<
        wasmtime::component::__internal::Vec<wasmtime::component::__internal::String>,
    > {
        unimplemented!()
    }
    fn initial_cwd(&mut self) -> wasmtime::Result<Option<wasmtime::component::__internal::String>> {
        unimplemented!()
    }
}
impl p2::bindings::cli::exit::Host for CommandState {
    fn exit(&mut self, status: Result<(), ()>) -> wasmtime::Result<()> {
        unimplemented!()
    }
    fn exit_with_code(&mut self, status_code: u8) -> wasmtime::Result<()> {
        unimplemented!()
    }
}
impl p2::bindings::io::error::Host for CommandState {}
impl p2::bindings::io::error::HostError for CommandState {
    fn to_debug_string(
        &mut self,
        self_: wasmtime::component::Resource<p2::IoError>,
    ) -> wasmtime::Result<wasmtime::component::__internal::String> {
        unimplemented!()
    }
    fn drop(&mut self, rep: wasmtime::component::Resource<p2::IoError>) -> wasmtime::Result<()> {
        unimplemented!()
    }
}
impl p2::bindings::io::streams::Host for CommandState {
    fn convert_stream_error(
        &mut self,
        err: super::super::super::_TrappableError0,
    ) -> wasmtime::Result<p2::bindings::io::streams::StreamError> {
        unimplemented!()
    }
}
impl p2::bindings::io::streams::HostOutputStream for CommandState {
    fn check_write(
        &mut self,
        self_: wasmtime::component::Resource<p2::DynOutputStream>,
    ) -> Result<u64, super::super::super::_TrappableError0> {
        unimplemented!()
    }
    // heart break...
}

// --- State transition ---

pub fn initiate(command: WasmCommand, engine: &wasmtime::Engine) -> Init {
    let host = HostState { args: command.args };
    let wasi = p2::WasiCtxBuilder::new().build();

    let mut linker = wasmtime::component::Linker::new(engine);
    p2::bindings::cli::environment::add_to_linker(&mut linker, |state: &mut CommandState| state)
        .expect("failed to add WASI environment to linker");
    p2::bindings::cli::exit::add_to_linker(
        &mut linker,
        &LinkOptions::default(),
        |state: &mut CommandState| state,
    )
    .expect("failed to add WASI exit to linker");
    p2::bindings::io::error::add_to_linker(&mut linker, |state: &mut CommandState| state)
        .expect("failed to add WASI error to linker");
    bindings::Universe::add_to_linker(&mut linker, |state: &mut CommandState| state)
        .expect("failed to add bindings to linker");

    let mut store = wasmtime::Store::new(engine, CommandState { host, wasi });
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
        let args = self.wasmtime.store.data().host.args.clone();
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
        let args = self.wasmtime.store.data().host.args.clone();
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
