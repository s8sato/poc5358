mod bindings;
mod instruction;
mod state;
mod types;

pub mod prelude {
    pub use super::types::{allow::*, event::*, general::*, read::*, view::*, write::*};
}

#[cfg(test)]
#[expect(dead_code)]
mod tests {
    use prelude::{
        AccountAssetA, AccountAssetV, CompositeKey, FlexFuzzyCompositeKey, FlexFuzzyNodeKey,
        FlexFuzzyTree, FlexKeyElem, NodeValue, PermissionV, SingleKey,
    };

    use super::*;
    use std::collections::BTreeMap;
    use std::sync::LazyLock;
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
        supply_all
            .initiate(authority)
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

    static PERMISSION: LazyLock<BTreeMap<SingleKey, PermissionV>> = LazyLock::new(|| {
        [
            (
                SingleKey("almighty".into()),
                PermissionV {
                    permission: FlexFuzzyTree(BTreeMap::from([(
                        // Any (account, asset) pair
                        FlexFuzzyNodeKey::AccountAsset(FlexFuzzyCompositeKey(None, None)),
                        NodeValue::AccountAsset(AccountAssetA {
                            // Can burn, mint, send, receive, and read
                            bit_mask: 0b0011_0111,
                        }),
                    )])),
                },
            ),
            (
                SingleKey("inspector".into()),
                PermissionV {
                    permission: FlexFuzzyTree(BTreeMap::from([(
                        // Any (account, asset) pair
                        FlexFuzzyNodeKey::AccountAsset(FlexFuzzyCompositeKey(None, None)),
                        NodeValue::AccountAsset(AccountAssetA {
                            // Can read
                            bit_mask: 0b0000_0001,
                        }),
                    )])),
                },
            ),
            (
                SingleKey("everyman".into()),
                PermissionV {
                    permission: FlexFuzzyTree(BTreeMap::from([
                        (
                            // Any (account, asset) pair
                            FlexFuzzyNodeKey::AccountAsset(FlexFuzzyCompositeKey(None, None)),
                            NodeValue::AccountAsset(AccountAssetA {
                                // Can receive
                                bit_mask: 0b0000_0010,
                            }),
                        ),
                        (
                            // This account, any asset
                            FlexFuzzyNodeKey::AccountAsset(FlexFuzzyCompositeKey(
                                Some(FlexKeyElem::This),
                                None,
                            )),
                            NodeValue::AccountAsset(AccountAssetA {
                                // Can send, receive, and read
                                bit_mask: 0b0000_0111,
                            }),
                        ),
                    ])),
                },
            ),
        ]
        .into()
    });

    static ACCOUNT_PERMISSION: LazyLock<BTreeMap<CompositeKey, ()>> = LazyLock::new(|| {
        [
            (CompositeKey("alice".into(), "everyman".into()), ()),
            (CompositeKey("bob".into(), "everyman".into()), ()),
            (CompositeKey("carol".into(), "everyman".into()), ()),
            (CompositeKey("dave".into(), "everyman".into()), ()),
            (CompositeKey("eve".into(), "everyman".into()), ()),
        ]
        .into()
    });

    #[test]
    #[ignore = "reason: not implemented yet"]
    fn almighty_reads_and_sends_others() {
        let almighty = SingleKey("alice".into());
        let mut world = state::World {
            // permission: PERMISSION.clone(),
            account_asset: ACCOUNT_ASSET.clone(),
            // account_permission: ACCOUNT_PERMISSION.clone(),
        };
        // world.permission.insert(
        //     (
        //         CompositeKey("alice".into(), "almighty".into()),
        //         (),
        //     ),
        // );

        let supply_all = {
            let engine = wasmtime::Engine::default();
            let component = component::Component::from_file(
                &engine,
                "../target/wasm32-wasip2/debug/instruction.wasm",
            )
            .expect("failed to load component");

            instruction::WasmInstruction {
                component,
                args: serde_json::json!({
                    "asset": "rose",
                    "threshold": 100,
                    "supply_amount": 20,
                    // The almighty can supply from anyone
                    "supplier": "dave"
                })
                .to_string(),
            }
        };

        println!("Initiating instruction");
        supply_all
            .initiate(almighty)
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

        todo!()
    }

    #[test]
    #[ignore = "reason: not implemented yet"]
    fn inspector_reads_but_does_not_send_others() {
        let _inspector = SingleKey("bob".into());

        todo!()
    }

    #[test]
    #[ignore = "reason: not implemented yet"]
    fn everyman_does_not_read_or_send_others() {
        let _everyman = SingleKey("carol".into());

        todo!()
    }
}
