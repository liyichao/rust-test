#![allow(dead_code)]
use std::ptr;

pub struct LinkedList<T> {
    length: usize,
    list_head: Link<T>,
    list_tail: Rawlink<Node<T>>,
}

type Link<T> = Option<Box<Node<T>>>;
struct Rawlink<T> {
    p: *mut T,
}

impl<T> Copy for Rawlink<T> {}
impl<T> Clone for Rawlink<T> {
    #[inline]
    fn clone(&self) -> Rawlink<T> {
        Rawlink{p: self.p}
    }
}
struct Node<T> {
    next: Link<T>,
    prev: Rawlink<Node<T>>,
    value: T,
}

pub struct Iter<'a, T: 'a> {
    head: &'a Link<T>,
    tail: Rawlink<Node<T>>,
    nelem: usize,
}

fn main() {

    let a: i32 = 1;
    let rawlink = Rawlink{p: ptr::null_mut()};
    let node = Node{
        next: None,
        prev: rawlink,
        value: 2,
    };
    let it: Iter<i32>;
    {
        let link = Some(Box::new(node));
        it = Iter {
            head: &link,
            tail: rawlink,
            nelem: 1,
        };
    }
}
