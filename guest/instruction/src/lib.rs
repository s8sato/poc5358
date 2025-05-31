use poc::wit::types::*;
use serde::Deserialize;

wit_bindgen::generate!({
    world: "instruction",
    path: "../../wit",
});

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
    fn read_request(args: String) -> ReadSet {
        let args: Args = serde_json::from_str(&args).expect("wrong args");

        let inner = vec![ReadEntry {
            key: FuzzyNodeKey::AccountAsset(FuzzyCompositeKey {
                e0: None,
                e1: Some(args.asset.to_string()),
            }),
            value: NodeValueRead::AccountAsset,
        }];

        ReadSet { inner }
    }

    fn write_request(view: ViewSet, args: String) -> WriteSet {
        let args: Args = serde_json::from_str(&args).expect("wrong args");

        let inner = view
            .inner
            .into_iter()
            .filter_map(|entry| {
                let NodeValueView::AccountAsset(value) = entry.value;
                (value.balance < args.threshold).then(|| {
                    vec![
                        WriteEntry {
                            key: entry.key,
                            value: NodeValueWrite::AccountAsset(AccountAssetW::Receive(
                                args.supply_amount,
                            )),
                        },
                        WriteEntry {
                            key: NodeKey::AccountAsset(CompositeKey {
                                e0: args.supplier.clone(),
                                e1: args.asset.clone(),
                            }),
                            value: NodeValueWrite::AccountAsset(AccountAssetW::Send(
                                args.supply_amount,
                            )),
                        },
                    ]
                })
            })
            .flatten()
            .collect();

        WriteSet { inner }
    }
}

export!(SupplyAll);
