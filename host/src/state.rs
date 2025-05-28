use crate::prelude::*;

pub struct State(ViewSet);

impl State {
    pub fn new() -> Self {
        Self(Tree(Default::default()))
    }

    pub fn read(&self, request: &ReadSet) -> ViewSet {
        let inner = self
            .0
            .0
            .clone()
            .into_iter()
            .filter(|(k, _v)| {
                k.super_keys()
                    .any(|super_key| request.0.keys().any(|key| *key == super_key))
            })
            .collect();

        Tree(inner)
    }

    pub fn write(&mut self, request: WriteSet) {
        request.0.into_iter().for_each(|(k, v)| match v {
            NodeValue::AccountAsset(AccountAssetW::Receive(amount)) => {
                self.0
                    .0
                    .entry(k)
                    .and_modify(|existing| {
                        let NodeValue::AccountAsset(AccountAssetV { balance }) = existing;
                        *balance = balance.saturating_add(amount);
                    })
                    .or_insert_with(|| NodeValue::AccountAsset(AccountAssetV { balance: amount }));
            }
            NodeValue::AccountAsset(AccountAssetW::Send(amount)) => {
                self.0
                    .0
                    .entry(k)
                    .and_modify(|existing| {
                        let NodeValue::AccountAsset(AccountAssetV { balance }) = existing;
                        *balance = balance.checked_sub(amount).unwrap_or_else(|| {
                            panic!("Cannot send more than the balance");
                        });
                    })
                    .or_insert_with(|| {
                        panic!("Cannot send from no balance");
                    });
            }
        })
    }
}
