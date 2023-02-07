extern crate alloc;

use crate::{Edit, List};
use alloc::collections::VecDeque;

impl<Item> Edit<Item> for VecDeque<Item> {}

impl<Item> List for VecDeque<Item> {
    type Item = Item;

    fn len(&self) -> usize {
        VecDeque::len(self)
    }

    fn get(&self, index: usize) -> &Item {
        &self[index]
    }

    fn get_mut(&mut self, index: usize) -> &mut Item {
        &mut self[index]
    }

    fn insert(&mut self, index: usize, item: Item) {
        VecDeque::insert(self, index, item);
    }

    fn remove(&mut self, index: usize) {
        VecDeque::remove(self, index);
    }
}
