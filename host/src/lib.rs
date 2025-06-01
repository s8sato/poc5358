mod bindings;
mod instruction;
mod state;
mod types;

pub mod prelude {
    pub use super::types::{allow::*, event::*, general::*, read::*, view::*, write::*};
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::sync::LazyLock;
    use types::{general::CompositeKey, general::SingleKey, view::AccountAssetV};
    use wasmtime::component;

    static ACCOUNT_ASSET: LazyLock<BTreeMap<CompositeKey, AccountAssetV>> = LazyLock::new(|| {
        [
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
        .into()
    });

    #[test]
    fn instruction_flows() {
        let mut world = state::World {
            account_asset: ACCOUNT_ASSET.clone(),
        };

        let supply_all = {
            let engine = wasmtime::Engine::default();
            let component = component::Component::from_file(
                &engine,
                "../target/wasm32-wasip2/debug/instruction.wasm",
            )
            .expect("component should have been built by: cargo build --target wasm32-wasip2 --manifest-path guest/instruction/Cargo.toml");

            instruction::WasmInstruction {
                component,
                args: serde_json::json!({
                    "asset": "rose",
                    "threshold": 100,
                    "supply_amount": 50,
                    "supplier": "alice"
                })
                .to_string(),
            }
        };
        let authority = SingleKey("alice".into());

        println!("Initiating instruction");
        instruction::initiate(supply_all, authority)
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
