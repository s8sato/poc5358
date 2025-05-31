mod bindings;
mod command;
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
    fn it_works() {
        let engine = wasmtime::Engine::default();
        let component =
            component::Component::from_file(&engine, "../target/wasm32-wasip2/debug/command.wasm")
                .expect("failed to load component");

        let mut world = {
            let account_asset = [
                (
                    CompositeKey("alice".into(), "rose".into()),
                    AccountAssetV { balance: 500 },
                ),
                (
                    CompositeKey("bob".into(), "rose".into()),
                    AccountAssetV { balance: 110 },
                ),
                (
                    CompositeKey("carol".into(), "rose".into()),
                    AccountAssetV { balance: 100 },
                ),
                (
                    CompositeKey("dave".into(), "rose".into()),
                    AccountAssetV { balance: 90 },
                ),
                (
                    CompositeKey("eve".into(), "rose".into()),
                    AccountAssetV { balance: 80 },
                ),
            ]
            .into();
            state::World { account_asset }
        };
        let supply_all = command::WasmCommand {
            component,
            args: serde_json::json!({
                "asset": "rose",
                "threshold": 100,
                "supply_amount": 50,
                "supplier": "alice"
            })
            .to_string(),
        };

        command::initiate(supply_all, &engine)
            .read_request()
            .read_approval()
            .expect("read request rejected")
            .read(&world)
            .expect("failed to read")
            .write_request()
            .write_approval()
            .expect("write request rejected")
            .write(&mut world)
            .expect("failed to write");

        let expected = [
            (
                CompositeKey("alice".into(), "rose".into()),
                AccountAssetV { balance: 400 },
            ),
            (
                CompositeKey("bob".into(), "rose".into()),
                AccountAssetV { balance: 110 },
            ),
            (
                CompositeKey("carol".into(), "rose".into()),
                AccountAssetV { balance: 100 },
            ),
            (
                CompositeKey("dave".into(), "rose".into()),
                AccountAssetV { balance: 140 },
            ),
            (
                CompositeKey("eve".into(), "rose".into()),
                AccountAssetV { balance: 130 },
            ),
        ];

        assert_eq!(world.account_asset, expected.into());
    }
}
