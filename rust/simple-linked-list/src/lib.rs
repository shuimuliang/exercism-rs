use std::iter::FromIterator;

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    data: T,
    next: Link<T>,
}

pub struct SimpleLinkedList<T> {
    head: Link<T>,
    len: usize,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {head: None, len: 0}
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, _element: T) {
        let new_node = Box::new(Node {
            data: _element,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.len -= 1;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|t| {
            &t.data
        })
    }

    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut li = SimpleLinkedList::new();
        while let Some(t) = self.pop() {
           li.push(t);
        }
        li
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut li = SimpleLinkedList::new();
        for i in _iter {
            li.push(i)
        }
        li
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

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut v: Vec<T> = Vec::new();

        while let Some(t) = self.pop() {
           v.insert(0, t);
        }
        v
    }
}
