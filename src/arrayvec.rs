use crate::{Edit, List};
use arrayvec::ArrayVec;

impl<Item, const CAP: usize> Edit<Item> for ArrayVec<Item, CAP> {}

impl<Item, const CAP: usize> List<Item> for ArrayVec<Item, CAP> {
    fn len(&self) -> usize {
        ArrayVec::len(self)
    }

    fn get(&self, index: usize) -> &Item {
        &self[index]
    }

    fn get_mut(&mut self, index: usize) -> &mut Item {
        &mut self[index]
    }

    fn insert(&mut self, index: usize, item: Item) {
        ArrayVec::insert(self, index, item)
    }

    fn remove(&mut self, index: usize) {
        ArrayVec::remove(self, index);
    }
}

#[cfg(test)]
mod tests {
    use crate::Edit;
    use arrayvec::ArrayVec;

    #[test]
    fn replacing_the_first_item_with_one() {
        let mut items = ArrayVec::from([1, 2, 3, 4, 5]);

        items.edit(|mut item| {
            if *item == 1 {
                *item = 6;
            }
        });

        assert_eq!(items, ArrayVec::from([6, 2, 3, 4, 5]));
    }

    #[test]
    fn replacing_an_interior_item_with_one() {
        let mut items = ArrayVec::from([1, 2, 3, 4, 5]);

        items.edit(|mut item| {
            if *item == 3 {
                *item = 6;
            }
        });

        assert_eq!(items, ArrayVec::from([1, 2, 6, 4, 5]));
    }

    #[test]
    fn replacing_the_last_item_with_one() {
        let mut items = ArrayVec::from([1, 2, 3, 4, 5]);

        items.edit(|mut item| {
            if *item == 5 {
                *item = 6;
            }
        });

        assert_eq!(items, ArrayVec::from([1, 2, 3, 4, 6]));
    }

    #[test]
    fn replacing_the_first_item_with_many() {
        let mut items: ArrayVec<_, 10> = ArrayVec::from_iter([1, 2, 3, 4, 5].iter().copied());

        items.edit(|item| {
            if *item == 1 {
                item.replace([6, 7, 8])
            }
        });

        assert_eq!(
            items,
            ArrayVec::<_, 10>::from_iter([6, 7, 8, 2, 3, 4, 5].iter().copied())
        );
    }

    #[test]
    fn replacing_an_interior_item_with_many() {
        let mut items: ArrayVec<_, 10> = ArrayVec::from_iter([1, 2, 3, 4, 5].iter().copied());

        items.edit(|item| {
            if *item == 3 {
                item.replace([6, 7, 8]);
            }
        });

        assert_eq!(
            items,
            ArrayVec::<_, 10>::from_iter([1, 2, 6, 7, 8, 4, 5].iter().copied())
        );
    }

    #[test]
    fn replacing_the_last_item_with_many() {
        let mut items: ArrayVec<_, 10> = ArrayVec::from_iter([1, 2, 3, 4, 5].iter().copied());

        items.edit(|item| {
            if *item == 5 {
                item.replace([6, 7, 8]);
            }
        });

        assert_eq!(
            items,
            ArrayVec::<_, 10>::from_iter([1, 2, 3, 4, 6, 7, 8].iter().copied())
        );
    }

    #[test]
    fn removing_the_first_item() {
        let mut items: ArrayVec<_, 10> = ArrayVec::from_iter([1, 2, 3, 4, 5].iter().copied());

        items.edit(|item| {
            if *item == 1 {
                item.remove();
            }
        });

        assert_eq!(
            items,
            ArrayVec::<_, 10>::from_iter([2, 3, 4, 5].iter().copied())
        );
    }

    #[test]
    fn removing_an_interior_item() {
        let mut items: ArrayVec<_, 10> = ArrayVec::from_iter([1, 2, 3, 4, 5].iter().copied());

        items.edit(|item| {
            if *item == 3 {
                item.remove();
            }
        });

        assert_eq!(
            items,
            ArrayVec::<_, 10>::from_iter([1, 2, 4, 5].iter().copied())
        );
    }

    #[test]
    fn removing_the_last_item() {
        let mut items: ArrayVec<_, 10> = ArrayVec::from_iter([1, 2, 3, 4, 5].iter().copied());

        items.edit(|item| {
            if *item == 5 {
                item.remove();
            }
        });

        assert_eq!(
            items,
            ArrayVec::<_, 10>::from_iter([1, 2, 3, 4].iter().copied())
        );
    }

    #[test]
    fn inserting_an_item_before_the_first_item() {
        let mut items: ArrayVec<_, 10> = ArrayVec::from_iter([1, 2, 3, 4, 5].iter().copied());

        items.edit(|item| {
            if *item == 1 {
                item.insert_before(6);
            }
        });

        assert_eq!(
            items,
            ArrayVec::<_, 10>::from_iter([6, 1, 2, 3, 4, 5].iter().copied())
        );
    }

    #[test]
    fn inserting_an_item_before_an_interior_item() {
        let mut items: ArrayVec<_, 10> = ArrayVec::from_iter([1, 2, 3, 4, 5].iter().copied());

        items.edit(|item| {
            if *item == 3 {
                item.insert_before(6);
            }
        });

        assert_eq!(
            items,
            ArrayVec::<_, 10>::from_iter([1, 2, 6, 3, 4, 5].iter().copied())
        );
    }

    #[test]
    fn inserting_an_item_before_the_last_item() {
        let mut items: ArrayVec<_, 10> = ArrayVec::from_iter([1, 2, 3, 4, 5].iter().copied());

        items.edit(|item| {
            if *item == 5 {
                item.insert_before(6);
            }
        });

        assert_eq!(
            items,
            ArrayVec::<_, 10>::from_iter([1, 2, 3, 4, 6, 5].iter().copied())
        );
    }

    #[test]
    fn inserting_an_item_after_the_first_item() {
        let mut items: ArrayVec<_, 10> = ArrayVec::from_iter([1, 2, 3, 4, 5].iter().copied());

        items.edit(|item| {
            if *item == 1 {
                item.insert_after(6);
            }
        });

        assert_eq!(
            items,
            ArrayVec::<_, 10>::from_iter([1, 6, 2, 3, 4, 5].iter().copied())
        );
    }

    #[test]
    fn inserting_an_item_after_an_interior_item() {
        let mut items: ArrayVec<_, 10> = ArrayVec::from_iter([1, 2, 3, 4, 5].iter().copied());

        items.edit(|item| {
            if *item == 3 {
                item.insert_after(6);
            }
        });

        assert_eq!(
            items,
            ArrayVec::<_, 10>::from_iter([1, 2, 3, 6, 4, 5].iter().copied())
        );
    }

    #[test]
    fn inserting_an_item_after_the_last_item() {
        let mut items: ArrayVec<_, 10> = ArrayVec::from_iter([1, 2, 3, 4, 5].iter().copied());

        items.edit(|item| {
            if *item == 5 {
                item.insert_after(6);
            }
        });

        assert_eq!(
            items,
            ArrayVec::<_, 10>::from_iter([1, 2, 3, 4, 5, 6].iter().copied())
        );
    }
}
