wit_bindgen::generate!({
    world: "common",
    path: "../../wit",
    // additional_derives: [Debug, Clone, PartialEq, Eq],
});

use exports::poc::wit::{self, general::*, read::*, view::*, write::*};
use host::prelude as host;

pub struct Common;

impl wit::general::Guest for Common {
    type HostKeyElem = host::KeyElem;
}

impl GuestHostKeyElem for host::KeyElem {
    fn new(wit: KeyElem) -> Self {
        wit
    }

    fn as_wit(&self) -> KeyElem {
        self.clone()
    }
}

impl wit::read::Guest for Common {
    type HostReadSet = host::ReadSet;
}

impl GuestHostReadSet for host::ReadSet {
    fn new(wit: ReadSet) -> Self {
        let inner = wit
            .into_iter()
            .map(|entry| {
                let wit::general::FuzzyNodeKey::AccountAsset(k) = entry.key;
                (
                    host::FuzzyNodeKey::AccountAsset(k),
                    host::NodeValue::AccountAsset(host::AccountAssetR),
                )
            })
            .collect();

        host::FuzzyTree(inner)
    }

    fn as_wit(&self) -> ReadSet {
        self.0
            .clone()
            .into_iter()
            .map(|(key, _value)| {
                let host::FuzzyNodeKey::AccountAsset(k) = key;
                ReadEntry {
                    key: wit::general::FuzzyNodeKey::AccountAsset(k),
                    value: NodeValueRead::AccountAsset,
                }
            })
            .collect()
    }
}

impl wit::view::Guest for Common {
    type HostViewSet = host::ViewSet;
}

impl GuestHostViewSet for host::ViewSet {
    fn new(wit: ViewSet) -> Self {
        let inner = wit
            .into_iter()
            .map(|entry| {
                let wit::general::NodeKey::AccountAsset(k) = entry.key;
                let NodeValueView::AccountAsset(AccountAssetV { balance }) = entry.value;
                (
                    host::NodeKey::AccountAsset(k),
                    host::NodeValue::AccountAsset(host::AccountAssetV { balance }),
                )
            })
            .collect();

        host::Tree(inner)
    }

    fn as_wit(&self) -> ViewSet {
        self.0
            .clone()
            .into_iter()
            .map(|(key, value)| {
                let host::NodeKey::AccountAsset(k) = key;
                let host::NodeValue::AccountAsset(host::AccountAssetV { balance }) = value;
                ViewEntry {
                    key: wit::general::NodeKey::AccountAsset(k),
                    value: NodeValueView::AccountAsset(AccountAssetV { balance }),
                }
            })
            .collect()
    }
}

impl wit::write::Guest for Common {
    type HostWriteSet = host::WriteSet;
}

impl GuestHostWriteSet for host::WriteSet {
    fn new(wit: WriteSet) -> Self {
        let inner = wit
            .into_iter()
            .map(|entry| {
                let wit::general::NodeKey::AccountAsset(k) = entry.key;
                let value = match entry.value {
                    NodeValueWrite::AccountAsset(AccountAssetW::Receive(amount)) => {
                        host::NodeValue::AccountAsset(host::AccountAssetW::Receive(amount))
                    }
                    NodeValueWrite::AccountAsset(AccountAssetW::Send(amount)) => {
                        host::NodeValue::AccountAsset(host::AccountAssetW::Send(amount))
                    }
                };
                (host::NodeKey::AccountAsset(k), value)
            })
            .collect();

        host::Tree(inner)
    }

    fn as_wit(&self) -> WriteSet {
        self.0
            .clone()
            .into_iter()
            .map(|(key, value)| {
                let host::NodeKey::AccountAsset(k) = key;
                let value = match value {
                    host::NodeValue::AccountAsset(host::AccountAssetW::Receive(amount)) => {
                        NodeValueWrite::AccountAsset(AccountAssetW::Receive(amount))
                    }
                    host::NodeValue::AccountAsset(host::AccountAssetW::Send(amount)) => {
                        NodeValueWrite::AccountAsset(AccountAssetW::Send(amount))
                    }
                };
                WriteEntry {
                    key: wit::general::NodeKey::AccountAsset(k),
                    value,
                }
            })
            .collect()
    }
}

export!(Common);
