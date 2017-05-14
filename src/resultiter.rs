#![warn(missing_docs)]

use std::fmt::Debug;
use std::iter::Iterator;
use std::result::Result;

/// This is for internal use.
pub trait _ResultTrait {
    #[allow(missing_docs)]
    type Type;
    #[allow(missing_docs)]
    type ErrType;
    #[allow(missing_docs)]
    fn _unwrap(self) -> Self::Type where Self::ErrType: Debug;
    #[allow(missing_docs)]
    fn _unwrap_or(self, Self::Type) -> Self::Type;
    #[allow(missing_docs)]
    fn _is_ok(&self) -> bool;
    #[allow(missing_docs)]
    fn _is_err(&self) -> bool;
    #[allow(missing_docs)]
    fn _as_result(self) -> Result<Self::Type, Self::ErrType>;
    #[allow(missing_docs)]
    fn _map<U, F>(self, F) -> Result<U, Self::ErrType> where F: FnOnce(Self::Type) -> U;
    #[allow(missing_docs)]
    fn _map_err<F, O>(self, f: O) -> Result<Self::Type, F> where O: FnOnce(Self::ErrType) -> F;
}
impl<T, E> _ResultTrait for Result<T, E> {
    type Type = T;
    type ErrType = E;
    fn _unwrap(self) -> Self::Type where Self::ErrType: Debug { self.unwrap() }
    fn _unwrap_or(self, def: Self::Type) -> Self::Type { self.unwrap_or(def) }
    fn _is_ok(&self) -> bool { self.is_ok() }
    fn _is_err(&self) -> bool { self.is_err() }
    fn _as_result(self) -> Self { self }
    fn _map<U, F>(self, f: F) -> Result<U, Self::ErrType> where F: FnOnce(Self::Type) -> U { self.map(f) }
    fn _map_err<F, O>(self, f: O) -> Result<Self::Type, F> where O: FnOnce(Self::ErrType) -> F { self.map_err(f) }
}

/// Add some methods for the iterator of `Result<T, E>`.
pub trait ResultIter: Iterator where Self: Sized, <Self as Iterator>::Item: _ResultTrait {
    /// Create an iterator which yields the unwrapped value as the next value.
    /// (`Unwrap<I>::next()` will panic if the value is `Err(_)`.)
    fn unwrap(self) -> Unwrap<Self> {
        Unwrap { iter: self }
    }
    /// Create an iterator which yields the unwrapped value or a default as the next value.
    fn unwrap_or(self, def: <<Self as Iterator>::Item as _ResultTrait>::Type) -> UnwrapOr<Self> {
        UnwrapOr { iter: self, def: def }
    }
    /// Count the number of `Ok(_)` in this iterator.
    fn count_ok(self) -> usize {
        self.fold(0usize, |acc, x| acc + if x._is_ok() { 1 } else { 0 })
    }
    /// Count the number of `Err(_)` in this iterator.
    fn count_err(self) -> usize {
        self.fold(0usize, |acc, x| acc + if x._is_err() { 1 } else { 0 })
    }
    /// Count the number of `Ok(_)` and `Err(_)` in this iterator.
    /// Return a tuple as `(number of Ok value, number of Err value)`
    fn count_ok_err(self) -> (usize, usize) {
        self.fold((0usize, 0usize), |acc, x| if x._is_ok() { (acc.0 + 1, acc.1) } else { (acc.0, acc.1 + 1) })
    }
    /// Searches for an element of an iterator that the value is `Ok(_)`.
    /// If each element of the iterator all equal `Err(_)`, it returns `None`.
    fn find_ok(&mut self) -> Option<<<Self as Iterator>::Item as _ResultTrait>::Type> {
        while let Some(e) = self.next() {
            if let Ok(x) = e._as_result() {
                return Some(x);
            }
        }
        None
    }
    /// Searches for an element of an iterator that the value is `Err(_)`.
    /// If each element of the iterator all equal `Ok(_)`, it returns `None`.
    fn find_err(&mut self) -> Option<<<Self as Iterator>::Item as _ResultTrait>::ErrType> {
        while let Some(e) = self.next() {
            if let Err(x) = e._as_result() {
                return Some(x);
            }
        }
        None
    }
    /// Tests if any element of the iterator are `Ok(_)`.
    fn has_ok(&mut self) -> bool {
        self.any(|e| e._is_ok())
    }
    /// Tests if any element of the iterator are `Err(_)`.
    fn has_err(&mut self) -> bool {
        self.any(|e| e._is_err())
    }
    /// Create an iterator which yields unwrapped `Ok(_)` value.
    fn ok_iter(self) -> OkIter<Self> {
        OkIter { iter: self }
    }
    /// Create an iterator which yields unwrapped `Err(_)` value.
    fn err_iter(self) -> ErrIter<Self> {
        ErrIter { iter: self }
    }
}

impl<I: Iterator> ResultIter for I where Self: Sized, <I as Iterator>::Item: _ResultTrait {}

pub struct Unwrap<I> {
    iter: I,
}

impl<I: Iterator> Iterator for Unwrap<I> where I::Item: _ResultTrait, <<I as Iterator>::Item as _ResultTrait>::ErrType: Debug {
    type Item = <<I as Iterator>::Item as _ResultTrait>::Type;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|e| e._unwrap())
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

pub struct OkIter<I> {
    iter: I,
}

impl<I: Iterator> Iterator for OkIter<I> where I::Item: _ResultTrait {
    type Item = <<I as Iterator>::Item as _ResultTrait>::Type;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(e) = self.iter.next() {
                if let Ok(x) = e._as_result() {
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

pub struct ErrIter<I> {
    iter: I,
}

impl<I: Iterator> Iterator for ErrIter<I> where I::Item: _ResultTrait {
    type Item = <<I as Iterator>::Item as _ResultTrait>::ErrType;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(e) = self.iter.next() {
                if let Err(x) = e._as_result() {
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

pub struct UnwrapOr<I: Iterator> where <I as Iterator>::Item: _ResultTrait {
    iter: I,
    def: <<I as Iterator>::Item as _ResultTrait>::Type
}

impl<I: Iterator> Iterator for UnwrapOr<I> where I::Item: _ResultTrait, <I::Item as _ResultTrait>::Type: Clone {
    type Item = <<I as Iterator>::Item as _ResultTrait>::Type;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|e| e._unwrap_or(self.def.clone()))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
