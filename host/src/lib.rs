mod bindings;
mod instruction;
mod state;
mod types;

mod prelude {
    pub use super::types::{general::*, read::*, view::*, write::*};
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::{general::CompositeKey, view::AccountAssetV};
    use wasmtime::component;

    #[test]
    fn instruction_flows() {
        let engine = wasmtime::Engine::default();
        let component = component::Component::from_file(
            &engine,
            "../target/wasm32-wasip2/debug/instruction.wasm",
        )
        .expect("component should have been built by: cargo build --target wasm32-wasip2 --manifest-path guest/instruction/Cargo.toml");

        let mut world = {
            let account_asset = [
                (
                    CompositeKey("alice".into(), "rose".into()),
                    AccountAssetV { balance: 500 },
                ),
                (
                    CompositeKey("bob".into(), "rose".into()),
                    AccountAssetV { balance: 100 },
                ),
                (
                    CompositeKey("carol".into(), "rose".into()),
                    AccountAssetV { balance: 90 },
                ),
                (
                    CompositeKey("dave".into(), "rose".into()),
                    AccountAssetV { balance: 90 },
                ),
                (
                    CompositeKey("eve".into(), "tulip".into()),
                    AccountAssetV { balance: 90 },
                ),
            ]
            .into();
            state::World { account_asset }
        };
        let supply_all = instruction::WasmInstruction {
            component,
            args: serde_json::json!({
                "asset": "rose",
                "threshold": 100,
                "supply_amount": 50,
                "supplier": "alice"
            })
            .to_string(),
        };

        println!("Initiating instruction");
        instruction::initiate(supply_all, &engine)
            .read_request()
            .read_approval()
            .expect("read request should be approved")
            .read(&world)
            .expect("should read")
            .write_request()
            .write_approval()
            .expect("write request should be approved")
            .write(&mut world)
            .expect("should write");

        let expected = [
            (
                CompositeKey("alice".into(), "rose".into()),
                AccountAssetV { balance: 400 },
            ),
            (
                CompositeKey("bob".into(), "rose".into()),
                AccountAssetV { balance: 100 },
            ),
            (
                CompositeKey("carol".into(), "rose".into()),
                AccountAssetV { balance: 140 },
            ),
            (
                CompositeKey("dave".into(), "rose".into()),
                AccountAssetV { balance: 140 },
            ),
            (
                CompositeKey("eve".into(), "tulip".into()),
                AccountAssetV { balance: 90 },
            ),
        ];

        assert_eq!(world.account_asset, expected.into());
    }
}
