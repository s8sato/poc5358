use std::marker::PhantomData;

wit_bindgen::generate!({
    world: "common",
    path: "../../wit",
    // additional_derives: [Debug, Clone, PartialEq, Eq],
});

use exports::poc::wit;

mod general {
    use super::*;
    use wit::general;

    pub struct General;

    impl general::Guest for General {
        type HostNodeKey = host::NodeKey;
    }

    impl general::GuestHostNodeKey for host::NodeKey {
        fn new(_wit: general::NodeKey) -> Self {
            host::NodeKey
        }

        fn as_wit(&self) -> general::NodeKey {
            general::NodeKey::AccountAsset((String::new(), String::new()))
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
