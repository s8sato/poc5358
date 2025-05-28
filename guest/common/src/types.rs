use std::marker::PhantomData;

wit_bindgen::generate!({
    world: "common",
    path: "../../wit",
    // additional_derives: [Debug, Clone, PartialEq, Eq],
});

use exports::poc::wit;
use host::prelude as host;

mod general {
    use super::*;
    use wit::general::*;

    pub struct General;

    impl Guest for General {
        type HostNodeKey = host::NodeKey;
    }

    impl GuestHostNodeKey for host::NodeKey {
        fn new(wit: NodeKey) -> Self {
            let NodeKey::AccountAsset((account, asset)) = wit;
            host::NodeKey::AccountAsset((account, asset))
        }

        fn as_wit(&self) -> NodeKey {
            let host::NodeKey::AccountAsset((account, asset)) = self;
            NodeKey::AccountAsset((account.clone(), asset.clone()))
        }
    }
}

use general::General;

export!(General);

pub trait Wit: Sized {
    type WitType: From<Self> + Into<Self>;
}

pub struct Tree<T: Mode>(PhantomData<T>);

pub trait Mode {}

pub struct Read;
pub struct View;
pub struct Write;

impl Mode for Read {}
impl Mode for View {}
impl Mode for Write {}

pub struct Context;
pub type ReadSet = Tree<Read>;
pub type ViewSet = Tree<View>;
pub type WriteSet = Tree<Write>;
