use crate::{Edit, List};
use smallvec::SmallVec;

impl<Item, Array: smallvec::Array<Item = Item>> Edit<Item> for SmallVec<Array> {}

impl<Item, Array: smallvec::Array<Item = Item>> List<Item> for SmallVec<Array> {
    fn len(&self) -> usize {
        SmallVec::len(self)
    }

    fn get(&self, index: usize) -> &Item {
        &self[index]
    }

    fn get_mut(&mut self, index: usize) -> &mut Item {
        &mut self[index]
    }

    fn insert(&mut self, index: usize, item: Item) {
        SmallVec::insert(self, index, item)
    }

    fn remove(&mut self, index: usize) {
        SmallVec::remove(self, index);
    }
}
