pub mod general {
    use std::collections::BTreeMap;

    pub trait Mode {
        // PoC only supports AccountAsset variant
        type AccountAsset: std::fmt::Debug + Clone + PartialEq + Eq;
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Tree<T: Mode>(pub BTreeMap<NodeKey, NodeValue<T>>);

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct FlexTree<T: Mode>(pub BTreeMap<FlexNodeKey, NodeValue<T>>);

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct FuzzyTree<T: Mode>(pub BTreeMap<FuzzyNodeKey, NodeValue<T>>);

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct FlexFuzzyTree<T: Mode>(pub BTreeMap<FlexFuzzyNodeKey, NodeValue<T>>);

    pub type KeyElem = String;
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum FlexKeyElem {
        /// Generic pointer to the current account; resolved to an absolute KeyElem
        This,
        /// Explicit absolute KeyElem
        That(KeyElem),
    }

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct SingleKey(pub KeyElem);
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct FlexSingleKey(pub FlexKeyElem);
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct CompositeKey(pub KeyElem, pub KeyElem);
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct FlexCompositeKey(pub FlexKeyElem, pub KeyElem);

    pub type AccountK = SingleKey;
    pub type ExecutableK = SingleKey;
    pub type PermissionK = SingleKey;
    pub type AccountAssetK = CompositeKey;
    pub type AccountPermissionK = CompositeKey;
    pub type FlexAccountAssetK = FlexCompositeKey;

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct FuzzySingleKey(pub Option<KeyElem>);
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct FlexFuzzySingleKey(pub Option<FlexKeyElem>);
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct FuzzyCompositeKey(pub Option<KeyElem>, pub Option<KeyElem>);
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct FlexFuzzyCompositeKey(pub Option<FlexKeyElem>, pub Option<KeyElem>);

    pub type FuzzyAccountK = FuzzySingleKey;
    pub type FuzzyAccountAssetK = FuzzyCompositeKey;
    pub type FlexFuzzyAccountAssetK = FlexFuzzyCompositeKey;

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum NodeKey {
        AccountAsset(AccountAssetK),
    }

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum FlexNodeKey {
        AccountAsset(FlexAccountAssetK),
    }

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum FuzzyNodeKey {
        AccountAsset(FuzzyAccountAssetK),
    }

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum FlexFuzzyNodeKey {
        AccountAsset(FlexFuzzyAccountAssetK),
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum NodeValue<T: Mode> {
        AccountAsset(T::AccountAsset),
    }

    pub trait Capture {
        type Captured;
        fn captures(&self, candidate: &Self::Captured) -> bool;
    }

    impl Capture for FuzzySingleKey {
        type Captured = SingleKey;
        fn captures(&self, candidate: &Self::Captured) -> bool {
            let FuzzySingleKey(cap) = self;
            cap.as_ref().is_none_or(|cap| candidate.0 == *cap)
        }
    }

    impl Capture for FuzzyCompositeKey {
        type Captured = CompositeKey;
        fn captures(&self, candidate: &Self::Captured) -> bool {
            let FuzzyCompositeKey(cap0, cap1) = self;
            cap0.as_ref().is_none_or(|cap0| candidate.0 == *cap0)
                && cap1.as_ref().is_none_or(|cap1| candidate.1 == *cap1)
        }
    }

    /// Resolves FlexKeyElem::This to absolute KeyElem.
    pub trait Resolve {
        type Resolved;
        fn resolve(self, this: KeyElem) -> Self::Resolved;
    }

    /// Un-resolves absolute KeyElem to FlexKeyElem::That.
    pub trait UnResolve {
        type UnResolved;
        fn unresolve(self) -> Self::UnResolved;
    }

    impl Resolve for FlexKeyElem {
        type Resolved = KeyElem;
        fn resolve(self, this: KeyElem) -> Self::Resolved {
            match self {
                FlexKeyElem::This => this,
                FlexKeyElem::That(that) => that,
            }
        }
    }

    impl UnResolve for KeyElem {
        type UnResolved = FlexKeyElem;
        fn unresolve(self) -> Self::UnResolved {
            FlexKeyElem::That(self)
        }
    }

    impl Resolve for FlexSingleKey {
        type Resolved = SingleKey;
        fn resolve(self, this: KeyElem) -> Self::Resolved {
            SingleKey(self.0.resolve(this))
        }
    }

    impl Resolve for FlexFuzzySingleKey {
        type Resolved = FuzzySingleKey;
        fn resolve(self, this: KeyElem) -> Self::Resolved {
            FuzzySingleKey(self.0.map(|elem| elem.resolve(this)))
        }
    }

    impl Resolve for FlexCompositeKey {
        type Resolved = CompositeKey;
        fn resolve(self, this: KeyElem) -> Self::Resolved {
            CompositeKey(self.0.resolve(this), self.1)
        }
    }

    impl Resolve for FlexFuzzyCompositeKey {
        type Resolved = FuzzyCompositeKey;
        fn resolve(self, this: KeyElem) -> Self::Resolved {
            FuzzyCompositeKey(self.0.map(|elem| elem.resolve(this)), self.1)
        }
    }

    impl Resolve for FlexNodeKey {
        type Resolved = NodeKey;
        fn resolve(self, this: KeyElem) -> Self::Resolved {
            match self {
                FlexNodeKey::AccountAsset(key) => NodeKey::AccountAsset(key.resolve(this)),
            }
        }
    }

    impl Resolve for FlexFuzzyNodeKey {
        type Resolved = FuzzyNodeKey;
        fn resolve(self, this: KeyElem) -> Self::Resolved {
            match self {
                FlexFuzzyNodeKey::AccountAsset(key) => {
                    FuzzyNodeKey::AccountAsset(key.resolve(this))
                }
            }
        }
    }

    impl<T: Mode> Resolve for FlexTree<T> {
        type Resolved = Tree<T>;
        fn resolve(self, this: KeyElem) -> Self::Resolved {
            Tree(
                self.0
                    .into_iter()
                    .map(|(k, v)| (k.resolve(this.clone()), v))
                    .collect(),
            )
        }
    }

    impl<T: Mode> Resolve for FlexFuzzyTree<T> {
        type Resolved = FuzzyTree<T>;
        fn resolve(self, this: KeyElem) -> Self::Resolved {
            FuzzyTree(
                self.0
                    .into_iter()
                    .map(|(k, v)| (k.resolve(this.clone()), v))
                    .collect(),
            )
        }
    }
}

pub mod read {
    use super::general::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Read;

    impl Mode for Read {
        type AccountAsset = AccountAssetR;
    }

    pub type ReadSet = FuzzyTree<Read>;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct AccountAssetR;
}

pub mod view {
    use super::general::*;
    use derive_more::Debug;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct View;

    impl Mode for View {
        type AccountAsset = AccountAssetV;
    }

    pub type ViewSet = Tree<View>;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct AccountAssetV {
        pub balance: u32,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct PermissionV {
        pub permission: super::allow::AllowSet,
    }

    #[derive(Debug, Clone)]
    pub struct ExecutableV {
        #[debug("Wasm Component")]
        pub component: wasmtime::component::Component,
    }

    impl PartialEq for ExecutableV {
        fn eq(&self, _other: &Self) -> bool {
            unimplemented!("PoC does not support equality for ExecutableV")
        }
    }

    impl Eq for ExecutableV {}
}

pub mod write {
    use super::general::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Write;

    impl Mode for Write {
        type AccountAsset = AccountAssetW;
    }

    pub type WriteSet = FlexTree<Write>;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum AccountAssetW {
        Send(u32),
        Receive(u32),
    }
}

pub mod event {
    use super::{general::*, write::AccountAssetW, write::WriteSet};

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Event;

    impl Mode for Event {
        type AccountAsset = AccountAssetE;
    }

    pub type EventSet = Tree<Event>;

    #[derive(Debug, Clone, PartialEq, Eq)]
    #[repr(u8)]
    pub enum AccountAssetE {
        Read = 0b0000_0001,
        Receive = 0b0000_0010,
        Send = 0b0000_0100,
        Mint = 0b0001_0000,
        Burn = 0b0010_0000,
    }

    impl From<(WriteSet, AccountK)> for EventSet {
        fn from((write_set, authority): (WriteSet, AccountK)) -> Self {
            Tree(
                write_set
                    .0
                    .into_iter()
                    .map(|(k, v)| {
                        let value = match v {
                            NodeValue::AccountAsset(AccountAssetW::Send(_)) => {
                                NodeValue::AccountAsset(AccountAssetE::Send)
                            }
                            NodeValue::AccountAsset(AccountAssetW::Receive(_)) => {
                                NodeValue::AccountAsset(AccountAssetE::Receive)
                            }
                        };
                        (k.resolve(authority.0.clone()), value)
                    })
                    .collect(),
            )
        }
    }
}

pub mod allow {
    use super::general::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Allow;

    impl Mode for Allow {
        type AccountAsset = AccountAssetA;
    }

    pub type AllowSet = FlexFuzzyTree<Allow>;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct AccountAssetA {
        pub bit_mask: u8,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::general::Capture;
    use crate::types::general::Resolve;

    #[test]
    fn fuzzy_key_captures() {
        let fuzzy_key = general::FuzzySingleKey(None);
        let candidate = general::SingleKey("test".into());
        assert!(fuzzy_key.captures(&candidate));

        let fuzzy_key = general::FuzzySingleKey(Some("test".into()));
        let candidate = general::SingleKey("test".into());
        assert!(fuzzy_key.captures(&candidate));

        let fuzzy_key = general::FuzzyCompositeKey(None, None);
        let candidate = general::CompositeKey("test1".into(), "test2".into());
        assert!(fuzzy_key.captures(&candidate));

        let fuzzy_key = general::FuzzyCompositeKey(None, Some("test2".into()));
        let candidate = general::CompositeKey("test1".into(), "test2".into());
        assert!(fuzzy_key.captures(&candidate));

        let fuzzy_key = general::FuzzyCompositeKey(Some("test1".into()), None);
        let candidate = general::CompositeKey("test1".into(), "test2".into());
        assert!(fuzzy_key.captures(&candidate));

        let fuzzy_key = general::FuzzyCompositeKey(Some("test1".into()), Some("test2".into()));
        let candidate = general::CompositeKey("test1".into(), "test2".into());
        assert!(fuzzy_key.captures(&candidate));

        let fuzzy_key = general::FuzzyCompositeKey(Some("test1".into()), Some("test2".into()));
        let candidate = general::CompositeKey("test0".into(), "test2".into());
        assert!(!fuzzy_key.captures(&candidate));

        let fuzzy_key = general::FuzzyCompositeKey(None, Some("test2".into()));
        let candidate = general::CompositeKey("test1".into(), "test3".into());
        assert!(!fuzzy_key.captures(&candidate));
    }

    #[test]
    fn flex_key_resolves() {
        let flex_key = general::FlexSingleKey(general::FlexKeyElem::This);
        let resolved_key = flex_key.resolve("current_authority".into());
        assert_eq!(resolved_key, general::SingleKey("current_authority".into()));

        let flex_key =
            general::FlexCompositeKey(general::FlexKeyElem::That("alice".into()), "rose".into());
        let resolved_key = flex_key.resolve("current_authority".into());
        assert_eq!(
            resolved_key,
            general::CompositeKey("alice".into(), "rose".into())
        );
    }
}
