//! Editer allows mutating a collection in place while iterating over it.
//!
//! The [`edit`] function iterates over a given [`List`]. For each item in the list, it calls a
//! given function with a [`Slot`]. The `Slot` can be used to access the current item and/or mutate
//! the list at the current position. You can:
//!
//! * Insert a new item before or after the current item using [`Slot::insert_before`] or
//!   [`Slot::insert_after`].
//!
//!   ```
//!   use editer::edit;
//!
//!   let mut items = vec![1, 2, 3, 4, 5];
//!
//!   edit(&mut items, |item| {
//!       if item == 2 {
//!           item.insert_after(6);
//!       } else if item == 3 {
//!           item.insert_before(7);
//!       }
//!   });
//!
//!   assert_eq!(items, vec![1, 2, 6, 7, 3, 4, 5]);
//!   ```
//!
//! * Replace the current item with zero or more new items using [`DerefMut::deref_mut`] or
//!   [`Slot::replace`].
//!
//!   ```
//!   # use editer::edit;
//!   #
//!   let mut items = vec![1, 2, 3, 4, 5];
//!
//!   edit(&mut items, |mut item| {
//!       if item == 2 {
//!           *item = 6;
//!       }
//!
//!       if item == 3 {
//!           item.replace([7, 8, 9]);
//!       }
//!   });
//!
//!   assert_eq!(items, vec![1, 6, 7, 8, 9, 4, 5]);
//!   ```
//!
//! * Remove the current item using [`Slot::remove`].
//!
//!   ```
//!   # use editer::edit;
//!   #
//!   let mut items = vec![1, 2, 3, 4, 5];
//!
//!   edit(&mut items, |item| {
//!       if item == 3 {
//!           item.remove();
//!       }
//!   });
//!
//!   assert_eq!(items, vec![1, 2, 4, 5]);
//!   ```
//!
//! [`try_edit`] is the fallible version of `edit`. It applies the given editor function to each
//! item in the given list, like `edit`. It stops at the first error and returns it.
//!
//! ```
//! # use editer::try_edit;
//! #
//! let mut items = vec![1, 2, 3, 4, 5];
//!
//! let result: Result<(), &str> = try_edit(&mut items, |item| {
//!     if item == 4 {
//!         Err("Whoops!")
//!     } else {
//!         item.remove();
//!         Ok(())
//!     }
//! });
//!
//! assert_eq!(result, Err("Whoops!"));
//! assert_eq!(items, vec![4, 5]);
//! ```
//!
//! [`DerefMut::deref_mut`]: core::ops::DerefMut::deref_mut

#![cfg_attr(not(feature = "std"), no_std)]

pub mod slot;
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
