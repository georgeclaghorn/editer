# Editer

Editer allows mutating a collection in place while iterating over it.

**Quick links**

* [API documentation](https://docs.rs/editer)
* [Releases/changelog](https://github.com/georgeclaghorn/editer/releases)
* [`editer` on crates.io](https://crates.io/crates/editer)

## Usage

The [`edit`] function iterates over a given [`List`]. For each item in the list, it calls a given
function with a [`Slot`]. The `Slot` can be used to access the current item and/or mutate the list
at the current position. You can:

* Insert a new item before or after the current item.

  ```rust
  let mut items = vec![1, 2, 3, 4, 5];

  edit(items, |item| {
      if item == 2 {
          item.insert_after(6);
      }

      if item == 3 {
          item.insert_before(7);
      }
  });

  assert_eq!(items, vec![1, 2, 6, 7, 3, 4, 5]);
  ```

* Replace the current item with zero or more new items.

  ```rust
  let mut items = vec![1, 2, 3, 4, 5];

  edit(items, |mut item| {
      if item == 2 {
          *item = 6;
      }

      if item == 3 {
          item.replace([7, 8, 9]);
      }
  });

  assert_eq!(items, vec![1, 6, 7, 8, 9, 4, 5]);
  ```

* Remove the current item.

  ```rust
  let mut items = vec![1, 2, 3, 4, 5];

  edit(items, |item| {
      if item == 3 {
          item.remove();
      }
  });

  assert_eq!(items, vec![1, 6, 7, 8, 9, 4, 5]);
  ```

[`try_edit`] is the fallible version of `edit`. It applies the given editor function to each item
in the given list, like `edit`. It stops at the first error and returns it.

[`edit`]: https://docs.rs/serde_magnus/latest/editer/fn.edit.html
[`try_edit`]: https://docs.rs/serde_magnus/latest/editer/fn.try_edit.html
[`List`]: https://docs.rs/serde_magnus/latest/editer/trait.List.html
[`Slot`]: https://docs.rs/serde_magnus/latest/editer/slot/struct.Slot.html

## Requirements

Editer requires Rust 1.57 or newer.

## License

Editer is distributed under the terms of the MIT License. See `LICENSE` for details.
