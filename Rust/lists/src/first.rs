use std::{
    mem::{self, replace},
};

struct Node {
    elem: i32,
    next: Link,
}
enum Link {
    Empty,
    More(Box<Node>),
}

pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(node);
    }

    // pub fn alt_push(self, elem: i32) -> Self {
    //     List {
    //         head: Link::More(Box::new(Node {
    //             elem,
    //             next: self.head,
    //         })),
    //     }
    // }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }

    // pub fn pop_node(&mut self) -> Link {
    //     match self.head {
    //         Link::Empty => Link::Empty,
    //         Link::More(node) => {
    //             self.head = node.next;
    //             Link::Empty
    //         }
    //     }
    // }
}
// impl Drop for List {
//     fn drop(&mut self) {
//         self.head.drop();
//     }
// }

// impl Drop for Link{
//     fn drop(&mut self) {
//         match self {
//            Link::Empty => {},
//            Link::More(ref mut boxed_node) => {
//             boxed_node.drop();
//            }
//         }
//     }
// }

// impl Drop for Box<Node> {
//     fn drop(&mut self) {
//         self.as_mut().drop();
//         deallocate(self.ptr());
//     }
// }

// impl Drop for Node {
//     fn drop(&mut self) {
//        self.next.drop();
//     }
// }
impl Drop for List {
    fn drop(&mut self) {
        let mut curr_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = curr_link {
            curr_link = replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
