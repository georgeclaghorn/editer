//! Editer allows mutating a collection in place while iterating over it.
//!
//! The [`edit`] function iterates over a given [`List`]. For each item in the list, it calls a
//! given function with a [`Slot`]. The `Slot` can be used to access the current item and/or mutate
//! the list at the current position. You can:
//!
//! * Insert zero or more new items before or after the current item using [`Slot::insert_before`]
//!   or [`Slot::insert_after`].
//!
//!   ```
//!   use editer::edit;
//!
//!   let mut items = vec![1, 2, 3, 4, 5];
//!
//!   edit(&mut items, |item| {
//!       if item == 2 {
//!           item.insert_after([6, 7]);
//!       } else if item == 3 {
//!           item.insert_before([8, 9]);
//!       }
//!   });
//!
//!   assert_eq!(items, vec![1, 2, 6, 7, 8, 9, 3, 4, 5]);
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
//! use editer::try_edit;
//!
//! let mut items = vec![1, 2, 3, 4, 5];
//!
//! let result = try_edit(&mut items, |item| {
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
//! ## Optional features
//!
//! Implementations of `List` for third-party types are provided behind optional features:
//!
//! * The **`alloc`** feature, which is enabled by default, implements `List` for [`Vec`] and
//!   [`VecDeque`].
//! * The **`arrayvec`** feature implements `List` for [`arrayvec::ArrayVec`].
//! * The **`smallvec`** feature implements `List` for [`smallvec::SmallVec`].
//! * The **`tinyvec`** feature implements `List` for [`tinyvec::ArrayVec`] and
//!   [`tinyvec::TinyVec`].
//!
//! Editer can be used without the standard library by disabling default features.
//!
//! [`DerefMut::deref_mut`]: core::ops::DerefMut::deref_mut
//! [`VecDeque`]: https://doc.rust-lang.org/alloc/collections/vec_deque/struct.VecDeque.html
//! [`arrayvec::ArrayVec`]: https://docs.rs/arrayvec/latest/arrayvec/struct.ArrayVec.html
//! [`smallvec::SmallVec`]: https://docs.rs/smallvec/latest/smallvec/struct.SmallVec.html
//! [`tinyvec::ArrayVec`]: https://docs.rs/tinyvec/latest/tinyvec/struct.ArrayVec.html
//! [`tinyvec::TinyVec`]: https://docs.rs/tinyvec/latest/tinyvec/enum.TinyVec.html

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod slot;
use self::slot::Slot;

mod integrations;

/// Iterates over `items`, calling `edit` with a [`Slot`] for each item. The `Slot` allows
/// accessing the current item and/or updating the list at the current position.
pub fn edit<List>(items: &mut List, mut edit: impl FnMut(Slot<List>))
where
    List: self::List + ?Sized,
{
    let mut index = 0;

    while index < items.len() {
        let mut stride = Stride(1);
        edit(Slot::new(items, index, &mut stride));
        index += stride.get();
    }
}

/// The fallible version of [`edit`].
///
/// Iterates over `items`, calling `edit` with a [`Slot`] for each item. The `Slot` allows
/// accessing the current item and/or updating the list at the current position.
///
/// Stops at the first error and returns it.
pub fn try_edit<List, Error>(
    items: &mut List,
    mut edit: impl FnMut(Slot<List>) -> Result<(), Error>,
) -> Result<(), Error>
where
    List: self::List + ?Sized,
{
    let mut index = 0;

    while index < items.len() {
        let mut stride = Stride(1);
        edit(Slot::new(items, index, &mut stride))?;
        index += stride.get();
    }

    Ok(())
}

/// Allows calling [`edit`] and [`try_edit`] as methods on [`List`]s rather than free functions.
pub trait Edit: List {
    /// Calls [`edit`] on `self`.
    ///
    /// ```
    /// use editer::Edit;
    ///
    /// let mut items = vec![1, 2, 3, 4, 5];
    ///
    /// items.edit(|item| {
    ///     if item == 3 {
    ///         item.remove();
    ///     }
    /// });
    ///
    /// assert_eq!(items, vec![1, 2, 4, 5]);
    /// ```
    fn edit(&mut self, edit: impl FnMut(Slot<Self>)) {
        crate::edit(self, edit)
    }

    /// Calls [`try_edit`] on `self`.
    ///
    /// ```
    /// use editer::Edit;
    ///
    /// let mut items = vec![1, 2, 3, 4, 5];
    ///
    /// let result = items.try_edit(|item| {
    ///     if item == 4 {
    ///         Err("Whoops!")
    ///     } else {
    ///         item.remove();
    ///         Ok(())
    ///     }
    /// });
    ///
    /// assert_eq!(result, Err("Whoops!"));
    /// assert_eq!(items, vec![4, 5]);
    /// ```
    fn try_edit<Error>(
        &mut self,
        edit: impl FnMut(Slot<Self>) -> Result<(), Error>,
    ) -> Result<(), Error> {
        crate::try_edit(self, edit)
    }
}

impl<List> Edit for List where List: crate::List + ?Sized {}

/// A homogeneous collection that can be mutated in place while iterating.
#[allow(clippy::len_without_is_empty)]
pub trait List {
    /// The type of each item in the list.
    type Item;

    /// Returns the number of items in the list, also referred to as its ‘length’.
    fn len(&self) -> usize;

    /// Returns a shared reference to the item at `index`, panicking if `index` is out of bounds.
    fn index(&self, index: usize) -> &Self::Item;

    /// Returns a mutable reference to the item at `index`, panicking if `index` is out of bounds.
    fn index_mut(&mut self, index: usize) -> &mut Self::Item;

    /// Inserts `items` at `index`.
    fn insert(&mut self, index: usize, items: impl Iterator<Item = Self::Item> + ExactSizeIterator);

    /// Removes the item at `index`.
    fn remove(&mut self, index: usize);

    /// Replaces the item at `index` with the zero or more `items`.
    fn replace(
        &mut self,
        index: usize,
        mut items: impl Iterator<Item = Self::Item> + ExactSizeIterator,
    ) {
        if let Some(item) = items.next() {
            *self.index_mut(index) = item;

            self.insert(index + 1, items);
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
