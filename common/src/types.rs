use std::marker::PhantomData;

wit_bindgen::generate!({
    world: "types",
    // additional_derives: [Debug, Clone, PartialEq, Eq],
});

struct Types;

impl exports::wit::types::general::Guest for Types {
    type HostNodeKey = host::NodeKey;
}

impl exports::wit::types::general::GuestHostNodeKey for host::NodeKey {
    fn new(_wit: exports::wit::types::general::NodeKey) -> Self {
        host::NodeKey
    }

    fn as_wit(&self) -> exports::wit::types::general::NodeKey {
        exports::wit::types::general::NodeKey::AccountAsset((String::new(), String::new()))
    }
}

export!(Types);

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
