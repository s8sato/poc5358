use std::collections::BTreeMap;

use crate::prelude::*;

trait WorldState {
    fn new() -> Self;
    fn read(&self, request: &ReadSet) -> ViewSet;
    fn write(&mut self, request: WriteSet);
}

pub struct World {
    pub account_asset: BTreeMap<AccountAssetK, AccountAssetV>,
}

impl WorldState for World {
    fn new() -> Self {
        Self {
            account_asset: BTreeMap::new(),
        }
    }

    fn read(&self, request: &ReadSet) -> ViewSet {
        let keys: Vec<&AccountAssetK> = self
            .account_asset
            .keys()
            .filter(|key| {
                request
                    .0
                    .keys()
                    .any(|FuzzyNodeKey::AccountAsset(capture)| capture.captures(key))
            })
            .collect();
        let map = self
            .account_asset
            .iter()
            .filter(|(key, _)| keys.contains(key))
            .map(|(k, v)| {
                (
                    NodeKey::AccountAsset(k.clone()),
                    NodeValue::AccountAsset(v.clone()),
                )
            })
            .collect();

        Tree(map)
    }

    fn write(&mut self, request: WriteSet) {
        request
            .0
            .into_iter()
            .for_each(
                |(NodeKey::AccountAsset(k), NodeValue::AccountAsset(v))| match v {
                    AccountAssetW::Receive(amount) => {
                        self.account_asset
                            .entry(k)
                            .and_modify(|existing| {
                                let AccountAssetV { balance } = existing;
                                *balance = balance.saturating_add(amount);
                            })
                            .or_insert_with(|| AccountAssetV { balance: amount });
                    }
                    AccountAssetW::Send(amount) => {
                        self.account_asset
                            .entry(k)
                            .and_modify(|existing| {
                                let AccountAssetV { balance } = existing;
                                *balance = balance.checked_sub(amount).unwrap_or_else(|| {
                                    panic!("Cannot send more than the balance");
                                });
                            })
                            .or_insert_with(|| {
                                panic!("Cannot send from no balance");
                            });
                    }
                },
            )
    }
}
