use serde::Deserialize;

wit_bindgen::generate!({
    world: "command",
    path: "../../wit",
});

use poc::wit::{general, read, view, write};

struct SupplyAll;

#[derive(Debug, Deserialize)]
struct Args {
    // Name of the asset to supply
    asset: String,
    // Account balances below this threshold will be supplied
    threshold: u32,
    // Amount to supply to accounts
    supply_amount: u32,
    // The supplier account
    supplier: String,
}

impl Guest for SupplyAll {
    fn read_request(_ctx: Context, args: Json) -> ReadSet {
        let args: Args = serde_json::from_str(&args).expect("wrong args");

        vec![read::ReadEntry {
            key: general::FuzzyNodeKey::AccountAsset(general::FuzzyCompositeKey {
                e0: None,
                e1: Some(format!("{}", args.asset)),
            }),
            value: read::NodeValueRead::AccountAsset,
        }]
    }

    fn write_request(view: view::ViewSet, args: Json) -> WriteSet {
        let args: Args = serde_json::from_str(&args).expect("wrong args");

        view.into_iter()
            .filter_map(|entry| {
                #[expect(irrefutable_let_patterns)]
                let view::NodeValueView::AccountAsset(value) = entry.value else {
                    panic!("unexpected value type");
                };
                (value.balance < args.threshold).then(|| {
                    vec![
                        write::WriteEntry {
                            key: entry.key,
                            value: write::NodeValueWrite::AccountAsset(
                                write::AccountAssetW::Receive(args.supply_amount),
                            ),
                        },
                        write::WriteEntry {
                            key: general::NodeKey::AccountAsset(general::CompositeKey {
                                e0: args.supplier.clone(),
                                e1: args.asset.clone(),
                            }),
                            value: write::NodeValueWrite::AccountAsset(write::AccountAssetW::Send(
                                args.supply_amount,
                            )),
                        },
                    ]
                })
            })
            .flatten()
            .collect()
    }
}

export!(SupplyAll);
