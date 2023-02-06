extern crate alloc;

use crate::{Edit, List};
use alloc::vec::Vec;

impl<Item> Edit<Item> for Vec<Item> {}

impl<Item> List<Item> for Vec<Item> {
    fn len(&self) -> usize {
        Vec::len(self)
    }

    fn get(&self, index: usize) -> &Item {
        &self[index]
    }

    fn get_mut(&mut self, index: usize) -> &mut Item {
        &mut self[index]
    }

    fn insert(&mut self, index: usize, item: Item) {
        Vec::insert(self, index, item);
    }

    fn splice(&mut self, index: usize, items: impl Iterator<Item = Item>) {
        Vec::splice(self, index..index + 1, items);
    }
}

#[cfg(test)]
mod tests {
    use crate::Edit;

    #[test]
    fn replacing_an_item_with_one() {
        let mut items = vec![1, 2, 3, 4, 5];

        items.edit(|mut item| {
            if *item == 3 {
                *item = 6
            }
        });

        assert_eq!(items, vec![1, 2, 6, 4, 5]);
    }

    #[test]
    fn replacing_an_item_with_many() {
        let mut items = vec![1, 2, 3, 4, 5];

        items.edit(|item| {
            if *item == 3 {
                item.replace([6, 7, 8]);
            }
        });

        assert_eq!(items, vec![1, 2, 6, 7, 8, 4, 5]);
    }

    #[test]
    fn removing_an_item() {
        let mut items = vec![1, 2, 3, 4, 5];

        items.edit(|item| {
            if *item == 3 {
                item.remove();
            }
        });

        assert_eq!(items, vec![1, 2, 4, 5]);
    }

    #[test]
    fn inserting_an_item_before() {
        let mut items = vec![1, 2, 3, 4, 5];

        items.edit(|item| {
            if *item == 3 {
                item.insert_before(6);
            }
        });

        assert_eq!(items, vec![1, 2, 6, 3, 4, 5]);
    }

    #[test]
    fn inserting_an_item_after() {
        let mut items = vec![1, 2, 3, 4, 5];

        items.edit(|item| {
            if *item == 3 {
                item.insert_after(6);
            }
        });

        assert_eq!(items, vec![1, 2, 3, 6, 4, 5]);
    }
}
