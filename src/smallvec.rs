use crate::{Edit, List};
use smallvec::SmallVec;

impl<Array: smallvec::Array> Edit for SmallVec<Array> {}

impl<Array: smallvec::Array> List for SmallVec<Array> {
    type Item = Array::Item;

    fn len(&self) -> usize {
        SmallVec::len(self)
    }

    fn get(&self, index: usize) -> &Self::Item {
        &self[index]
    }

    fn get_mut(&mut self, index: usize) -> &mut Self::Item {
        &mut self[index]
    }

    fn insert(&mut self, index: usize, item: Self::Item) {
        SmallVec::insert(self, index, item)
    }

    fn remove(&mut self, index: usize) {
        SmallVec::remove(self, index);
    }
}
