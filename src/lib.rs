mod optioniter;
mod resultiter;
pub use optioniter::OptionIter;
pub use resultiter::ResultIter;

#[cfg(test)]
mod tests {
    use optioniter::OptionIter;
    use resultiter::ResultIter;

    #[test]
    fn options_unwrap() {
        let a: Vec<Option<i32>> = vec![Some(1), Some(2), Some(3)];
        let v: Vec<_> = a.into_iter().unwrap().collect();
        assert_eq!(v, vec![1i32, 2, 3]);
    }

    #[test]
    fn options_unwrap_or() {
        let a: Vec<Option<i32>> = vec![Some(1), None, Some(3)];
        let v: Vec<_> = a.into_iter().unwrap_or(5).collect();
        assert_eq!(v, vec![1i32, 5, 3]);
    }

    #[test]
    fn options_count_some() {
        let a: Vec<Option<i32>> = vec![Some(1), None, Some(3)];
        let n = a.into_iter().count_some();
        assert_eq!(n, 2);
    }

    #[test]
    fn options_find_some() {
        let a: Vec<Option<i32>> = vec![Some(1), None, Some(3)];
        let mut it = a.into_iter();
        assert_eq!(it.find_some(), Some(1));
        assert_eq!(it.find_some(), Some(3));
        assert_eq!(it.find_some(), None);
    }

    #[test]
    fn options_has_some() {
        let a: Vec<Option<i32>> = vec![None, Some(3), None];
        assert_eq!(a.into_iter().has_some(), true);
        let a: Vec<Option<i32>> = vec![None, None, None];
        assert_eq!(a.into_iter().has_some(), false);
    }

    #[test]
    fn options_has_none() {
        let a: Vec<Option<i32>> = vec![Some(3), None, Some(1)];
        assert_eq!(a.into_iter().has_none(), true);
        let a: Vec<Option<i32>> = vec![Some(1), Some(2), Some(3)];
        assert_eq!(a.into_iter().has_none(), false);
    }

    #[test]
    fn options_some_iter() {
        let a: Vec<Option<i32>> = vec![Some(3), None, Some(1)];
        let v: Vec<_> = a.into_iter().some_iter().collect();
        assert_eq!(vec![3i32, 1], v);
    }

    #[test]
    fn results_unwrap() {
        let a: Vec<Result<i32, ()>> = vec![Ok(1), Ok(2), Ok(3)];
        let v: Vec<_> = a.into_iter().unwrap().collect();
        assert_eq!(v, vec![1i32, 2, 3]);
    }

    #[test]
    fn results_unwrap_or() {
        let a: Vec<Result<i32, ()>> = vec![Ok(1), Err(()), Ok(3)];
        let v: Vec<_> = a.into_iter().unwrap_or(5).collect();
        assert_eq!(v, vec![1i32, 5, 3]);
    }

    #[test]
    fn results_count_ok() {
        let a: Vec<Result<i32, ()>> = vec![Ok(1), Err(()), Ok(3)];
        assert_eq!(a.into_iter().count_ok(), 2);
    }

    #[test]
    fn results_count_err() {
        let a: Vec<Result<i32, ()>> = vec![Ok(1), Err(()), Ok(3)];
        assert_eq!(a.into_iter().count_err(), 1)
    }

    #[test]
    fn results_count_ok_err() {
        let a: Vec<Result<i32, ()>> = vec![Ok(1), Err(()), Ok(3)];
        assert_eq!(a.into_iter().count_ok_err(), (2, 1))
    }

    #[test]
    fn results_find_ok() {
        let a: Vec<Result<i32, ()>> = vec![Ok(1), Err(()), Ok(3)];
        let mut it = a.into_iter();
        assert_eq!(it.find_ok(), Some(1));
        assert_eq!(it.find_ok(), Some(3));
        assert_eq!(it.find_ok(), None);
    }

    #[test]
    fn results_find_err() {
        let a: Vec<Result<(), i32>> = vec![Err(1), Ok(()), Err(3)];
        let mut it = a.into_iter();
        assert_eq!(it.find_err(), Some(1));
        assert_eq!(it.find_err(), Some(3));
        assert_eq!(it.find_err(), None);
    }

    #[test]
    fn results_has_ok() {
        let a: Vec<Result<(), ()>> = vec![Err(()), Err(()), Ok(())];
        assert_eq!(a.into_iter().has_ok(), true);
        let a: Vec<Result<(), ()>> = vec![Err(()), Err(()), Err(())];
        assert_eq!(a.into_iter().has_ok(), false);
    }

    #[test]
    fn results_has_err() {
        let a: Vec<Result<(), ()>> = vec![Ok(()), Err(()), Ok(())];
        assert_eq!(a.into_iter().has_err(), true);
        let a: Vec<Result<(), ()>> = vec![Ok(()), Ok(()), Ok(())];
        assert_eq!(a.into_iter().has_err(), false);
    }

    #[test]
    fn results_ok_iter() {
        let a: Vec<Result<i32, i32>> = vec![Ok(1), Err(2), Ok(3), Err(4)];
        let v: Vec<_> = a.into_iter().ok_iter().collect();
        assert_eq!(vec![1i32, 3], v);
    }

    #[test]
    fn results_err_iter() {
        let a: Vec<Result<i32, i32>> = vec![Ok(1), Err(2), Ok(3), Err(4)];
        let v: Vec<_> = a.into_iter().err_iter().collect();
        assert_eq!(vec![2i32, 4], v);
    }
}
