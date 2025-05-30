pub mod general {
    use std::collections::BTreeMap;

    pub trait Mode {
        type AccountAsset: std::fmt::Debug + Clone + PartialEq + Eq;
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Tree<T: Mode>(pub BTreeMap<NodeKey, NodeValue<T>>);

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct FuzzyTree<T: Mode>(pub BTreeMap<FuzzyNodeKey, NodeValue<T>>);

    pub type KeyElem = String;

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct SingleKey(pub KeyElem);
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct CompositeKey(pub KeyElem, pub KeyElem);
    pub type AccountAssetK = CompositeKey;

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct FuzzySingleKey(pub Option<KeyElem>);
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct FuzzyCompositeKey(pub Option<KeyElem>, pub Option<KeyElem>);
    pub type FuzzyAccountAssetK = FuzzyCompositeKey;

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum NodeKey {
        AccountAsset(AccountAssetK),
    }

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum FuzzyNodeKey {
        AccountAsset(FuzzyAccountAssetK),
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
                && cap1.as_ref().is_none_or(|cap1| candidate.0 == *cap1)
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
}

pub mod write {
    use super::general::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Write;

    impl Mode for Write {
        type AccountAsset = AccountAssetW;
    }

    pub type WriteSet = Tree<Write>;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum AccountAssetW {
        Send(u32),
        Receive(u32),
    }
}
