use crate::Stride;
use core::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

pub struct Slot<'a, List: crate::List + ?Sized> {
    list: &'a mut List,
    index: usize,
    stride: &'a mut Stride,
}

impl<'a, List: crate::List + ?Sized> Slot<'a, List> {
    pub(crate) fn new(list: &'a mut List, index: usize, stride: &'a mut Stride) -> Slot<'a, List> {
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

impl<'a, List: crate::List + ?Sized> Deref for Slot<'a, List> {
    type Target = List::Item;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<'a, List: crate::List + ?Sized> DerefMut for Slot<'a, List> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

impl<'a, List: crate::List + ?Sized> PartialEq<List::Item> for Slot<'a, List>
where
    List::Item: PartialEq,
{
    fn eq(&self, other: &List::Item) -> bool {
        self.get().eq(other)
    }
}

impl<'a, List: crate::List + ?Sized> PartialOrd<List::Item> for Slot<'a, List>
where
    List::Item: PartialOrd,
{
    fn partial_cmp(&self, other: &List::Item) -> Option<core::cmp::Ordering> {
        self.get().partial_cmp(other)
    }
}

impl<'a, List: crate::List + ?Sized> Display for Slot<'a, List>
where
    List::Item: Display,
{
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.get().fmt(formatter)
    }
}

impl<'a, List: crate::List + ?Sized> Debug for Slot<'a, List>
where
    List::Item: Debug,
{
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.debug_tuple("Slot").field(self.get()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let mut list = vec![1, 2, 3, 4, 5];
        let mut stride = Stride::new();
        let slot = Slot::new(&mut list, 2, &mut stride);

        assert_eq!("3", format!("{}", slot));
    }

    #[test]
    fn debug() {
        let mut list = vec![1, 2, 3, 4, 5];
        let mut stride = Stride::new();
        let slot = Slot::new(&mut list, 2, &mut stride);

        assert_eq!("Slot(3)", format!("{:?}", slot));
    }
}
