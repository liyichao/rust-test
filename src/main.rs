pub struct LinkedList<T> {
    length: usize,
    list_head: Link<T>,
    list_tail: Rawlink<Node<T>>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Rawlink<T> {
    p: *mut T,
}

struct Node<T> {
    next: Link<T>,
    prev: Rawlink<Node<T>>,
    value: T,
}

pub struct Iter<'a, T:'a> {
    head: &'a Link<T>,
    tail: Rawlink<Node<T>>,
    nelem: usize,
}

struct a {
    b: i32
}
fn main() {

    let t = &a{b: 2};
    println!("{}", t.b);
}
