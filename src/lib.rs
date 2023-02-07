#![cfg_attr(not(feature = "std"), no_std)]

mod slot;

mod integrations;

use crate::slot::Slot;

pub trait Edit<Item>: List<Item = Item> {
    fn edit(&mut self, mut edit: impl FnMut(Slot<Self>)) {
        let mut index = 0;

        while index < self.len() {
            let mut stride = Stride(1);
            edit(Slot::new(self, index, &mut stride));
            index += stride.get();
        }
    }

    fn try_edit<Error>(
        &mut self,
        mut edit: impl FnMut(Slot<Self>) -> Result<(), Error>,
    ) -> Result<(), Error> {
        let mut index = 0;

        while index < self.len() {
            let mut stride = Stride(1);
            edit(Slot::new(self, index, &mut stride))?;
            index += stride.get();
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

struct Stride(usize);

impl Stride {
    pub fn set(&mut self, value: usize) {
        self.0 = value
    }

    pub fn get(self) -> usize {
        self.0
    }
}
