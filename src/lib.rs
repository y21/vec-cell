use std::cell::UnsafeCell;
use std::collections::TryReserveError;
use std::ops::RangeBounds;
use std::vec::IntoIter;

mod iter;
pub use iter::Iter;

#[macro_export]
macro_rules! vec_cell {
    ( $( $val:expr ),+ $( , )? ) => {{
        let vc = $crate::VecCell::new();
        vc.extend_from_slice(&[$( $val ),+]);
        vc
    }};
}

/// A `Vec<T>` type that can be mutated with just a shared reference.
#[derive(Debug, Default)]
pub struct VecCell<T> {
    inner: UnsafeCell<Vec<T>>,
}

impl<T: Clone> Clone for VecCell<T> {
    fn clone(&self) -> Self {
        Self {
            inner: UnsafeCell::new(unsafe { self.as_ref().clone() }),
        }
    }
}

macro_rules! delegate_method {
    (#[doc = $d:expr] $m:ident( $( $n:ident : $nt:ty ),* ) -> $t:ty $( where T: $bound:tt )? ) => {
        #[doc = $d]
        #[inline]
        pub fn $m(&self, $( $n: $nt ),*) -> $t
        $(
            where T: $bound
        )?
        {
            unsafe { self.as_mut().$m($( $n ),*) }
        }
    };
}

macro_rules! delegate_vec_methods {
    ($( $m:ident( $( $n:ident : $nt:ty ),* ) -> $t:ty $( where T: $bound:tt )? ),*) => {
        $(
            delegate_method! {
                #[doc = concat!(" See [Vec::", stringify!($m), "](std::vec::Vec::", stringify!($m), ") for more information.")]
                $m( $( $n : $nt ),* ) -> $t $( where T: $bound )?
            }
        )*
    }
}

macro_rules! delegate_slice_methods {
    ($( $m:ident( $( $n:ident : $nt:ty ),* ) -> $t:ty $( where T: $bound:tt )? ),*) => {
        $(
            delegate_method! {
                #[doc = concat!(" See [slice::", stringify!($m), "](slice::", stringify!($m), ") for more information.")]
                $m( $( $n : $nt ),* ) -> $t $( where T: $bound )?
            }
        )*
    }
}

impl<T> VecCell<T> {
    pub fn new() -> Self {
        Self {
            inner: UnsafeCell::new(Vec::new()),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: UnsafeCell::new(Vec::with_capacity(capacity)),
        }
    }

    #[inline]
    pub unsafe fn as_ref(&self) -> &Vec<T> {
        &*self.inner.get()
    }

    #[inline]
    pub unsafe fn as_mut(&self) -> &mut Vec<T> {
        &mut *self.inner.get()
    }

    #[inline]
    pub unsafe fn get_ref(&self, index: usize) -> Option<&T> {
        self.as_ref().get(index)
    }

    #[inline]
    pub fn into_inner(self) -> Vec<T> {
        self.inner.into_inner()
    }

    #[inline]
    pub fn iter(&self) -> iter::Iter<'_, T>
    where
        T: Clone,
    {
        iter::Iter::new(self)
    }

    pub fn get(&self, index: usize) -> Option<T>
    where
        T: Clone,
    {
        unsafe { self.as_ref().get(index).cloned() }
    }

    pub fn first(&self) -> Option<T>
    where
        T: Clone,
    {
        unsafe { self.as_ref().first().cloned() }
    }

    pub fn last(&self) -> Option<T>
    where
        T: Clone,
    {
        unsafe { self.as_ref().last().cloned() }
    }

    pub fn drain_collect<R: RangeBounds<usize>>(&self, range: R) -> Vec<T> {
        unsafe { self.as_mut().drain(range).collect() }
    }

    pub fn drain<R: RangeBounds<usize>>(&self, range: R) {
        unsafe { drop(self.as_mut().drain(range)) }
    }

    delegate_vec_methods! {
        capacity() -> usize,
        insert(index: usize, value: T) -> (),
        pop() -> Option<T>,
        push(value: T) -> (),
        reserve(additional: usize) -> (),
        reserve_exact(additional: usize) -> (),
        shrink_to(min_capacity: usize) -> (),
        swap_remove(index: usize) -> T,
        truncate(len: usize) -> (),
        dedup() -> () where T: PartialEq,
        extend(it: impl IntoIterator<Item = T>) -> () where T: Clone,
        extend_from_slice(other: &[T]) -> () where T: Clone,
        remove(index: usize) -> T,
        resize(new_len: usize, value: T) -> () where T: Clone,
        split_off(at: usize) -> Vec<T>,
        try_reserve(additional: usize) -> Result<(), TryReserveError>
    }

    delegate_slice_methods! {
        len() -> usize,
        is_empty() -> bool,
        as_ptr() -> *const T,
        as_mut_ptr() -> *mut T,
        binary_search(x: &T) -> Result<usize, usize> where T: Ord,
        contains(x: &T) -> bool where T: PartialEq,
        fill(value: T) -> () where T: Clone,
        reverse() -> (),
        rotate_left(mid: usize) -> (),
        rotate_right(k: usize) -> (),
        sort() -> () where T: Ord,
        sort_unstable() -> () where T: Ord,
        starts_with(other: &[T]) -> bool where T: PartialEq,
        swap(a: usize, b: usize) -> ()
    }
}

impl<T> From<Vec<T>> for VecCell<T> {
    fn from(vec: Vec<T>) -> Self {
        Self {
            inner: UnsafeCell::new(vec),
        }
    }
}

impl<T> From<VecCell<T>> for Vec<T> {
    fn from(vec_cell: VecCell<T>) -> Self {
        vec_cell.into_inner()
    }
}

impl<T> IntoIterator for VecCell<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_inner().into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let x: VecCell<u8> = vec_cell![1, 2, 3];
        x.push(12);
        x.push(34);

        assert_eq!(x.into_inner().as_slice(), &[1, 2, 3, 12, 34]);
    }
}
