use std::marker::PhantomData;

mod command;

pub trait Ffi: Sized {
    type FfiType: From<Self> + Into<Self>;
}

struct Tree<T: Mode>(PhantomData<T>);

trait Mode {}

struct Read;
struct View;
struct Write;

impl Mode for Read {}
impl Mode for View {}
impl Mode for Write {}

type ReadSet = Tree<Read>;
type StateView = Tree<View>;
type WriteSet = Tree<Write>;

struct FfiTreeRead;

struct FfiTreeView;
struct FfiTreeWrite;

impl Ffi for Tree<Read> {
    type FfiType = FfiTreeRead;
}

impl From<Tree<Read>> for FfiTreeRead {
    fn from(_tree: Tree<Read>) -> Self {
        FfiTreeRead
    }
}

impl Into<Tree<Read>> for FfiTreeRead {
    fn into(self) -> Tree<Read> {
        Tree(PhantomData)
    }
}

impl Ffi for Tree<View> {
    type FfiType = View;
}

impl From<Tree<View>> for View {
    fn from(_tree: Tree<View>) -> Self {
        View
    }
}

impl Into<Tree<View>> for View {
    fn into(self) -> Tree<View> {
        Tree(PhantomData)
    }
}

impl Ffi for Tree<Write> {
    type FfiType = Write;
}

impl From<Tree<Write>> for Write {
    fn from(_tree: Tree<Write>) -> Self {
        Write
    }
}

impl Into<Tree<Write>> for Write {
    fn into(self) -> Tree<Write> {
        Tree(PhantomData)
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
