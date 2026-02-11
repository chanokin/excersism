use std::collections::BTreeSet;

#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T> {
    data: BTreeSet<T>,    
}

impl<T: ::core::clone::Clone + std::cmp::Ord> CustomSet<T> {
    pub fn new(_input: &[T]) -> Self {
        let mut _set = Self {
            data: BTreeSet::new(),
        };

        for _x in _input {
            _set.add(_x.clone());
        }
            
        _set
    }

    pub fn contains(&self, _element: &T) -> bool {
        self.data.contains(_element)
    }

    pub fn add(&mut self, _element: T) {
        self.data.insert(_element);
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        if self.is_empty() {
            return true;
        }

        for element in &self.data {
            if ! _other.contains(element) {
                return false;
            }
        }
        
        true
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        if self.is_empty() || _other.is_empty() {
            return true;
        }

        for x in &self.data {
            if _other.contains(x) {
                return false;
            }
        }

        true
    }

    #[must_use]
    pub fn intersection(&self, _other: &Self) -> Self {
        let mut result = CustomSet::new(&[]);
        if self.is_empty() || _other.is_empty() {
            return result;
        }

        for x in &self.data {
            if _other.contains(x) {
                result.add(x.clone());
            }
        }

        result
    }

    #[must_use]
    pub fn difference(&self, _other: &Self) -> Self {
        let mut result = CustomSet::new(&[]);
        if self.is_empty() {
            return result;
        }

        if _other.is_empty() {
            result.data = self.data.clone();
            return result;
        }

        for first in &self.data {
            if ! _other.contains(first) {
                result.add(first.clone());
            }
        }

        result
    }

    #[must_use]
    pub fn union(&self, _other: &Self) -> Self {
        let mut result = CustomSet::new(&[]);

        for x in &self.data {
            result.add(x.clone());
        }

        for x in &_other.data {
            result.add(x.clone());
        }

        result
    }
}
