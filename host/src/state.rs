use std::collections::BTreeMap;

use crate::prelude::*;

pub trait WorldState {
    fn executable(&self, executable: &ExecutableK) -> Option<&ExecutableV>;
    fn permission(&self, authority: &AccountK) -> AllowSet;
    fn read(&self, request: &ReadSet) -> ViewSet;
    fn write(&mut self, request: &WriteSet, authority: AccountK);
}

pub struct World {
    pub executable: BTreeMap<ExecutableK, ExecutableV>,
    pub permission: BTreeMap<PermissionK, PermissionV>,
    pub account_asset: BTreeMap<AccountAssetK, AccountAssetV>,
    pub account_permission: BTreeMap<AccountPermissionK, ()>,
}

impl WorldState for World {
    fn executable(&self, executable: &ExecutableK) -> Option<&ExecutableV> {
        self.executable.get(executable)
    }

    fn permission(&self, authority: &AccountK) -> AllowSet {
        let permission_keys: Vec<_> = self
            .account_permission
            .keys()
            .filter_map(|key| (key.0 == authority.0).then_some(key.1.clone()))
            .collect();
        let permission_union = self
            .permission
            .iter()
            .filter(|(k, _)| permission_keys.contains(&k.0))
            .map(|(_, v)| v.permission.0.clone())
            .fold(BTreeMap::new(), |mut acc, curr| {
                for (k, v) in curr {
                    acc.entry(k)
                        .and_modify(|e| {
                            let NodeValue::AccountAsset(AccountAssetA { bit_mask: acc }) = e;
                            let NodeValue::AccountAsset(AccountAssetA { bit_mask: curr }) = v;
                            *acc |= curr;
                        })
                        .or_insert(v);
                }
                acc
            });

        FlexFuzzyTree(permission_union)
    }

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
