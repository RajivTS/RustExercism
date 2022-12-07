use std::iter::FromIterator;

type Link<T> = Option<Box<Node<T>>>;

pub struct SimpleLinkedList<T> {
    head: Link<T>,
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Node<T> {    
    fn new(elem: T, next: Link<T>) -> Self {
        Node {
            elem,
            next,
        }
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        match &self.head {
            Some(head) => {
                let mut count = 1;
                let mut curr = head;
                while let Some(n) = &curr.next {
                    curr = n;
                    count += 1
                }
                count
            },
            None => 0,
        }
        
    }

    pub fn push(&mut self, element: T) {
        let new_head = Box::new(Node::new(element, self.head.take()));
        self.head = Some(new_head);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|prev_head| {
            self.head = prev_head.next;
            prev_head.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut list = Self::new();
        let mut curr = self.head;
        while curr.is_some() {
            curr = curr.take().and_then(|node| {
                list.push(node.elem);
                node.next
            });
        }
        list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        for value in iter {
            list.push(value);
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut list = Vec::new();
        while let Some(element) = linked_list.pop() {
            list.push(element);
        }
        list.reverse();
        list
    }
}
