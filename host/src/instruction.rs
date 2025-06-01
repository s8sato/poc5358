#![expect(dead_code)]

use crate::bindings;
use crate::prelude as host;

use wasmtime_wasi::p2;

pub enum InstructionEnum {
    Builtin(BuiltinInstruction),
    Wasm(WasmInstruction),
}

pub enum BuiltinInstruction {}

pub struct WasmInstruction {
    // TODO #5147: Reference the component compiled and registered in advance.
    // component: WasmComponentId,
    pub component: WasmComponent,
    pub args: String,
}

pub type WasmComponent = wasmtime::component::Component;

pub struct Wasmtime {
    universe: bindings::Universe,
    store: wasmtime::Store<InstructionState>,
}

/// Data relevant only during Wasm execution.
pub struct InstructionState {
    pub host: HostState,
    pub wasi: p2::WasiCtx,
    pub resource_table: wasmtime_wasi::ResourceTable,
}

pub struct HostState {
    args: String,
}

impl p2::IoView for InstructionState {
    fn table(&mut self) -> &mut wasmtime_wasi::ResourceTable {
        &mut self.resource_table
    }
}
impl p2::WasiView for InstructionState {
    fn ctx(&mut self) -> &mut p2::WasiCtx {
        &mut self.wasi
    }
}

impl bindings::poc::wit::types::Host for InstructionState {}

// --- State transition ---

pub fn initiate(instruction: WasmInstruction, authority: host::AccountK) -> Init {
    let host = HostState {
        args: instruction.args,
    };
    let engine = instruction.component.engine();
    let mut store = wasmtime::Store::new(
        engine,
        InstructionState {
            host,
            wasi: p2::WasiCtxBuilder::new().build(),
            resource_table: wasmtime_wasi::ResourceTable::new(),
        },
    );

    let mut linker = wasmtime::component::Linker::new(engine);
    p2::add_to_linker_sync(&mut linker).expect("failed to add WASI bindings to linker");
    bindings::Universe::add_to_linker(&mut linker, |state: &mut InstructionState| state)
        .expect("failed to add bindings to linker");

    let universe = bindings::Universe::instantiate(&mut store, &instruction.component, &linker)
        .expect("failed to instantiate component");
    let wasmtime = Wasmtime { universe, store };

    Init {
        authority,
        wasmtime,
    }
}

pub struct Init {
    authority: host::AccountK,
    wasmtime: Wasmtime,
}

impl Init {
    pub fn read_request(self) -> ToRead {
        let args = self.wasmtime.store.data().host.args.clone();
        let Init {
            authority,
            mut wasmtime,
        } = self;
        let request = wasmtime
            .universe
            .call_read_request(&mut wasmtime.store, &args)
            .expect("failed to call read_request function");

        ToRead {
            authority,
            wasmtime,
            request,
        }
    }
}

pub struct ToRead {
    authority: host::AccountK,
    wasmtime: Wasmtime,
    request: bindings::ReadSet,
}

impl ToRead {
    pub fn read_approval(self) -> Result<Reading, ()> {
        let ToRead {
            authority,
            wasmtime,
            request,
        } = self;
        // TODO: seek approval from the authorizer
        Ok(Reading {
            authority,
            wasmtime,
            request,
        })
    }
}

pub struct Reading {
    authority: host::AccountK,
    wasmtime: Wasmtime,
    request: bindings::ReadSet,
}

impl Reading {
    pub fn read(self, state: &impl crate::state::WorldState) -> Result<HasRead, ()> {
        let Reading {
            authority,
            wasmtime,
            request,
        } = self;
        let request = host::ReadSet::from(request);
        println!("Reading request: {:#?}", &request);
        let result = state.read(&request).into();

        Ok(HasRead {
            authority,
            wasmtime,
            result,
        })
    }
}

pub struct HasRead {
    authority: host::AccountK,
    wasmtime: Wasmtime,
    result: bindings::ViewSet,
}

impl HasRead {
    pub fn write_request(self) -> ToWrite {
        let args = self.wasmtime.store.data().host.args.clone();
        let HasRead {
            authority,
            mut wasmtime,
            result,
        } = self;
        let request = wasmtime
            .universe
            .call_write_request(&mut wasmtime.store, &result, &args)
            .expect("failed to call write_request function");

        ToWrite { authority, request }
    }
}

pub struct ToWrite {
    authority: host::AccountK,
    request: bindings::WriteSet,
}

impl ToWrite {
    pub fn write_approval(self) -> Result<Writing, ()> {
        let ToWrite { authority, request } = self;
        // TODO: seek approval from the authorizer
        Ok(Writing { authority, request })
    }
}

pub struct Writing {
    authority: host::AccountK,
    request: bindings::WriteSet,
}

impl Writing {
    pub fn write(self, state: &mut impl crate::state::WorldState) -> Result<HasWritten, ()> {
        let Writing { authority, request } = self;
        let request = host::WriteSet::from(request);
        println!("Writing request: {:#?}", &request);
        state.write(&request, authority.clone());
        let result = (request, authority).into();

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
