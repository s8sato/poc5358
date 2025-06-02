use crate::prelude::{self as host, Resolve, UnResolve};
use poc::wit::types::*;

wasmtime::component::bindgen!({
    world: "universe",
    path: "../wit",
    additional_derives: [Clone, PartialEq, Eq, PartialOrd, Ord],
});

impl From<ReadSet> for host::ReadSet {
    fn from(guest_ty: ReadSet) -> Self {
        let inner = guest_ty
            .inner
            .into_iter()
            .map(|entry| {
                let FuzzyNodeKey::AccountAsset(k) = entry.key;
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
            .into_keys()
            .map(|key| {
                let host::FuzzyNodeKey::AccountAsset(host::FuzzyCompositeKey(e0, e1)) = key;
                ReadEntry {
                    key: FuzzyNodeKey::AccountAsset(FuzzyCompositeKey { e0, e1 }),
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
                let NodeKey::AccountAsset(k) = entry.key;
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
                    key: NodeKey::AccountAsset(CompositeKey { e0, e1 }),
                    value: NodeValueView::AccountAsset(AccountAssetV { balance }),
                }
            })
            .collect();
        ViewSet { inner }
    }
}

impl From<WriteSet> for host::WriteSet {
    fn from(guest_ty: WriteSet) -> Self {
        let mut inner = guest_ty.inner;
        inner.sort_unstable();
        inner.dedup_by(|a, b| {
            a.key == b.key
                && match (&a.value, &mut b.value) {
                    (
                        NodeValueWrite::AccountAsset(AccountAssetW::Receive(a)),
                        NodeValueWrite::AccountAsset(AccountAssetW::Receive(b)),
                    ) => {
                        *b += a;
                        true
                    }
                    (
                        NodeValueWrite::AccountAsset(AccountAssetW::Send(a)),
                        NodeValueWrite::AccountAsset(AccountAssetW::Send(b)),
                    ) => {
                        *b += a;
                        true
                    }
                    _ => panic!(
                        "WriteSet aggregation failed: {:?} and {:?} are not compatible",
                        a.value, b.value
                    ),
                }
        });
        let inner = inner
            .into_iter()
            .map(|entry| {
                let NodeKey::AccountAsset(k) = entry.key;
                let value = match entry.value {
                    NodeValueWrite::AccountAsset(AccountAssetW::Receive(amount)) => {
                        host::NodeValue::AccountAsset(host::AccountAssetW::Receive(amount))
                    }
                    NodeValueWrite::AccountAsset(AccountAssetW::Send(amount)) => {
                        host::NodeValue::AccountAsset(host::AccountAssetW::Send(amount))
                    }
                };
                (
                    host::FlexNodeKey::AccountAsset(host::FlexCompositeKey(
                        host::FlexKeyElem::That(k.e0),
                        k.e1,
                    )),
                    value,
                )
            })
            .collect();

        host::FlexTree(inner)
    }
}

impl From<(host::WriteSet, host::AccountK)> for WriteSet {
    fn from((host_ty, authority): (host::WriteSet, host::AccountK)) -> Self {
        let inner = host_ty
            .0
            .clone()
            .into_iter()
            .map(|(key, value)| {
                let host::NodeKey::AccountAsset(host::CompositeKey(e0, e1)) =
                    key.resolve(authority.0.clone());
                let value = match value {
                    host::NodeValue::AccountAsset(host::AccountAssetW::Receive(amount)) => {
                        NodeValueWrite::AccountAsset(AccountAssetW::Receive(amount))
                    }
                    host::NodeValue::AccountAsset(host::AccountAssetW::Send(amount)) => {
                        NodeValueWrite::AccountAsset(AccountAssetW::Send(amount))
                    }
                };
                WriteEntry {
                    key: NodeKey::AccountAsset(CompositeKey { e0, e1 }),
                    value,
                }
            })
            .collect();
        WriteSet { inner }
    }
}

impl From<EventSet> for host::EventSet {
    fn from(guest_ty: EventSet) -> Self {
        let inner = guest_ty
            .inner
            .into_iter()
            .map(|entry| {
                let NodeKey::AccountAsset(k) = entry.key;
                let NodeValueEvent::AccountAsset(status) = entry.value;
                (
                    host::NodeKey::AccountAsset(host::CompositeKey(k.e0, k.e1)),
                    host::NodeValue::AccountAsset(status.into()),
                )
            })
            .collect();

        host::Tree(inner)
    }
}

impl From<AccountAssetE> for host::AccountAssetE {
    fn from(e: AccountAssetE) -> Self {
        match e.status_bit {
            0b0000_0001 => host::AccountAssetE::Read,
            0b0000_0010 => host::AccountAssetE::Receive,
            0b0000_0100 => host::AccountAssetE::Send,
            0b0001_0000 => host::AccountAssetE::Mint,
            0b0010_0000 => host::AccountAssetE::Burn,
            _ => panic!("Invalid AccountAssetE status bit: {:08b}", e.status_bit),
        }
    }
}

impl From<host::EventSet> for EventSet {
    fn from(host_ty: host::EventSet) -> Self {
        let inner = host_ty
            .0
            .clone()
            .into_iter()
            .map(|(key, value)| {
                let host::NodeKey::AccountAsset(host::CompositeKey(e0, e1)) = key;
                let host::NodeValue::AccountAsset(status) = value;
                EventEntry {
                    key: NodeKey::AccountAsset(CompositeKey { e0, e1 }),
                    value: NodeValueEvent::AccountAsset(AccountAssetE {
                        status_bit: status as u8,
                    }),
                }
            })
            .collect();

        EventSet { inner }
    }
}

impl From<AllowSet> for host::AllowSet {
    fn from(guest_ty: AllowSet) -> Self {
        let inner = guest_ty
            .inner
            .into_iter()
            .map(|entry| {
                let FuzzyNodeKey::AccountAsset(k) = entry.key;
                let NodeValueAllow::AccountAsset(AccountAssetA { bit_mask }) = entry.value;
                (
                    host::FlexFuzzyNodeKey::AccountAsset(host::FlexFuzzyCompositeKey(
                        k.e0.map(UnResolve::unresolve),
                        k.e1,
                    )),
                    host::NodeValue::AccountAsset(host::AccountAssetA { bit_mask }),
                )
            })
            .collect();

        host::FlexFuzzyTree(inner)
    }
}

impl From<(host::AllowSet, host::AccountK)> for AllowSet {
    fn from((host_ty, authority): (host::AllowSet, host::AccountK)) -> Self {
        let inner = host_ty
            .0
            .clone()
            .into_iter()
            .map(|(key, value)| {
                let host::FlexFuzzyNodeKey::AccountAsset(host::FlexFuzzyCompositeKey(e0, e1)) = key;
                let host::NodeValue::AccountAsset(host::AccountAssetA { bit_mask }) = value;
                AllowEntry {
                    key: FuzzyNodeKey::AccountAsset(FuzzyCompositeKey {
                        e0: e0.map(|elem| elem.resolve(authority.0.clone())),
                        e1,
                    }),
                    value: NodeValueAllow::AccountAsset(AccountAssetA { bit_mask }),
                }
            })
            .collect();
        AllowSet { inner }
    }
}

impl From<&WriteSet> for EventSet {
    fn from(write_set: &WriteSet) -> Self {
        let inner = write_set
            .inner
            .iter()
            .map(|entry| {
                let value = match entry.value {
                    NodeValueWrite::AccountAsset(AccountAssetW::Send(_)) => {
                        NodeValueEvent::AccountAsset(AccountAssetE {
                            status_bit: 0b0000_00100,
                        })
                    }
                    NodeValueWrite::AccountAsset(AccountAssetW::Receive(_)) => {
                        NodeValueEvent::AccountAsset(AccountAssetE {
                            status_bit: 0b0000_00010,
                        })
                    }
                };
                EventEntry {
                    key: entry.key.clone(),
                    value,
                }
            })
            .collect();

        EventSet { inner }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::{FlexCompositeKey, FlexKeyElem};

    use super::*;

    #[test]
    fn write_set_with_duplicate_intents_aggregates() {
        let write_set = WriteSet {
            inner: vec![
                WriteEntry {
                    key: NodeKey::AccountAsset(CompositeKey {
                        e0: "alice".to_string(),
                        e1: "rose".to_string(),
                    }),
                    value: NodeValueWrite::AccountAsset(AccountAssetW::Receive(10)),
                },
                WriteEntry {
                    key: NodeKey::AccountAsset(CompositeKey {
                        e0: "alice".to_string(),
                        e1: "rose".to_string(),
                    }),
                    value: NodeValueWrite::AccountAsset(AccountAssetW::Receive(20)),
                },
            ],
        };

        let host_write_set: host::WriteSet = write_set.into();
        let key = host::FlexNodeKey::AccountAsset(FlexCompositeKey(
            FlexKeyElem::That("alice".to_string()),
            "rose".to_string(),
        ));
        assert_eq!(host_write_set.0.len(), 1);
        assert_eq!(
            host_write_set.0[&key],
            host::NodeValue::AccountAsset(host::AccountAssetW::Receive(30))
        );
    }

    #[test]
    #[should_panic(expected = "WriteSet aggregation failed")]
    fn write_set_with_contradictory_intents_does_not_aggregate() {
        let write_set = WriteSet {
            inner: vec![
                WriteEntry {
                    key: NodeKey::AccountAsset(CompositeKey {
                        e0: "alice".to_string(),
                        e1: "rose".to_string(),
                    }),
                    value: NodeValueWrite::AccountAsset(AccountAssetW::Receive(10)),
                },
                WriteEntry {
                    key: NodeKey::AccountAsset(CompositeKey {
                        e0: "alice".to_string(),
                        e1: "rose".to_string(),
                    }),
                    value: NodeValueWrite::AccountAsset(AccountAssetW::Send(20)),
                },
            ],
        };

        let _host_write_set: host::WriteSet = write_set.into();
    }
}
