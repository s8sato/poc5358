use serde::Deserialize;

wit_bindgen::generate!({
    world: "lib",
});

struct SupplyAll;

impl Guest for SupplyAll {
    fn read_request(_ctx: Context, args: Json) -> ReadSet {
        let args: Args = serde_json::from_str(&args).expect("wrong args");

        vec![ReadEntry {
            key: FuzzyNodeKey::AccountAsset((None, Some(format!("{}", args.asset)))),
            value: NodeValueRead::AccountAsset,
        }]
    }

    fn write_request(view: StateView, args: Json) -> WriteSet {
        let args: Args = serde_json::from_str(&args).expect("wrong args");

        view.into_iter()
            .filter_map(|entry| {
                #[expect(irrefutable_let_patterns)]
                let NodeValueView::AccountAsset(value) = entry.value else {
                    panic!("unexpected value type");
                };
                (value.balance < args.threshold).then(|| {
                    vec![
                        WriteEntry {
                            key: entry.key,
                            value: NodeValueWrite::AccountAsset(AccountAssetW::Receive(
                                args.supply_amount,
                            )),
                        },
                        WriteEntry {
                            key: NodeKey::AccountAsset((args.supplier.clone(), args.asset.clone())),
                            value: NodeValueWrite::AccountAsset(AccountAssetW::Send(
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

export!(SupplyAll);
