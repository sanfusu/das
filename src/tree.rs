use core::{cmp::Ord, iter::Iterator};
use core::{mem::MaybeUninit, usize};
pub trait TreeChild<'a> {
    fn as_tree<T: Tree<'a>>(&self) -> &'a T;
}

pub trait Tree<'a> {
    type Child: TreeChild<'a>;
    fn height(&self) -> usize;
    fn is_leaf(&self) -> bool;
    fn childs<T>(&'a self) -> T
    where
        T: Iterator<Item = &'a Self::Child>;
    fn preorder_iter<T>(&'a self) -> T
    where
        T: Iterator<Item = &'a Self::Child>;
    fn postorder_iter<T>(&'a self) -> T
    where
        T: Iterator<Item = &'a Self::Child>;
    fn inorder_iter<T>(&'a self) -> T
    where
        T: Iterator<Item = &'a Self::Child>;
}

pub struct FullBinaryTreeArray<T: Default + Ord, const N: usize> {
    pub value: [T; N],
    pub len: usize,
}
pub struct Pos {
    pub idx: usize,
}
pub enum DasError {
    ContainerIsFull,
}
impl<T: Default + Ord, const N: usize> FullBinaryTreeArray<T, N> {
    pub fn new() -> Self {
        let mut value: [T; N] = unsafe { MaybeUninit::zeroed().assume_init() };
        value.iter_mut().for_each(|x| *x = Default::default());
        Self { value, len: 0 }
    }
    pub fn right_child_index(p: usize) -> usize {
        2 * p + 1
    }
    pub fn left_child_index(p: usize) -> usize {
        2 * p
    }
    pub fn is_full(&self) -> bool {
        self.len == self.value.len()
    }
    pub fn insert(&mut self, _value: T) -> Result<&mut Self, DasError> {
        if self.is_full() {
            return Err(DasError::ContainerIsFull);
        } else {
            return Ok(self);
        }
    }
    pub fn parent_index(c: usize) -> usize {
        c / 2
    }
    pub fn iter_ancestor() {}
}
