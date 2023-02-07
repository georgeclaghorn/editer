#![cfg_attr(not(feature = "std"), no_std)]

mod slot;

mod integrations;

use crate::slot::Slot;
use tap::TapFallible;

pub trait Edit: List {
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

struct Iteration<'a, List: crate::List + ?Sized> {
    list: &'a mut List,
    index: &'a mut usize,
    stride: Stride,
}

impl<'a, List: crate::List + ?Sized> Iteration<'a, List> {
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
