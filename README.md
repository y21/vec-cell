# `vec_cell`
This crate exposes a safe interface for `UnsafeCell<Vec<T>>`. It's like `RefCell<Vec<T>>` but without the runtime cost!

This is useful in situations where you need to mutate a `Vec<T>` but you only have a shared reference (`&`) to it.
For example `Rc<Vec<i32>>` only hands out `&Vec<i32>`, which will not let you add elements to it. Instead, use `Rc<VecCell<i32>>`!
```rs
let nums = Rc::new(vec_cell![1, 2, 3]);
nums.push(4);

for num in nums.iter() {
    // Every element is cloned.
    // That is fine for `i32`s.

    println!("{num}");
}
```

It does so by not ever handing out references to the inner vector or any of its elements.
The downside is that calling `get()` **clones** the inner element, which is why this is only of use if your `T` is cheaply cloneable (e.g. numbers, `Rc`, etc.).

If you do need a reference, there is an unsafe `get_ref(index)` method exposed.
You can also always use the unsafe methods `as_ref()` and `as_mut()` to get a reference to the inner vector (or a mutable reference respectively). Do note, though, that the no-aliasing rule still holds, so the following is UB:
```rs
let nums: VecCell<i32> = vec_cell![1, 2, 3];
let nums_ref: &Vec<i32> = unsafe { nums.as_ref() };
nums.push(4); // call to push creates &mut Vec<i32>, even though a `&Vec<i32>` exists!
```
Keep this in mind when using unsafe methods of this crate, or when questioning design decisions of this crate.

Like `RefCell` and `Cell`, `VecCell` is not thread-safe.
