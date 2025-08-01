/// Yields each item of a and then each item of b
pub fn append<I, J>(mut _a: I, mut _b: J) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    std::iter::from_fn(move || {

            if let Some(item) = _a.next() {
                return Some(item);
            } 
            else if let Some(item) = _b.next() {
                return Some(item);
            }
            
            None
    })
}

/// Combines all items in all nested iterators inside into one flattened iterator
pub fn concat<I>(mut _nested_iter: I) -> impl Iterator<Item = <I::Item as Iterator>::Item>
where
    I: Iterator,
    I::Item: Iterator,
{
    // _nested_iter.flatten()
    let mut x: Vec<_> = Vec::new();
    while let Some(mut _iter) = _nested_iter.next() {
        while let Some(item) = _iter.next() {
            x.push(item);
        }
    }

    x.into_iter()
        
}

/// Returns an iterator of all items in iter for which `predicate(item)` is true
pub fn filter<I, F>(mut _iter: I, _predicate: F) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    // _iter.filter(_predicate)
    let mut x: Vec<_> = Vec::new();
    while let Some(item) = _iter.next() {
        if _predicate(&item) {
            x.push(item)
        }
    }

    x.into_iter()
}

pub fn length<I: Iterator>(mut _iter: I) -> usize {
    let mut count: usize = 0;
    while let Some(_val) = _iter.next() {
        count += 1;
    }
    count
}

/// Returns an iterator of the results of applying `function(item)` on all iter items
pub fn map<I, F, U>(mut _iter: I, _function: F) -> impl Iterator<Item = U>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    std::iter::from_fn(move || {
        if let Some(item) = _iter.next() {
            return Some(_function(item));
        }
        None
    })
}

pub fn foldl<I, F, U>(mut _iter: I, _initial: U, _function: F) -> U
where
    I: Iterator,
    F: Fn(U, I::Item) -> U,
{
    let mut output: U = _initial;
    while let Some(item) = _iter.next() {
        output = _function(output, item);
    }

    output
}

pub fn foldr<I, F, U>(mut _iter: I, _initial: U, _function: F) -> U
where
    I: DoubleEndedIterator,
    F: Fn(U, I::Item) -> U,
{
    let mut output: U = _initial;
    while let Some(item) = _iter.next_back() {
        output = _function(output, item);
    }

    output

}

/// Returns an iterator with all the original items, but in reverse order
pub fn reverse<I: DoubleEndedIterator>(mut _iter: I) -> impl Iterator<Item = I::Item> {
    std::iter::from_fn( move || {
        _iter.next_back()
    })
}
