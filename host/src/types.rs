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
    pub enum NodeKey {
        AccountAsset((KeyElem, KeyElem)),
    }

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum FuzzyNodeKey {
        AccountAsset((Option<KeyElem>, Option<KeyElem>)),
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum NodeValue<T: Mode> {
        AccountAsset(T::AccountAsset),
    }

    impl NodeKey {
        pub fn super_keys(&self) -> impl Iterator<Item = FuzzyNodeKey> {
            let NodeKey::AccountAsset((account, asset)) = self;
            [
                FuzzyNodeKey::AccountAsset((None, None)),
                FuzzyNodeKey::AccountAsset((None, Some(asset.clone()))),
                FuzzyNodeKey::AccountAsset((Some(account.clone()), None)),
                FuzzyNodeKey::AccountAsset((Some(account.clone()), Some(asset.clone()))),
            ]
            .into_iter()
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
