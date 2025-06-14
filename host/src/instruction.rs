#![expect(dead_code)]

use crate::bindings;
use crate::prelude as host;
use crate::prelude::ExecutableK;

use wasmtime_wasi::p2;

pub enum InstructionEnum {
    Builtin(BuiltinInstruction),
    Wasm(WasmInstruction),
}

pub enum BuiltinInstruction {}

pub struct WasmInstruction {
    pub executable: ExecutableK,
    pub args: String,
}

pub type WasmComponent = wasmtime::component::Component;

pub struct Wasmtime {
    instruction: bindings::Universe,
    authorizer: bindings::Universe,
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

impl WasmInstruction {
    pub fn initiate(
        self,
        authority: host::AccountK,
        world: &impl crate::state::WorldState,
    ) -> Init {
        let host = HostState { args: self.args };
        let executable = world
            .executable(&self.executable)
            .expect("executable not found in the world state");
        let engine = executable.component.engine();
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

        let instruction =
            bindings::Universe::instantiate(&mut store, &executable.component, &linker)
                .expect("failed to instantiate instruction component");
        let authorizer = bindings::Universe::instantiate(&mut store, world.authorizer(), &linker)
            .expect("failed to instantiate authorizer component");
        let wasmtime = Wasmtime {
            instruction,
            authorizer,
            store,
        };

        Init {
            authority,
            wasmtime,
        }
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
            .instruction
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
    pub fn read_approval(self, world: &impl crate::state::WorldState) -> Result<Reading, ()> {
        let ToRead {
            authority,
            mut wasmtime,
            request,
        } = self;
        let permission = world.permission(&authority);
        let permission = bindings::AllowSet::from((permission, authority.clone()));

        let verdict = wasmtime
            .authorizer
            .call_read_approval(&mut wasmtime.store, &request, &permission)
            .expect("failed to call read_approval function");
        if !verdict {
            return Err(());
        }

        Ok(Reading {
            authority,
            wasmtime,
            request,
            permission,
        })
    }
}

pub struct Reading {
    authority: host::AccountK,
    wasmtime: Wasmtime,
    request: bindings::ReadSet,
    permission: bindings::AllowSet,
}

impl Reading {
    pub fn read(self, world: &impl crate::state::WorldState) -> Result<HasRead, ()> {
        let Reading {
            authority,
            wasmtime,
            request,
            permission,
        } = self;
        let request = host::ReadSet::from(request);
        println!("Reading request: {:#?}", &request);
        let result = world.read(&request).into();

        Ok(HasRead {
            authority,
            wasmtime,
            result,
            permission,
        })
    }
}

pub struct HasRead {
    authority: host::AccountK,
    wasmtime: Wasmtime,
    result: bindings::ViewSet,
    permission: bindings::AllowSet,
}

impl HasRead {
    pub fn write_request(self) -> ToWrite {
        let args = self.wasmtime.store.data().host.args.clone();
        let HasRead {
            authority,
            mut wasmtime,
            result,
            permission,
        } = self;
        let request = wasmtime
            .instruction
            .call_write_request(&mut wasmtime.store, &result, &args)
            .expect("failed to call write_request function");

        ToWrite {
            authority,
            wasmtime,
            permission,
            request,
        }
    }
}

pub struct ToWrite {
    authority: host::AccountK,
    wasmtime: Wasmtime,
    request: bindings::WriteSet,
    permission: bindings::AllowSet,
}

impl ToWrite {
    pub fn write_approval(self) -> Result<Writing, ()> {
        let ToWrite {
            authority,
            mut wasmtime,
            request,
            permission,
        } = self;
        let intent = bindings::EventSet::from(&request);

        let verdict = wasmtime
            .authorizer
            .call_write_approval(&mut wasmtime.store, &intent, &permission)
            .expect("failed to call write_approval function");
        if !verdict {
            return Err(());
        }

        Ok(Writing { authority, request })
    }
}

pub struct Writing {
    authority: host::AccountK,
    request: bindings::WriteSet,
}

impl Writing {
    pub fn write(self, world: &mut impl crate::state::WorldState) -> Result<HasWritten, ()> {
        let Writing { authority, request } = self;
        let request = host::WriteSet::from(request);
        println!("Writing request: {:#?}", &request);
        world.write(&request, authority.clone());
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
