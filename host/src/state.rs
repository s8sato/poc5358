use std::collections::BTreeMap;

use crate::prelude::*;

pub trait WorldState {
    fn read(&self, request: &ReadSet) -> ViewSet;
    fn write(&mut self, request: &WriteSet, authority: AccountK);
}

#[expect(dead_code)]
pub struct World {
    pub permission: BTreeMap<PermissionK, PermissionV>,
    pub account_asset: BTreeMap<AccountAssetK, AccountAssetV>,
    pub account_permission: BTreeMap<AccountPermissionK, ()>,
}

impl WorldState for World {
    fn read(&self, request: &ReadSet) -> ViewSet {
        let keys: Vec<&AccountAssetK> = self
            .account_asset
            .keys()
            .filter(|key| {
                println!("Checking key: {key:?}");
                request
                    .0
                    .keys()
                    .inspect(|fuzzy_key| {
                        println!("Against fuzzy key: {fuzzy_key:?}");
                    })
                    .any(|FuzzyNodeKey::AccountAsset(capture)| {
                        let captured = capture.captures(key);
                        println!("Captured: {captured}");
                        captured
                    })
            })
            .collect();
        println!("Keys to read: {:#?}", &keys);
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
        println!("Read map: {:#?}", &map);

        Tree(map)
    }

    fn write(&mut self, request: &WriteSet, authority: AccountK) {
        request.0.iter().for_each(
            |(FlexNodeKey::AccountAsset(k), NodeValue::AccountAsset(v))| match v {
                AccountAssetW::Receive(amount) => {
                    self.account_asset
                        .entry(k.clone().resolve(authority.0.clone()))
                        .and_modify(|existing| {
                            let AccountAssetV { balance } = existing;
                            println!("Adding amount: {k:?}");
                            println!("Current balance: {}", &balance);
                            *balance = balance.saturating_add(*amount);
                            println!("New balance: {balance}");
                        })
                        .or_insert_with(|| AccountAssetV { balance: *amount });
                }
                AccountAssetW::Send(amount) => {
                    self.account_asset
                        .entry(k.clone().resolve(authority.0.clone()))
                        .and_modify(|existing| {
                            let AccountAssetV { balance } = existing;
                            println!("Subtracting amount: {k:?}");
                            println!("Current balance: {}", &balance);
                            *balance = balance.checked_sub(*amount).unwrap_or_else(|| {
                                panic!("Cannot send more than the balance");
                            });
                            println!("New balance: {balance}");
                        })
                        .or_insert_with(|| {
                            panic!("Cannot send from no balance");
                        });
                }
            },
        )
    }
}
