#[derive(Debug, Clone)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            next: None,
        }
    }
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T: std::fmt::Debug + std::fmt::Display + Clone> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, _element: T) {

        let mut node = Node::<T>::new(_element);
        node.next = self.head.clone();
        let new_box = Box::<Node<T>>::new(node);
        self.head = Some(new_box);
        self.size += 1;
        println!("size after push {}", self.size);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let mut old_head = self.head.as_mut().unwrap().clone();
        let val = old_head.data.clone();
        let _next = old_head.next.as_mut();
        self.head = None;
        self.size -= 1;

        print!("val popped {val}");
        println!("size after pop {}", self.size);
        if _next.is_none() {
            return Some(val);
        }

        self.head = Some(Box::<Node<T>>::new(*_next.unwrap().clone()));
        Some(val)
    }

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        
        let _head = self.head.as_ref().unwrap();

        Some(&_head.data)
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut _linked_list = SimpleLinkedList::<T>::new();
        if self.is_empty() {
            return _linked_list
        }

        
        let mut node = self.head.as_ref().unwrap();
        loop {
            let val = node.data.clone();

            _linked_list.push(val);

            if node.next.is_none() {
                break;
            }
            
            node = node.next.as_ref().unwrap();
        }

        _linked_list
    }
}

impl<T: std::fmt::Debug + std::fmt::Display + Clone> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut linked_list = SimpleLinkedList::<T>::new();
        for x in _iter {
            linked_list.push(x);
        }

        linked_list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.
//
// Please note that the "front" of the linked list should correspond to the "back"
// of the vector as far as the tests are concerned.

impl<T: std::fmt::Debug + std::fmt::Display + Clone> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        if _linked_list.is_empty() {
            return Vec::<T>::new();
        }
        
        let mut output: Vec<T> = Vec::<T>::with_capacity(_linked_list.size);
        let mut node = _linked_list.head.as_ref().unwrap();
        loop {
            let val = node.data.clone();
            output.insert(0, val);

            if node.next.is_none() {
                break;
            }
            
            node = node.next.as_ref().unwrap();
        }

        output
    }
}
