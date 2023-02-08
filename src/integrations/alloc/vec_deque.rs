extern crate alloc;

use crate::List;
use alloc::collections::VecDeque;

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

#[cfg(test)]
mod tests {
    use crate::edit;
    use std::collections::VecDeque;

    #[test]
    fn replacing_the_first_item_with_one() {
        let mut items = VecDeque::from([1, 2, 3, 4, 5]);

        edit(&mut items, |mut item| {
            if item == 1 {
                *item = 6;
            }
        });

        assert_eq!(items, VecDeque::from([6, 2, 3, 4, 5]));
    }

    #[test]
    fn replacing_an_interior_item_with_one() {
        let mut items = VecDeque::from([1, 2, 3, 4, 5]);

        edit(&mut items, |mut item| {
            if item == 3 {
                *item = 6;
            }
        });

        assert_eq!(items, VecDeque::from([1, 2, 6, 4, 5]));
    }

    #[test]
    fn replacing_the_last_item_with_one() {
        let mut items = VecDeque::from([1, 2, 3, 4, 5]);

        edit(&mut items, |mut item| {
            if item == 5 {
                *item = 6;
            }
        });

        assert_eq!(items, VecDeque::from([1, 2, 3, 4, 6]));
    }

    #[test]
    fn replacing_the_first_item_with_many() {
        let mut items = VecDeque::from([1, 2, 3, 4, 5]);

        edit(&mut items, |item| {
            if item == 1 {
                item.replace([6, 7, 8]);
            }
        });

        assert_eq!(items, VecDeque::from([6, 7, 8, 2, 3, 4, 5]));
    }

    #[test]
    fn replacing_an_interior_item_with_many() {
        let mut items = VecDeque::from([1, 2, 3, 4, 5]);

        edit(&mut items, |item| {
            if item == 3 {
                item.replace([6, 7, 8]);
            }
        });

        assert_eq!(items, VecDeque::from([1, 2, 6, 7, 8, 4, 5]));
    }

    #[test]
    fn replacing_the_last_item_with_many() {
        let mut items = VecDeque::from([1, 2, 3, 4, 5]);

        edit(&mut items, |item| {
            if item == 5 {
                item.replace([6, 7, 8]);
            }
        });

        assert_eq!(items, VecDeque::from([1, 2, 3, 4, 6, 7, 8]));
    }

    #[test]
    fn removing_the_first_item() {
        let mut items = VecDeque::from([1, 2, 3, 4, 5]);

        edit(&mut items, |item| {
            if item == 1 {
                item.remove();
            }
        });

        assert_eq!(items, VecDeque::from([2, 3, 4, 5]));
    }

    #[test]
    fn removing_an_interior_item() {
        let mut items = VecDeque::from([1, 2, 3, 4, 5]);

        edit(&mut items, |item| {
            if item == 3 {
                item.remove();
            }
        });

        assert_eq!(items, VecDeque::from([1, 2, 4, 5]));
    }

    #[test]
    fn removing_the_last_item() {
        let mut items = VecDeque::from([1, 2, 3, 4, 5]);

        edit(&mut items, |item| {
            if item == 5 {
                item.remove();
            }
        });

        assert_eq!(items, VecDeque::from([1, 2, 3, 4]));
    }

    #[test]
    fn inserting_an_item_before_the_first_item() {
        let mut items = VecDeque::from([1, 2, 3, 4, 5]);

        edit(&mut items, |item| {
            if item == 1 {
                item.insert_before(6);
            }
        });

        assert_eq!(items, VecDeque::from([6, 1, 2, 3, 4, 5]));
    }

    #[test]
    fn inserting_an_item_before_an_interior_item() {
        let mut items = VecDeque::from([1, 2, 3, 4, 5]);

        edit(&mut items, |item| {
            if item == 3 {
                item.insert_before(6);
            }
        });

        assert_eq!(items, VecDeque::from([1, 2, 6, 3, 4, 5]));
    }

    #[test]
    fn inserting_an_item_before_the_last_item() {
        let mut items = VecDeque::from([1, 2, 3, 4, 5]);

        edit(&mut items, |item| {
            if item == 5 {
                item.insert_before(6);
            }
        });

        assert_eq!(items, VecDeque::from([1, 2, 3, 4, 6, 5]));
    }

    #[test]
    fn inserting_an_item_after_the_first_item() {
        let mut items = VecDeque::from([1, 2, 3, 4, 5]);

        edit(&mut items, |item| {
            if item == 1 {
                item.insert_after(6);
            }
        });

        assert_eq!(items, VecDeque::from([1, 6, 2, 3, 4, 5]));
    }

    #[test]
    fn inserting_an_item_after_an_interior_item() {
        let mut items = VecDeque::from([1, 2, 3, 4, 5]);

        edit(&mut items, |item| {
            if item == 3 {
                item.insert_after(6);
            }
        });

        assert_eq!(items, VecDeque::from([1, 2, 3, 6, 4, 5]));
    }

    #[test]
    fn inserting_an_item_after_the_last_item() {
        let mut items = VecDeque::from([1, 2, 3, 4, 5]);

        edit(&mut items, |item| {
            if item == 5 {
                item.insert_after(6);
            }
        });

        assert_eq!(items, VecDeque::from([1, 2, 3, 4, 5, 6]));
    }
}
