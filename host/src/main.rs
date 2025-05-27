fn main() {
    // let engine = Engine::default();
    // let component =
    //     Component::from_file(&engine, "../target/wasm32-wasip2/debug/wasm_command.wasm")
    //         .expect("failed to load component");
    // let mut store = Store::new(&engine, ());
    // let mut linker = wasmtime::component::Linker::new(&engine);
    // let instance = linker
    //     .instantiate(&mut store, &component)
    //     .expect("failed to instantiate component");

    // let read_request_fn = instance
    //     .get_typed_func::<(Context, Json), ReadSet>(&mut store, "read_request")
    //     .expect("failed to get read_request function");
    // let write_request_fn = instance
    //     .get_typed_func::<(ViewSet, Json), WriteSet>(&mut store, "write_request")
    //     .expect("failed to get write_request function");

    // let args = serde_json::json!({
    //     "asset": "rose",
    //     "threshold": 100,
    //     "supply_amount": 50,
    //     "supplier": "alice"
    // });
    // let read_request = read_request_fn
    //     .call(&mut store, (Context::default(), args.to_string()))
    //     .expect("failed to call read_request function");
    // dbg!(&read_request);
    // let view_set;
    // let write_request = write_request_fn
    //     .call(&mut store, (view_set, args.to_string()))
    //     .expect("failed to call write_request function");
    // dbg!(&write_request);
}
