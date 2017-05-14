#![warn(missing_docs)]

use std::iter::Iterator;
use std::marker::PhantomData;
use std::option::Option;

/// Add some methods for the iterator of `Option<T>`.
pub trait OptionIter<T>: Iterator<Item=Option<T>> where Self: Sized {
    /// Create an iterator which yields the unwrapped value as the next value.
    /// (`Unwrap<I>::next()` will panic if the value is `None`.)
    fn unwrap(self) -> Unwrap<Self, T> {
        Unwrap { iter: self, phantom: PhantomData }
    }
    /// Create an iterator which yields the unwrapped value or a default as the next value.
    fn unwrap_or(self, def: T) -> UnwrapOr<Self, T> {
        UnwrapOr { iter: self, def: def }
    }
    /// Count the number of `Some(_)` in this iterator.
    fn count_some(self) -> usize {
        self.fold(0usize, |acc, x| acc + if x.is_some() { 1 } else { 0 })
    }
    /// Searches for an element of an iterator that the value is `Some(_)`.
    /// If each element of the iterator all equal `None`, it returns `None`.
    fn find_some(&mut self) -> Option<T> {
        self.find(|e| e.is_some()).map(|x| x.unwrap())
    }
    /// Tests if any element of the iterator are `Some(_)`.
    fn has_some(&mut self) -> bool {
        self.any(|e| e.is_some())
    }
    /// Tests if any element of the iterator are `None`.
    fn has_none(&mut self) -> bool {
        self.any(|e| e.is_none())
    }
    /// Create an iterator which yields unwrapped `Some(_)` value.
    fn some_iter(self) -> SomeIter<Self, T> {
        SomeIter { iter: self, phantom: PhantomData }
    }
}

impl<I, T> OptionIter<T> for I where I: Iterator<Item=Option<T>>, Self: Sized {}

pub struct Unwrap<I, T> {
    iter: I,
    phantom: PhantomData<T>,
}

impl<I, T> Iterator for Unwrap<I, T> where I: Iterator<Item=Option<T>> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|e| e.unwrap())
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

pub struct SomeIter<I, T> {
    iter: I,
    phantom: PhantomData<T>,
}

impl<I, T> Iterator for SomeIter<I, T> where I: Iterator<Item=Option<T>> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(e) = self.iter.next() {
                if e.is_some() {
                    return Some(e.unwrap());
                }
            }
            else { return None; }
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.iter.size_hint().1)
    }
}

pub struct UnwrapOr<I: Iterator, T> {
    iter: I,
    def: T
}

impl<I, T> Iterator for UnwrapOr<I, T> where I: Iterator<Item=Option<T>>, T: Clone {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|e| e.unwrap_or(self.def.clone()))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
