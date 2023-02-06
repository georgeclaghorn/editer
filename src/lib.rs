#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
mod alloc;

#[cfg(feature = "arrayvec")]
mod arrayvec;

use core::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

pub trait Edit<Item>: List<Item> + Sized {
    fn edit(&mut self, mut edit: impl FnMut(Slot<Self, Item>)) {
        let mut index = 0;

        while index < self.len() {
            Iteration::new(self, &mut index).perform(&mut edit);
        }
    }
}

#[allow(clippy::len_without_is_empty)]
pub trait List<Item> {
    fn len(&self) -> usize;
    fn get(&self, index: usize) -> &Item;
    fn get_mut(&mut self, index: usize) -> &mut Item;
    fn insert(&mut self, index: usize, item: Item);
    fn splice(&mut self, index: usize, items: impl Iterator<Item = Item>);
}

struct Iteration<'a, List, Item>
where
    List: crate::List<Item>,
{
    list: &'a mut List,
    index: &'a mut usize,
    stride: Stride,
    phantom: PhantomData<Item>,
}

impl<'a, List, Item> Iteration<'a, List, Item>
where
    List: crate::List<Item>,
{
    fn new(list: &'a mut List, index: &'a mut usize) -> Iteration<'a, List, Item> {
        Iteration {
            list,
            index,
            stride: Stride::new(),
            phantom: PhantomData,
        }
    }

    fn perform(mut self, edit: &mut impl FnMut(Slot<List, Item>)) {
        self.apply(edit);
        self.advance();
    }

    fn apply(&mut self, edit: &mut impl FnMut(Slot<List, Item>)) {
        edit(Slot::new(self.list, *self.index, &mut self.stride))
    }

    fn advance(self) {
        *self.index += self.stride.get()
    }
}

struct Stride(usize);

impl Stride {
    fn new() -> Stride {
        Stride(1)
    }

    fn get(self) -> usize {
        self.0
    }

    fn set(&mut self, value: usize) {
        self.0 = value
    }
}

pub struct Slot<'a, List, Item>
where
    List: crate::List<Item>,
{
    list: &'a mut List,
    index: usize,
    stride: &'a mut Stride,
    phantom: PhantomData<Item>,
}

impl<'a, List, Item> Slot<'a, List, Item>
where
    List: crate::List<Item>,
{
    fn new(list: &'a mut List, index: usize, stride: &'a mut Stride) -> Slot<'a, List, Item> {
        Slot {
            list,
            index,
            stride,
            phantom: PhantomData,
        }
    }

    pub fn insert_before(self, item: Item) {
        self.stride.set(2);
        self.list.insert(self.index, item);
    }

    pub fn insert_after(self, item: Item) {
        self.stride.set(2);
        self.list.insert(self.index + 1, item);
    }

    pub fn replace<IntoIter>(self, items: impl IntoIterator<IntoIter = IntoIter>)
    where
        IntoIter: Iterator<Item = Item> + ExactSizeIterator,
    {
        self.splice(items.into_iter())
    }

    pub fn splice(self, items: impl Iterator<Item = Item> + ExactSizeIterator) {
        self.stride.set(items.len());
        self.list.splice(self.index, items);
    }

    pub fn remove(self) {
        self.replace([])
    }
}

impl<'a, List, Item> Deref for Slot<'a, List, Item>
where
    List: crate::List<Item>,
{
    type Target = Item;

    fn deref(&self) -> &Item {
        self.list.get(self.index)
    }
}

impl<'a, List, Item> DerefMut for Slot<'a, List, Item>
where
    List: crate::List<Item>,
{
    fn deref_mut(&mut self) -> &mut Item {
        self.list.get_mut(self.index)
    }
}
