#![warn(missing_docs)]

use std::iter::Iterator;
use std::option::Option;

/// This is for internal use.
pub trait _OptionTrait {
    #[allow(missing_docs)]
    type Type;
    #[allow(missing_docs)]
    fn _unwrap(self) -> Self::Type;
    #[allow(missing_docs)]
    fn _unwrap_or(self, Self::Type) -> Self::Type;
    #[allow(missing_docs)]
    fn _is_some(&self) -> bool;
    #[allow(missing_docs)]
    fn _is_none(&self) -> bool;
    #[allow(missing_docs)]
    fn _map<U, F>(self, F) -> Option<U> where F: FnOnce(Self::Type) -> U;
}
impl<T> _OptionTrait for Option<T> {
    type Type = T;
    fn _unwrap(self) -> Self::Type { self.unwrap() }
    fn _unwrap_or(self, def: Self::Type) -> Self::Type { self.unwrap_or(def) }
    fn _is_some(&self) -> bool { self.is_some() }
    fn _is_none(&self) -> bool { self.is_none() }
    fn _map<U, F>(self, f: F) -> Option<U> where F: FnOnce(Self::Type) -> U { self.map(f) }
}

/// Add some methods for the iterator of `Option<T>`.
pub trait OptionIter: Iterator where Self: Sized, <Self as Iterator>::Item: _OptionTrait {
    /// Create an iterator which yields the unwrapped value as the next value.
    /// (`Unwrap<I>::next()` will panic if the value is `None`.)
    fn unwrap(self) -> Unwrap<Self> {
        Unwrap { iter: self }
    }
    /// Create an iterator which yields the unwrapped value or a default as the next value.
    fn unwrap_or(self, def: <<Self as Iterator>::Item as _OptionTrait>::Type) -> UnwrapOr<Self> {
        UnwrapOr { iter: self, def: def }
    }
    /// Count the number of `Some(_)` in this iterator.
    fn count_some(self) -> usize {
        self.fold(0usize, |acc, x| acc + if x._is_some() { 1 } else { 0 })
    }
    /// Searches for an element of an iterator that the value is `Some(_)`.
    /// If each element of the iterator all equal `None`, it returns `None`.
    fn find_some(&mut self) -> Option<<<Self as Iterator>::Item as _OptionTrait>::Type> {
        self.find(|e| e._is_some()).map(|x| x._unwrap())
    }
    /// Tests if any element of the iterator are `Some(_)`.
    fn has_some(&mut self) -> bool {
        self.any(|e| e._is_some())
    }
    /// Tests if any element of the iterator are `None`.
    fn has_none(&mut self) -> bool {
        self.any(|e| e._is_none())
    }
    /// Create an iterator which yields unwrapped `Some(_)` value.
    fn some_iter(self) -> SomeIter<Self> {
        SomeIter { iter: self }
    }
}

impl<I: Iterator> OptionIter for I where Self: Sized, <I as Iterator>::Item: _OptionTrait {}

pub struct Unwrap<I> {
    iter: I,
}

impl<I: Iterator> Iterator for Unwrap<I> where I::Item: _OptionTrait {
    type Item = <<I as Iterator>::Item as _OptionTrait>::Type;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|e| e._unwrap())
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

pub struct SomeIter<I> {
    iter: I,
}

impl<I: Iterator> Iterator for SomeIter<I> where I::Item: _OptionTrait {
    type Item = <<I as Iterator>::Item as _OptionTrait>::Type;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(e) = self.iter.next() {
                if e._is_some() {
                    return Some(e._unwrap());
                }
            }
            else { return None; }
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.iter.size_hint().1)
    }
}

pub struct UnwrapOr<I: Iterator> where <I as Iterator>::Item: _OptionTrait {
    iter: I,
    def: <<I as Iterator>::Item as _OptionTrait>::Type
}

impl<I: Iterator> Iterator for UnwrapOr<I> where I::Item: _OptionTrait, <I::Item as _OptionTrait>::Type: Clone {
    type Item = <<I as Iterator>::Item as _OptionTrait>::Type;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|e| e._unwrap_or(self.def.clone()))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
