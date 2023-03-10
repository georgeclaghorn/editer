use crate::List;
use smallvec::SmallVec;

#[cfg_attr(docsrs, doc(cfg(feature = "smallvec")))]
impl<Item, Array> List for SmallVec<Array>
where
    Array: smallvec::Array<Item = Item>,
{
    type Item = Array::Item;

    fn len(&self) -> usize {
        SmallVec::len(self)
    }

    fn index(&self, index: usize) -> &Self::Item {
        &self[index]
    }

    fn index_mut(&mut self, index: usize) -> &mut Self::Item {
        &mut self[index]
    }

    fn insert(&mut self, index: usize, items: impl Iterator<Item = Item> + ExactSizeIterator) {
        SmallVec::reserve(self, items.len());

        for (offset, item) in items.enumerate() {
            SmallVec::insert(self, index + offset, item);
        }
    }

    fn remove(&mut self, index: usize) {
        SmallVec::remove(self, index);
    }
}

#[cfg(test)]
mod tests {
    use crate::edit;
    use smallvec::SmallVec;

    #[test]
    fn replacing_the_first_item_with_one() {
        let mut items = SmallVec::from([1, 2, 3, 4, 5]);

        edit(&mut items, |mut item| {
            if item == 1 {
                *item = 6;
            }
        });

        assert_eq!(items, SmallVec::from([6, 2, 3, 4, 5]));
    }

    #[test]
    fn replacing_an_interior_item_with_one() {
        let mut items = SmallVec::from([1, 2, 3, 4, 5]);

        edit(&mut items, |mut item| {
            if item == 3 {
                *item = 6;
            }
        });

        assert_eq!(items, SmallVec::from([1, 2, 6, 4, 5]));
    }

    #[test]
    fn replacing_the_last_item_with_one() {
        let mut items = SmallVec::from([1, 2, 3, 4, 5]);

        edit(&mut items, |mut item| {
            if item == 5 {
                *item = 6;
            }
        });

        assert_eq!(items, SmallVec::from([1, 2, 3, 4, 6]));
    }

    #[test]
    fn replacing_the_first_item_with_many() {
        let mut items = SmallVec::from([1, 2, 3, 4, 5]);

        edit(&mut items, |item| {
            if *item == 1 {
                item.replace([6, 7, 8]);
            }
        });

        assert_eq!(items, SmallVec::from([6, 7, 8, 2, 3, 4, 5]));
    }

    #[test]
    fn replacing_an_interior_item_with_many() {
        let mut items = SmallVec::from([1, 2, 3, 4, 5]);

        edit(&mut items, |item| {
            if item == 3 {
                item.replace([6, 7, 8]);
            }
        });

        assert_eq!(items, SmallVec::from([1, 2, 6, 7, 8, 4, 5]));
    }

    #[test]
    fn replacing_the_last_item_with_many() {
        let mut items = SmallVec::from([1, 2, 3, 4, 5]);

        edit(&mut items, |item| {
            if item == 5 {
                item.replace([6, 7, 8]);
            }
        });

        assert_eq!(items, SmallVec::from([1, 2, 3, 4, 6, 7, 8]));
    }

    #[test]
    fn borrowing_and_replacing_the_first_item_with_many() {
        let mut items = SmallVec::from([1, 2, 3, 4, 5]);

        edit(&mut items, |item| {
            if item == 1 {
                item.replace_with(|item| [item + 5, item + 6, item + 7]);
            }
        });

        assert_eq!(items, SmallVec::from([6, 7, 8, 2, 3, 4, 5]));
    }

    #[test]
    fn borrowing_and_replacing_an_interior_item_with_many() {
        let mut items = SmallVec::from([1, 2, 3, 4, 5]);

        edit(&mut items, |item| {
            if item == 3 {
                item.replace_with(|item| [item + 3, item + 4, item + 5]);
            }
        });

        assert_eq!(items, SmallVec::from([1, 2, 6, 7, 8, 4, 5]));
    }

    #[test]
    fn borrowing_and_replacing_the_last_item_with_many() {
        let mut items = SmallVec::from([1, 2, 3, 4, 5]);

        edit(&mut items, |item| {
            if item == 5 {
                item.replace_with(|item| [item + 1, item + 2, item + 3]);
            }
        });

        assert_eq!(items, SmallVec::from([1, 2, 3, 4, 6, 7, 8]));
    }

    #[test]
    fn removing_the_first_item() {
        let mut items = SmallVec::from([1, 2, 3, 4, 5]);

        edit(&mut items, |item| {
            if *item == 1 {
                item.remove();
            }
        });

        assert_eq!(items, SmallVec::from([2, 3, 4, 5]));
    }

    #[test]
    fn removing_an_interior_item() {
        let mut items = SmallVec::from([1, 2, 3, 4, 5]);

        edit(&mut items, |item| {
            if item == 3 {
                item.remove();
            }
        });

        assert_eq!(items, SmallVec::from([1, 2, 4, 5]));
    }

    #[test]
    fn removing_the_last_item() {
        let mut items = SmallVec::from([1, 2, 3, 4, 5]);

        edit(&mut items, |item| {
            if item == 5 {
                item.remove();
            }
        });

        assert_eq!(items, SmallVec::from([1, 2, 3, 4]));
    }

    #[test]
    fn inserting_items_before_the_first_item() {
        let mut items = SmallVec::from([1, 2, 3, 4, 5]);

        edit(&mut items, |item| {
            if item == 1 {
                item.insert_before([6, 7, 8]);
            }
        });

        assert_eq!(items, SmallVec::from([6, 7, 8, 1, 2, 3, 4, 5]));
    }

    #[test]
    fn inserting_items_before_an_interior_item() {
        let mut items = SmallVec::from([1, 2, 3, 4, 5]);

        edit(&mut items, |item| {
            if item == 3 {
                item.insert_before([6, 7, 8]);
            }
        });

        assert_eq!(items, SmallVec::from([1, 2, 6, 7, 8, 3, 4, 5]));
    }

    #[test]
    fn inserting_items_before_the_last_item() {
        let mut items = SmallVec::from([1, 2, 3, 4, 5]);

        edit(&mut items, |item| {
            if item == 5 {
                item.insert_before([6, 7, 8]);
            }
        });

        assert_eq!(items, SmallVec::from([1, 2, 3, 4, 6, 7, 8, 5]));
    }

    #[test]
    fn inserting_an_item_after_the_first_item() {
        let mut items = SmallVec::from([1, 2, 3, 4, 5]);

        edit(&mut items, |item| {
            if item == 1 {
                item.insert_after([6, 7, 8]);
            }
        });

        assert_eq!(items, SmallVec::from([1, 6, 7, 8, 2, 3, 4, 5]));
    }

    #[test]
    fn inserting_items_after_an_interior_item() {
        let mut items = SmallVec::from([1, 2, 3, 4, 5]);

        edit(&mut items, |item| {
            if item == 3 {
                item.insert_after([6, 7, 8]);
            }
        });

        assert_eq!(items, SmallVec::from([1, 2, 3, 6, 7, 8, 4, 5]));
    }

    #[test]
    fn inserting_items_after_the_last_item() {
        let mut items = SmallVec::from([1, 2, 3, 4, 5]);

        edit(&mut items, |item| {
            if item == 5 {
                item.insert_after([6, 7, 8]);
            }
        });

        assert_eq!(items, SmallVec::from([1, 2, 3, 4, 5, 6, 7, 8]));
    }
}
