#![warn(missing_docs)]

use std::fmt::Debug;
use std::iter::Iterator;
use std::marker::PhantomData;
use std::result::Result;

/// Add some methods for the iterator of `Result<T, E>`.
pub trait ResultIter<T, E>: Iterator<Item=Result<T, E>> where Self: Sized {
    /// Create an iterator which yields the unwrapped value as the next value.
    /// (`Unwrap<I>::next()` will panic if the value is `Err(_)`.)
    fn unwrap(self) -> Unwrap<Self, T, E> {
        Unwrap { iter: self, phantom: PhantomData }
    }
    /// Create an iterator which yields the unwrapped value or a default as the next value.
    fn unwrap_or(self, def: T) -> UnwrapOr<Self, T, E> {
        UnwrapOr { iter: self, def: def, phantom: PhantomData }
    }
    /// Count the number of `Ok(_)` in this iterator.
    fn count_ok(self) -> usize {
        self.fold(0usize, |acc, x| acc + if x.is_ok() { 1 } else { 0 })
    }
    /// Count the number of `Err(_)` in this iterator.
    fn count_err(self) -> usize {
        self.fold(0usize, |acc, x| acc + if x.is_err() { 1 } else { 0 })
    }
    /// Count the number of `Ok(_)` and `Err(_)` in this iterator.
    /// Return a tuple as `(number of Ok value, number of Err value)`
    fn count_ok_err(self) -> (usize, usize) {
        self.fold((0usize, 0usize), |acc, x| if x.is_ok() { (acc.0 + 1, acc.1) } else { (acc.0, acc.1 + 1) })
    }
    /// Searches for an element of an iterator that the value is `Ok(_)`.
    /// If each element of the iterator all equal `Err(_)`, it returns `None`.
    fn find_ok(&mut self) -> Option<T> {
        while let Some(e) = self.next() {
            if let Ok(x) = e {
                return Some(x);
            }
        }
        None
    }
    /// Searches for an element of an iterator that the value is `Err(_)`.
    /// If each element of the iterator all equal `Ok(_)`, it returns `None`.
    fn find_err(&mut self) -> Option<E> {
        while let Some(e) = self.next() {
            if let Err(x) = e {
                return Some(x);
            }
        }
        None
    }
    /// Tests if any element of the iterator are `Ok(_)`.
    fn has_ok(&mut self) -> bool {
        self.any(|e| e.is_ok())
    }
    /// Tests if any element of the iterator are `Err(_)`.
    fn has_err(&mut self) -> bool {
        self.any(|e| e.is_err())
    }
    /// Create an iterator which yields unwrapped `Ok(_)` value.
    fn ok_iter(self) -> OkIter<Self, T, E> {
        OkIter { iter: self, phantom: PhantomData }
    }
    /// Create an iterator which yields unwrapped `Err(_)` value.
    fn err_iter(self) -> ErrIter<Self, T, E> {
        ErrIter { iter: self, phantom: PhantomData }
    }
}

impl<I, T, E> ResultIter<T, E> for I where I: Iterator<Item=Result<T, E>> + Sized {}

pub struct Unwrap<I, T, E> {
    iter: I,
    phantom: PhantomData<*const (T, E)>,
}

impl<I, T, E> Iterator for Unwrap<I, T, E> where I: Iterator<Item=Result<T, E>>, E: Debug {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|e| e.unwrap())
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

pub struct OkIter<I, T, E> {
    iter: I,
    phantom: PhantomData<*const (T, E)>,
}

impl<I, T, E> Iterator for OkIter<I, T, E> where I: Iterator<Item=Result<T, E>> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(e) = self.iter.next() {
                if let Ok(x) = e {
                    return Some(x);
                }
            }
            else { return None; }
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.iter.size_hint().1)
    }
}

pub struct ErrIter<I, T, E> {
    iter: I,
    phantom: PhantomData<*const (T, E)>,
}

impl<I, T, E> Iterator for ErrIter<I, T, E> where I: Iterator<Item=Result<T, E>> {
    type Item = E;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(e) = self.iter.next() {
                if let Err(x) = e {
                    return Some(x);
                }
            }
            else { return None; }
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.iter.size_hint().1)
    }
}

pub struct UnwrapOr<I, T, E> {
    iter: I,
    def: T,
    phantom: PhantomData<*const E>,
}

impl<I, T, E> Iterator for UnwrapOr<I, T, E> where I: Iterator<Item=Result<T, E>>, T: Clone {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|e| e.unwrap_or(self.def.clone()))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
