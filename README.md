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

* Insert zero or more new items before or after the current item using [`Slot::insert_before`] or
  [`Slot::insert_after`].

  ```rust
  use editer::edit;

  let mut items = vec![1, 2, 3, 4, 5];

  edit(&mut items, |item| {
      if item == 2 {
          item.insert_after([6, 7]);
      } else if item == 3 {
          item.insert_before([8, 9]);
      }
  });

  assert_eq!(items, vec![1, 2, 6, 7, 8, 9, 3, 4, 5]);
  ```

* Replace the current item with zero or more new items using [`DerefMut::deref_mut`] or
  [`Slot::replace`].

  ```rust
  let mut items = vec![1, 2, 3, 4, 5];

  edit(&mut items, |mut item| {
      if item == 2 {
          *item = 6;
      }

      if item == 3 {
          item.replace([7, 8, 9]);
      }
  });

  assert_eq!(items, vec![1, 6, 7, 8, 9, 4, 5]);
  ```

* Remove the current item using [`Slot::remove`].

  ```rust
  let mut items = vec![1, 2, 3, 4, 5];

  edit(&mut items, |item| {
      if item == 3 {
          item.remove();
      }
  });

  assert_eq!(items, vec![1, 2, 4, 5]);
  ```

[`try_edit`] is the fallible version of `edit`. It applies the given editor function to each item
in the given list, like `edit`. It stops at the first error and returns it.

```rust
use editer::try_edit;

let mut items = vec![1, 2, 3, 4, 5];

let result = try_edit(&mut items, |item| {
    if item == 4 {
        Err("Whoops!")
    } else {
        item.remove();
        Ok(())
    }
});

assert_eq!(result, Err("Whoops!"));
assert_eq!(items, vec![4, 5]);
```

### Optional features

Implementations of `List` for third-party types are provided behind optional features:

* The **`alloc`** feature, which is enabled by default, implements `List` for [`Vec`] and
[`VecDeque`].
* The **`arrayvec`** feature implements `List` for [`arrayvec::ArrayVec`].
* The **`smallvec`** feature implements `List` for [`smallvec::SmallVec`].
* The **`tinyvec`** feature implements `List` for [`tinyvec::ArrayVec`] and [`tinyvec::TinyVec`].

Editer can be used without the standard library by disabling default features.

## Requirements

Editer requires Rust 1.57 or newer.

## License

Editer is distributed under the terms of the MIT License. See `LICENSE` for details.

[`edit`]: https://docs.rs/editer/latest/editer/fn.edit.html
[`try_edit`]: https://docs.rs/editer/latest/editer/fn.try_edit.html
[`List`]: https://docs.rs/editer/latest/editer/trait.List.html
[`Slot`]: https://docs.rs/editer/latest/editer/slot/struct.Slot.html
[`Slot::insert_before`]: https://docs.rs/editer/latest/editer/slot/struct.Slot.html#method.insert_before
[`Slot::insert_after`]: https://docs.rs/editer/latest/editer/slot/struct.Slot.html#method.insert_after
[`Slot::replace`]: https://docs.rs/editer/latest/editer/slot/struct.Slot.html#method.replace
[`Slot::remove`]: https://docs.rs/editer/latest/editer/slot/struct.Slot.html#method.remove
[`DerefMut::deref_mut`]: https://doc.rust-lang.org/core/ops/trait.DerefMut.html#tymethod.deref_mut
[`Vec`]: https://doc.rust-lang.org/alloc/vec/struct.Vec.html
[`VecDeque`]: https://doc.rust-lang.org/alloc/collections/vec_deque/struct.VecDeque.html
[`arrayvec::ArrayVec`]: https://docs.rs/arrayvec/latest/arrayvec/struct.ArrayVec.html
[`smallvec::SmallVec`]: https://docs.rs/smallvec/latest/smallvec/struct.SmallVec.html
[`tinyvec::ArrayVec`]: https://docs.rs/tinyvec/latest/tinyvec/struct.ArrayVec.html
[`tinyvec::TinyVec`]: https://docs.rs/tinyvec/latest/tinyvec/enum.TinyVec.html
