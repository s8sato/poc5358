use crate::prelude as host;
use poc::wit::{general, read::*, view::*, write::*};

wasmtime::component::bindgen!({
    world: "universe",
    path: "../wit",
    // additional_derives: [Debug, Clone, PartialEq, Eq],
});

impl From<ReadSet> for host::ReadSet {
    fn from(guest_ty: ReadSet) -> Self {
        let inner = guest_ty
            .inner
            .into_iter()
            .map(|entry| {
                let general::FuzzyNodeKey::AccountAsset(k) = entry.key;
                (
                    host::FuzzyNodeKey::AccountAsset(host::FuzzyCompositeKey(k.e0, k.e1)),
                    host::NodeValue::AccountAsset(host::AccountAssetR),
                )
            })
            .collect();

        host::FuzzyTree(inner)
    }
}

impl From<host::ReadSet> for ReadSet {
    fn from(host_ty: host::ReadSet) -> Self {
        let inner = host_ty
            .0
            .clone()
            .into_iter()
            .map(|(key, _value)| {
                let host::FuzzyNodeKey::AccountAsset(host::FuzzyCompositeKey(e0, e1)) = key;
                ReadEntry {
                    key: general::FuzzyNodeKey::AccountAsset(general::FuzzyCompositeKey { e0, e1 }),
                    value: NodeValueRead::AccountAsset,
                }
            })
            .collect();
        ReadSet { inner }
    }
}

impl From<ViewSet> for host::ViewSet {
    fn from(guest_ty: ViewSet) -> Self {
        let inner = guest_ty
            .inner
            .into_iter()
            .map(|entry| {
                let general::NodeKey::AccountAsset(k) = entry.key;
                let NodeValueView::AccountAsset(AccountAssetV { balance }) = entry.value;
                (
                    host::NodeKey::AccountAsset(host::CompositeKey(k.e0, k.e1)),
                    host::NodeValue::AccountAsset(host::AccountAssetV { balance }),
                )
            })
            .collect();

        host::Tree(inner)
    }
}

impl From<host::ViewSet> for ViewSet {
    fn from(host_ty: host::ViewSet) -> Self {
        let inner = host_ty
            .0
            .clone()
            .into_iter()
            .map(|(key, value)| {
                let host::NodeKey::AccountAsset(host::CompositeKey(e0, e1)) = key;
                let host::NodeValue::AccountAsset(host::AccountAssetV { balance }) = value;
                ViewEntry {
                    key: general::NodeKey::AccountAsset(general::CompositeKey { e0, e1 }),
                    value: NodeValueView::AccountAsset(AccountAssetV { balance }),
                }
            })
            .collect();
        ViewSet { inner }
    }
}

impl From<WriteSet> for host::WriteSet {
    fn from(guest_ty: WriteSet) -> Self {
        let inner = guest_ty
            .inner
            .into_iter()
            .map(|entry| {
                let general::NodeKey::AccountAsset(k) = entry.key;
                let value = match entry.value {
                    NodeValueWrite::AccountAsset(AccountAssetW::Receive(amount)) => {
                        host::NodeValue::AccountAsset(host::AccountAssetW::Receive(amount))
                    }
                    NodeValueWrite::AccountAsset(AccountAssetW::Send(amount)) => {
                        host::NodeValue::AccountAsset(host::AccountAssetW::Send(amount))
                    }
                };
                (
                    host::NodeKey::AccountAsset(host::CompositeKey(k.e0, k.e1)),
                    value,
                )
            })
            .collect();

        host::Tree(inner)
    }
}

impl From<host::WriteSet> for WriteSet {
    fn from(host_ty: host::WriteSet) -> Self {
        let inner = host_ty
            .0
            .clone()
            .into_iter()
            .map(|(key, value)| {
                let host::NodeKey::AccountAsset(host::CompositeKey(e0, e1)) = key;
                let value = match value {
                    host::NodeValue::AccountAsset(host::AccountAssetW::Receive(amount)) => {
                        NodeValueWrite::AccountAsset(AccountAssetW::Receive(amount))
                    }
                    host::NodeValue::AccountAsset(host::AccountAssetW::Send(amount)) => {
                        NodeValueWrite::AccountAsset(AccountAssetW::Send(amount))
                    }
                };
                WriteEntry {
                    key: general::NodeKey::AccountAsset(general::CompositeKey { e0, e1 }),
                    value,
                }
            })
            .collect();
        WriteSet { inner }
    }
}
