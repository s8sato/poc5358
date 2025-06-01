mod bindings;
mod instruction;
mod state;
mod types;

pub mod prelude {
    pub use super::types::{allow::*, event::*, general::*, read::*, view::*, write::*};
}

#[cfg(test)]
mod tests {
    use prelude::{
        AccountAssetA, AccountAssetK, AccountAssetV, AccountPermissionK, CompositeKey,
        FlexFuzzyCompositeKey, FlexFuzzyNodeKey, FlexFuzzyTree, FlexKeyElem, NodeValue,
        PermissionK, PermissionV, SingleKey,
    };
    use state::WorldState;

    use super::*;
    use std::collections::BTreeMap;
    use std::sync::LazyLock;
    use wasmtime::component;

    static ACCOUNT_ASSET: LazyLock<BTreeMap<AccountAssetK, AccountAssetV>> = LazyLock::new(|| {
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
            permission: BTreeMap::new(),
            account_asset: ACCOUNT_ASSET.clone(),
            account_permission: BTreeMap::new(),
        };

        let supply_all = {
            let component = component::Component::from_file(
                &ENGINE,
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
        let permission = world.permission(&authority);

        println!("Initiating instruction");
        supply_all
            .initiate(authority, &AUTHORIZER)
            .read_request()
            .read_approval(permission)
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

    static ENGINE: LazyLock<wasmtime::Engine> = LazyLock::new(|| {
        wasmtime::Engine::default()
    });

    static AUTHORIZER: LazyLock<wasmtime::component::Component> = LazyLock::new(|| {
        component::Component::from_file(
            &ENGINE,
            "../target/wasm32-wasip2/debug/authorizer.wasm",
        )
        .expect("component should have been built by: cargo build --target wasm32-wasip2 --manifest-path guest/authorizer/Cargo.toml")
    });

    static PERMISSION: LazyLock<BTreeMap<PermissionK, PermissionV>> = LazyLock::new(|| {
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

    static ACCOUNT_PERMISSION: LazyLock<BTreeMap<AccountPermissionK, ()>> = LazyLock::new(|| {
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
    fn almighty_reads_and_sends_others() {
        let almighty = SingleKey("alice".into());
        let mut world = state::World {
            permission: PERMISSION.clone(),
            account_asset: ACCOUNT_ASSET.clone(),
            account_permission: ACCOUNT_PERMISSION.clone(),
        };
        world
            .account_permission
            .insert(CompositeKey("alice".into(), "almighty".into()), ());

        let supply_all = {
            let component = component::Component::from_file(
                &ENGINE,
                "../target/wasm32-wasip2/debug/instruction.wasm",
            )
            .expect("component should have been built by: cargo build --target wasm32-wasip2 --manifest-path guest/instruction/Cargo.toml");

            instruction::WasmInstruction {
                component,
                args: serde_json::json!({
                    "asset": "rose",
                    "threshold": 100,
                    "supply_amount": 50,
                    // The almighty can supply from anyone
                    "supplier": "bob"
                })
                .to_string(),
            }
        };
        let permission = world.permission(&almighty);

        println!("Initiating instruction");
        supply_all
            .initiate(almighty, &AUTHORIZER)
            .read_request()
            .read_approval(permission)
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
                AccountAssetV { balance: 500 },
            ),
            (
                CompositeKey("bob".into(), "rose".into()),
                AccountAssetV { balance: 0 },
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

    #[test]
    #[ignore = "reason: not implemented yet"]
    fn inspector_reads_but_does_not_send_others() {
        let inspector = SingleKey("alice".into());
        let mut world = state::World {
            permission: PERMISSION.clone(),
            account_asset: ACCOUNT_ASSET.clone(),
            account_permission: ACCOUNT_PERMISSION.clone(),
        };
        world
            .account_permission
            .insert(CompositeKey("alice".into(), "inspector".into()), ());

        let supply_all = {
            let component = component::Component::from_file(
                &ENGINE,
                "../target/wasm32-wasip2/debug/instruction.wasm",
            )
            .expect("component should have been built by: cargo build --target wasm32-wasip2 --manifest-path guest/instruction/Cargo.toml");

            instruction::WasmInstruction {
                component,
                args: serde_json::json!({
                    "asset": "rose",
                    "threshold": 100,
                    "supply_amount": 50,
                    // The inspector cannot supply from others
                    "supplier": "bob"
                })
                .to_string(),
            }
        };
        let permission = world.permission(&inspector);

        println!("Initiating instruction");
        let res = supply_all
            .initiate(inspector, &AUTHORIZER)
            .read_request()
            .read_approval(permission)
            .expect("read request should be approved")
            .read(&world)
            .expect("should read")
            .write_request()
            .write_approval();

        assert!(res.is_err(), "write request should be rejected");

        // No effect on the world state
        let expected = [
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
        ];

        assert_eq!(world.account_asset, expected.into());
    }

    #[test]
    #[ignore = "reason: not implemented yet"]
    fn everyman_does_not_read_or_send_others() {
        let everyman = SingleKey("alice".into());
        let world = state::World {
            permission: PERMISSION.clone(),
            account_asset: ACCOUNT_ASSET.clone(),
            account_permission: ACCOUNT_PERMISSION.clone(),
        };

        let supply_all = {
            let component = component::Component::from_file(
                &ENGINE,
                "../target/wasm32-wasip2/debug/instruction.wasm",
            )
            .expect("component should have been built by: cargo build --target wasm32-wasip2 --manifest-path guest/instruction/Cargo.toml");

            instruction::WasmInstruction {
                component,
                args: serde_json::json!({
                    // The everyman cannot read from others
                    "asset": "rose",
                    "threshold": 100,
                    "supply_amount": 50,
                    // The everyman cannot supply from others
                    "supplier": "bob"
                })
                .to_string(),
            }
        };
        let permission = world.permission(&everyman);

        println!("Initiating instruction");
        let res = supply_all
            .initiate(everyman, &AUTHORIZER)
            .read_request()
            .read_approval(permission);

        assert!(res.is_err(), "read request should be rejected");

        // No effect on the world state
        let expected = [
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
        ];

        assert_eq!(world.account_asset, expected.into());
    }
}
