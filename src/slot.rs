use crate::Stride;
use core::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

/// Represents the current position in a mutating iteration of a list.
pub struct Slot<'list, 'stride, List>
where
    List: crate::List + ?Sized,
{
    list: &'list mut List,
    index: usize,
    stride: &'stride mut Stride,
}

impl<'list, 'stride, List> Slot<'list, 'stride, List>
where
    List: crate::List + ?Sized,
{
    pub(crate) fn new(
        list: &'list mut List,
        index: usize,
        stride: &'stride mut Stride,
    ) -> Slot<'list, 'stride, List> {
        Slot {
            list,
            index,
            stride,
        }
    }

    /// Immutably borrow the item at the current position.
    pub fn get(&self) -> &List::Item {
        self.list.get(self.index)
    }

    /// Mutably borrow the item at the current position.
    pub fn get_mut(&mut self) -> &mut List::Item {
        self.list.get_mut(self.index)
    }

    /// Insert an item before the current item.
    ///
    /// ```
    /// # use editer::edit;
    /// #
    /// let mut items = vec![1, 2, 3, 4, 5];
    ///
    /// edit(&mut items, |item| {
    ///     if item == 3 {
    ///         item.insert_before(6);
    ///     }
    /// });
    ///
    /// assert_eq!(items, vec![1, 2, 6, 3, 4, 5]);
    /// ```
    pub fn insert_before(self, item: List::Item) {
        self.list.insert(self.index, item);
        self.stride.set(2);
    }

    /// Insert an item after the current item.
    ///
    /// ```
    /// # use editer::edit;
    /// #
    /// let mut items = vec![1, 2, 3, 4, 5];
    ///
    /// edit(&mut items, |item| {
    ///     if item == 3 {
    ///         item.insert_after(6);
    ///     }
    /// });
    ///
    /// assert_eq!(items, vec![1, 2, 3, 6, 4, 5]);
    /// ```
    pub fn insert_after(self, item: List::Item) {
        self.list.insert(self.index + 1, item);
        self.stride.set(2);
    }

    /// Replace the current item with zero or more `items`.
    ///
    /// ```
    /// # use editer::edit;
    /// #
    /// let mut items = vec![1, 2, 3, 4, 5];
    ///
    /// edit(&mut items, |item| {
    ///     if item == 3 {
    ///         item.replace([6, 7, 8]);
    ///     }
    /// });
    ///
    /// assert_eq!(items, vec![1, 2, 6, 7, 8, 4, 5]);
    /// ```
    pub fn replace<IntoIter>(self, items: impl IntoIterator<IntoIter = IntoIter>)
    where
        IntoIter: Iterator<Item = List::Item> + ExactSizeIterator,
    {
        self.splice(items.into_iter())
    }

    /// Call `build` with the current item. Replace the current item with the zero or more
    /// resulting items.
    ///
    /// ```
    /// # use editer::edit;
    /// #
    /// let mut items = vec![1, 2, 3, 4, 5];
    ///
    /// edit(&mut items, |item| {
    ///     item.replace_with(|item| [
    ///         ((item - 1) * 3) + 1,
    ///         ((item - 1) * 3) + 2,
    ///         ((item - 1) * 3) + 3
    ///     ]);
    /// });
    ///
    /// assert_eq!(items, vec![
    ///      1,  2,  3,
    ///      4,  5,  6,
    ///      7,  8,  9,
    ///     10, 11, 12,
    ///     13, 14, 15
    /// ]);
    /// ```
    ///
    /// This is useful because [`Slot::replace`] taking ownership of `self` means you can't access
    /// the relevant slot in its argument:
    ///
    /// ```compile_fail
    /// # use editer::edit;
    /// #
    /// let mut items = vec![1, 2, 3, 4, 5];
    ///
    /// // error: borrow of moved value: `item`
    /// edit(&mut items, |item| {
    ///     item.replace([
    /// //  ---- value moved here
    ///         ((*item - 1) * 3) + 1,
    /// //        ^^^^^ value borrowed here after move
    ///         ((*item - 1) * 3) + 2,
    ///         ((*item - 1) * 3) + 3
    ///     ]);
    /// });
    /// ```
    pub fn replace_with<Replacements, IntoReplacementsIter>(
        self,
        build: impl FnOnce(&List::Item) -> Replacements,
    ) where
        Replacements: IntoIterator<IntoIter = IntoReplacementsIter>,
        IntoReplacementsIter: Iterator<Item = List::Item> + ExactSizeIterator,
    {
        let replacements = build(&self);
        self.replace(replacements);
    }

    fn splice(self, items: impl Iterator<Item = List::Item> + ExactSizeIterator) {
        self.stride.set(items.len());
        self.list.splice(self.index, items);
    }

    /// Remove the current item.
    ///
    /// ```
    /// # use editer::edit;
    /// #
    /// let mut items = vec![1, 2, 3, 4, 5];
    ///
    /// edit(&mut items, |item| {
    ///     if item == 3 {
    ///         item.remove();
    ///     }
    /// });
    ///
    /// assert_eq!(items, vec![1, 2, 4, 5]);
    /// ```
    pub fn remove(self) {
        self.list.remove(self.index);
        self.stride.set(0);
    }
}

impl<'list, 'stride, List> Deref for Slot<'list, 'stride, List>
where
    List: crate::List + ?Sized,
{
    type Target = List::Item;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<'list, 'stride, List> DerefMut for Slot<'list, 'stride, List>
where
    List: crate::List + ?Sized,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

impl<'list, 'stride, Referent, List> AsRef<Referent> for Slot<'list, 'stride, List>
where
    Referent: ?Sized,
    List: crate::List + ?Sized,
    List::Item: AsRef<Referent>,
{
    fn as_ref(&self) -> &Referent {
        self.deref().as_ref()
    }
}

impl<'list, 'stride, Referent, List> AsMut<Referent> for Slot<'list, 'stride, List>
where
    Referent: ?Sized,
    List: crate::List + ?Sized,
    List::Item: AsMut<Referent>,
{
    fn as_mut(&mut self) -> &mut Referent {
        self.deref_mut().as_mut()
    }
}

impl<'list, 'stride, List> PartialEq<List::Item> for Slot<'list, 'stride, List>
where
    List: crate::List + ?Sized,
    List::Item: PartialEq,
{
    fn eq(&self, other: &List::Item) -> bool {
        self.get().eq(other)
    }
}

impl<'list, 'stride, List> PartialOrd<List::Item> for Slot<'list, 'stride, List>
where
    List: crate::List + ?Sized,
    List::Item: PartialOrd,
{
    fn partial_cmp(&self, other: &List::Item) -> Option<core::cmp::Ordering> {
        self.get().partial_cmp(other)
    }
}

impl<'list, 'stride, List> Display for Slot<'list, 'stride, List>
where
    List: crate::List + ?Sized,
    List::Item: Display,
{
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.get().fmt(formatter)
    }
}

impl<'list, 'stride, List> Debug for Slot<'list, 'stride, List>
where
    List: crate::List + ?Sized,
    List::Item: Debug,
{
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.debug_tuple("Slot").field(self.get()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::Slot;
    use crate::Stride;

    #[test]
    fn deref() {
        let mut list = vec![1, 2, 3, 4, 5];
        let mut stride = Stride(1);
        let slot = Slot::new(&mut list, 2, &mut stride);

        let i: u64 = *slot;

        assert_eq!(i, 3);
    }

    #[test]
    fn deref_mut() {
        let mut list = vec![1, 2, 3, 4, 5];
        let mut stride = Stride(1);
        let mut slot = Slot::new(&mut list, 2, &mut stride);

        *slot = 6;

        assert_eq!(list[2], 6);
    }

    #[test]
    fn as_ref() {
        let mut list = vec![1, 2, 3, 4, 5];
        let mut stride = Stride(1);
        let slot = Slot::new(&mut list, 2, &mut stride);

        fn inner(i: &u64) {
            assert_eq!(*i, 3);
        }

        inner(&slot);
    }

    #[test]
    fn as_mut() {
        let mut list = vec![1, 2, 3, 4, 5];
        let mut stride = Stride(1);
        let mut slot = Slot::new(&mut list, 2, &mut stride);

        fn inner(i: &mut u64) {
            *i = 6;
        }

        inner(&mut slot);

        assert_eq!(list[2], 6);
    }

    #[test]
    fn eq() {
        let mut list = vec![1, 2, 3, 4, 5];
        let mut stride = Stride(1);
        let slot = Slot::new(&mut list, 2, &mut stride);

        assert_eq!(slot, 3);
        assert_ne!(slot, 5);
    }

    #[test]
    fn cmp() {
        let mut list = vec![1, 2, 3, 4, 5];
        let mut stride = Stride(1);
        let slot = Slot::new(&mut list, 2, &mut stride);

        assert!(slot < 5);
        assert!(!(slot > 5));
    }

    #[test]
    fn display() {
        let mut list = vec![1, 2, 3, 4, 5];
        let mut stride = Stride(1);
        let slot = Slot::new(&mut list, 2, &mut stride);

        assert_eq!("3", format!("{}", slot));
    }

    #[test]
    fn debug() {
        let mut list = vec![1, 2, 3, 4, 5];
        let mut stride = Stride(1);
        let slot = Slot::new(&mut list, 2, &mut stride);

        assert_eq!("Slot(3)", format!("{:?}", slot));
    }
}
