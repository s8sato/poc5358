pub mod general {
    use std::collections::BTreeMap;

    pub trait Mode {
        type AccountAsset;
    }

    pub struct Tree<T: Mode>(BTreeMap<NodeKey, NodeValue<T>>);

    pub struct FuzzyTree<T: Mode>(BTreeMap<FuzzyNodeKey, NodeValue<T>>);

    pub type KeyElem = &'static str;

    pub enum NodeKey {
        AccountAsset((KeyElem, KeyElem)),
    }

    pub enum FuzzyNodeKey {
        AccountAsset((Option<KeyElem>, Option<KeyElem>)),
    }

    pub enum NodeValue<T: Mode> {
        AccountAsset(T::AccountAsset),
    }
}

pub mod read {
    use super::general::*;

    pub struct Read;

    impl Mode for Read {
        type AccountAsset = AccountAssetR;
    }

    pub type ReadSet = FuzzyTree<Read>;

    pub struct AccountAssetR;
}

pub mod view {
    use super::general::*;

    pub struct View;

    impl Mode for View {
        type AccountAsset = AccountAssetV;
    }

    pub type ViewSet = Tree<View>;

    pub struct AccountAssetV {
        pub balance: u32,
    }
}

pub mod write {
    use super::general::*;

    pub struct Write;

    impl Mode for Write {
        type AccountAsset = AccountAssetW;
    }

    pub type WriteSet = Tree<Write>;

    pub enum AccountAssetW {
        Send(u32),
        Receive(u32),
    }
}
