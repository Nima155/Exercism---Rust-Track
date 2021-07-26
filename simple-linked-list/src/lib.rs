use std::{iter::FromIterator};

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>
}

pub struct SimpleLinkedList<'a,T> {
    // Delete this field
    // dummy is needed to avoid unused parNameter error during compilation
    head: Option<Box<Node<T>>>,
    tail: &'a mut Option<Box<Node<T>>>
    
}

impl<'a,T> SimpleLinkedList<'a,T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: head
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        if let None = self.head {
            return true;
        }
        false
    }

    pub fn len(&self) -> usize {
        let mut head = &self.head;

        let mut ans = match &self.head {
            Some(_) => 1,
            None => 0
        };

        while let Some(t) = head {
            ans += 1;
            let x = &t.next;
            head = x;
        }
        
        return ans;
    }

    pub fn push(&mut self, value: T) {
        if let Some(s) = self.tail {
            s.next = Some(Box::new(Node {value, next: None}));
        } else {
            self.head = Some(Box::new(Node {value, next: None}));
        }
         
        
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut head = &mut self.head;
        while let Some(s) = head {
            if let Some(z) = &mut s.next {
                if let None = z.next {
                    s.next = None;
                    return Some(z.value);
                }
            }
            
        }
        None
    }

    pub fn peek(&self) -> Option<&T> {
        Some(&self.tail.as_ref().unwrap().value)
    }

    // pub fn rev(mut self) -> SimpleLinkedList<'a,T> {
    //     let save: Option<Box<Node<T>>>;
    //     while let Some(mut x) =  self.head {
    //         if let Some(mut z) = x.next {
    //             if let Some(mut y) = z.next {
    //                 save = z.next;
    //                 z.next = Some(x);

    //             }
    //         }
    //     }
    //     self
    // }
}

impl<'a,T> FromIterator<T> for SimpleLinkedList<'a,T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        unimplemented!()
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

impl<'a,T> Into<Vec<T>> for SimpleLinkedList<'a,T> {
    fn into(self) -> Vec<T> {
        unimplemented!()
    }
}
