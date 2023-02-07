#![cfg_attr(not(feature = "std"), no_std)]

mod integrations;

use core::ops::{Deref, DerefMut};
use tap::TapFallible;

pub trait Edit: List + Sized {
    fn edit(&mut self, mut edit: impl FnMut(Slot<Self>)) {
        let mut index = 0;

        while index < self.len() {
            Iteration::new(self, &mut index).perform(&mut edit);
        }
    }

    fn try_edit<Error>(
        &mut self,
        mut edit: impl FnMut(Slot<Self>) -> Result<(), Error>,
    ) -> Result<(), Error> {
        let mut index = 0;

        while index < self.len() {
            Iteration::new(self, &mut index).try_perform(&mut edit)?;
        }

        Ok(())
    }
}

#[allow(clippy::len_without_is_empty)]
pub trait List {
    type Item;

    fn len(&self) -> usize;
    fn get(&self, index: usize) -> &Self::Item;
    fn get_mut(&mut self, index: usize) -> &mut Self::Item;
    fn insert(&mut self, index: usize, item: Self::Item);
    fn remove(&mut self, index: usize);

    fn splice(&mut self, index: usize, mut items: impl Iterator<Item = Self::Item>) {
        if let Some(item) = items.next() {
            *self.get_mut(index) = item;

            for (offset, item) in items.enumerate() {
                self.insert(index + offset + 1, item);
            }
        } else {
            self.remove(index);
        }
    }
}

struct Iteration<'a, List: crate::List> {
    list: &'a mut List,
    index: &'a mut usize,
    stride: Stride,
}

impl<'a, List: crate::List> Iteration<'a, List> {
    fn new(list: &'a mut List, index: &'a mut usize) -> Iteration<'a, List> {
        Iteration {
            list,
            index,
            stride: Stride::new(),
        }
    }

    fn perform(mut self, edit: &mut impl FnMut(Slot<List>)) {
        self.apply(edit);
        self.advance();
    }

    fn try_perform<Error>(
        mut self,
        edit: &mut impl FnMut(Slot<List>) -> Result<(), Error>,
    ) -> Result<(), Error> {
        self.apply(edit).tap_ok(|_| self.advance())
    }

    fn apply<Output>(&mut self, edit: &mut impl FnMut(Slot<List>) -> Output) -> Output {
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

pub struct Slot<'a, List: crate::List> {
    list: &'a mut List,
    index: usize,
    stride: &'a mut Stride,
}

impl<'a, List: crate::List> Slot<'a, List> {
    fn new(list: &'a mut List, index: usize, stride: &'a mut Stride) -> Slot<'a, List> {
        Slot {
            list,
            index,
            stride,
        }
    }

    pub fn get(&self) -> &List::Item {
        self.list.get(self.index)
    }

    pub fn get_mut(&mut self) -> &mut List::Item {
        self.list.get_mut(self.index)
    }

    pub fn insert_before(self, item: List::Item) {
        self.stride.set(2);
        self.list.insert(self.index, item);
    }

    pub fn insert_after(self, item: List::Item) {
        self.stride.set(2);
        self.list.insert(self.index + 1, item);
    }

    pub fn replace<IntoIter>(self, items: impl IntoIterator<IntoIter = IntoIter>)
    where
        IntoIter: Iterator<Item = List::Item> + ExactSizeIterator,
    {
        self.splice(items.into_iter())
    }

    pub fn splice(self, items: impl Iterator<Item = List::Item> + ExactSizeIterator) {
        self.stride.set(items.len());
        self.list.splice(self.index, items);
    }

    pub fn remove(self) {
        self.stride.set(0);
        self.list.remove(self.index);
    }
}

impl<'a, List: crate::List> Deref for Slot<'a, List> {
    type Target = List::Item;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<'a, List: crate::List> DerefMut for Slot<'a, List> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

impl<'a, List: crate::List> PartialEq<List::Item> for Slot<'a, List>
where
    List::Item: PartialEq,
{
    fn eq(&self, other: &List::Item) -> bool {
        self.get().eq(other)
    }
}

impl<'a, List: crate::List> PartialOrd<List::Item> for Slot<'a, List>
where
    List::Item: PartialOrd,
{
    fn partial_cmp(&self, other: &List::Item) -> Option<core::cmp::Ordering> {
        self.get().partial_cmp(other)
    }
}
