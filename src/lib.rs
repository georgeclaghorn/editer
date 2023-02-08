#![cfg_attr(not(feature = "std"), no_std)]

mod slot;
use self::slot::Slot;

mod integrations;

pub fn edit<List>(items: &mut List, mut edit: impl FnMut(Slot<List>))
where
    List: self::List,
{
    let mut index = 0;

    while index < items.len() {
        let mut stride = Stride(1);
        edit(Slot::new(items, index, &mut stride));
        index += stride.get();
    }
}

pub fn try_edit<List, Error>(
    items: &mut List,
    mut edit: impl FnMut(Slot<List>) -> Result<(), Error>,
) -> Result<(), Error>
where
    List: self::List,
{
    let mut index = 0;

    while index < items.len() {
        let mut stride = Stride(1);
        edit(Slot::new(items, index, &mut stride))?;
        index += stride.get();
    }

    Ok(())
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
