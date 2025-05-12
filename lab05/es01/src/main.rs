#![allow(warnings)]

pub mod mem_inspect {
    use crate::{List1, List2};

    // dump object info:
    // size, address, bytes
    pub fn dump_object<T>(obj: &T) {
        let ptr = obj as *const T as *const u8;
        let _size = size_of::<T>();
        let _ptr = ptr as usize;
        println!("Object size: {_size}; address: {_ptr:x}");

        dump_memory(ptr, _size);
    }

    // dump memory info
    pub fn dump_memory(start: *const u8, size: usize) {
        let bytes = unsafe { std::slice::from_raw_parts(start, size) };

        println!("Bytes:");
        for (i, byte) in bytes.iter().enumerate() {
            print!("{:02x} ", byte);
            if i % 8 == 7 {
                println!();
            }
        }
        println!()
    }

    #[test]
    fn dump_object_example() {
        // let s = "hello".to_string();
        // dump_object(&s);
        //
        // let b = Box::new(s);
        // // before running try to answer:
        // // 1. what is the size of b?
        // // 2. what is the content of b?
        // dump_object(&b);
        //
        // // how to the the pointer of the wrapped object?
        // let ptr = b.as_ref() as *const String as *const u8;
        // println!("Pointer: {ptr:?}");

        let mut l1 = List1::List::<i32>::new();
        l1.push(10);
        let mut l2 = List2::List::<i32>::new();
        l2.push(10);

        dump_object(&l1);
        dump_object(&l2);

        assert!(true);
    }
}

pub mod List1 {
    use std::mem;

    pub enum Node<T> {
        Cons(T, Box<Node<T>>),
        Nil,
    }

    pub struct List<T> {
        head: Node<T>,
    }

    impl<T> List<T> {
        pub fn new() -> Self {
            List { head: Node::Nil }
        }

        // insert a new element at the beginning of the list
        // you may encouter a problem with the borrow checker while trying to move self.head to a new variable
        // problem:
        // 1. you need to build a new list Node (elem: elem, self.head)
        // 2. but you can't move self.head, because self.head would be undefined
        // 3. you can't copy it either, because Box can't be copied
        // solution: use mem::replace to move the value of self.head into a new variable and replace it with Nil
        // 4. let self.head point to the new created node
        pub fn push(&mut self, elem: T) {
            let new_node = Node::Cons(elem, Box::new(mem::replace(&mut self.head, Node::Nil)));
            self.head = new_node;
        }

        // pop the first element of the list and return it
        fn pop(&mut self) -> Option<T> {
            match mem::replace(&mut self.head, Node::Nil) {
                Node::Cons(elem, next) => {
                    self.head = *next;
                    Some(elem)
                }
                Node::Nil => None,
            }
        }

        // return a referece to the first element of the list
        pub fn peek(&self) -> Option<&T> {
            match &self.head {
                Node::Cons(elem, _) => Some(&elem),
                Node::Nil => None,
            }
        }

        //uncomment after having implemented the ListIter struct
        //return an interator over the list values
        fn iter(&self) -> ListIter<T> {
            ListIter { current: &self.head }
        }

        // take the first n elements of the list and return a new list with them
        pub fn take(&mut self, n: usize) -> List<T> {
            let mut newList = List::new();
            for x in 0..n {
                match self.pop() {
                    Some(elem) => {
                        newList.push(elem);
                    },
                    None => break
                }
            }
            newList
        }
    }

    struct ListIter<'a, T> {
        // implement the iterator trait for ListIter
        current: &'a Node<T>,
    }

    impl<'a, T> ListIter<'a, T> {
        pub fn new(list: &'a List<T>) -> Self {
            ListIter {
                current: &list.head
            }
        }
    }

    impl<'a, T> Iterator for ListIter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            match self.current {
                Node::Cons(elem, next) => {
                    self.current = next;
                    Some(elem)
                }
                Node::Nil => None,
            }
        }
    }

    // something that may be useful for the iterator implementation:
    // let a = Some(T);
    // let b = &a;
    // match b { Some(i) => ... } // here i is a reference to T
}

pub mod List2 {
    pub struct Node<T> {
        elem: T,
        next: NodeLink<T>,
    }

    type NodeLink<T> = Option<Box<Node<T>>>;

    pub struct List<T> {
        head: NodeLink<T>,
    }

    // for this implementattion, since we are using option, take a look at the take method in Option<T>.
    // It allows to move the value of the option into another option and replace it with None
    // let mut a = Some(5);
    // let b = a.take(); // a is now None and b is Some(5)
    impl<T> List<T> {
        pub fn new() -> Self {
            List { head: None }
        }

        pub fn push(&mut self, elem: T) {
            let new: NodeLink<T> = Some(Box::new(Node {
                elem,
                next: self.head.take(),
            }));
            self.head = new;
        }

        pub fn pop(&mut self) -> Option<T> {
            match self.head.take() {
                None => None,
                Some(nl) => {
                    self.head = nl.next;
                    Some(nl.elem)
                }
            }
        }

        pub fn peek(&mut self) -> Option<&T> {
            match &self.head {
                None => None,
                Some(nl) => Some(&nl.elem),
            }
        }

        pub fn take(&mut self, n: usize) -> List<T> {
            let mut newList = List::new();
            for x in 0..n {
                newList.push(self.pop().unwrap());
            }
            newList
        }
    }
}

// use Rc, since we need more than one reference to the same node.
// You need to both strong and weak references

// For mutating the list and changing the next and prev fields we also need to be able to mutate the node,
// therefore we can use RefCell too (as for the tree at lesson)

// how to access content of Rc<RefCell<T>>:
// es let a = Rc::new(RefCell::new(5));
// let mut x = (*a).borrow_mut();  // with (*a) we dereference the Rc, with (*a).borrow_mut() we get a mutable reference to the content of the RefCell
// *x = 6; // we can now change the content of the RefCell

// hint for pop: you can return either a reference to the value or take the value out of the Rc,
// but usually it is not possible to take out the value from an Rc since it may be referenced elsewhere.
// if you can guarantee it's the only reference to the value  you can use Rc::try_unwrap(a).unwrap().into_inner() to get the value
// it first takes out the value from the Rc, then it tries to unwrap the value from the Result, and finally it takes the inner value from the Result
// see here
// https://stackoverflow.com/questions/70404603/how-to-return-the-contents-of-an-rc
// otherwise you can impose the COPY trait on T

// other hint that may be useful: Option<T> has a default clone implementation which calls the clone of T. Therefore:
// Some(T).clone() ->  Some(T.clone())
// None.clone() -> None

pub mod dlist {
    // *****
    // double linked list suggestions:
    // the node has both a next and a prev link

    use std::cell::{Ref, RefCell};
    use std::rc::{Rc, Weak};

    type Node<T> = RefCell<DNode<T>>;
    type NodeLink<T> = Rc<Node<T>>;
    type NodeBackLink<T> = Weak<Node<T>>;

    struct DNode<T> {
        elem: T,
        prev: NodeBackLink<T>, // which type do we use here?
        next: Option<NodeLink<T>>,     // which type do we use here?
    }

    struct DList<T> {
        head: Option<NodeLink<T>>,
        tail: NodeBackLink<T>,
    }

    impl<T> DList<T> {
        fn new() -> Self {
            DList {
                head: None,
                tail: Weak::new(),
            }
        }

        fn push_front(&mut self, val: T) {
            match self.head.take() {
                Some(head) => {
                    let new_node = Rc::new(RefCell::new(DNode {
                        elem: val,
                        next: Some(head.clone()),
                        prev: Weak::new(),
                    }));
                    head.borrow_mut().prev = Rc::downgrade(&new_node);
                    self.head = Some(new_node);
                },
                None => {
                    let new_node = Rc::new(RefCell::new(DNode {
                        elem: val,
                        next: None,
                        prev: Weak::new(),
                    }));
                    self.tail = Rc::downgrade(&new_node);
                    self.head = Some(new_node);
                }
            }
        }

        fn push_back(&mut self, val: T) {
            match self.tail.upgrade() {
                Some(tail) => {
                    let new_node = Rc::new(RefCell::new(DNode {
                        elem: val,
                        next: None,
                        prev: Rc::downgrade(&tail),
                    }));
                    tail.borrow_mut().next = Some(new_node.clone());
                    self.tail = Rc::downgrade(&new_node);
                },
                None => {
                    let new_node = Rc::new(RefCell::new(DNode {
                        elem: val,
                        next: None,
                        prev: Weak::new(),
                    }));
                    self.head = Some(new_node.clone());
                    self.tail = Rc::downgrade(&new_node);
                }
            }
        }

        pub fn pop_front(&mut self) -> Option<T> {
            match self.head.take() {
                Some(head) => {
                    let next = head.borrow_mut().next.take();
                    if let Some(next) = next { //c'è un elemento dopo
                        next.borrow_mut().prev = Weak::new();
                        self.head = Some(next);
                    } else { //non c'è nulla dopo
                        self.head = None;
                        self.tail = Weak::new();
                    }
                    Rc::try_unwrap(head)
                        .map(|node| node.into_inner().elem)
                        .ok()
                },
                None => None
            }
        }

        pub fn pop_back(&mut self) -> Option<T> {
            match self.tail.upgrade() {
                Some(tail) => {
                    let prev = tail.borrow_mut().prev.upgrade();
                    if let Some(prev) = prev { //c'è ancora un elemento prima
                        prev.borrow_mut().next = None;
                        self.tail = Rc::downgrade(&prev);
                    } else { //non c'è nulla prima
                        self.head = None;
                        self.tail = Weak::new();
                    }
                    Rc::try_unwrap(tail)
                        .map(|node| node.into_inner().elem)
                        .ok()
                },
                None => None
            }
        }

        pub fn peek_front(&self) -> Option<Ref<T>> {
            self.head.as_ref()
                .map(|node| Ref::map(node.borrow(), |inner| &inner.elem))
        }

        pub fn popn(&mut self, n: usize) -> Option<T> {
            let mut cnode = self.head.clone();
            for _ in 0..n {
                if let Some(node) = cnode {
                    cnode = node.borrow_mut().next.clone();
                } else {
                    return None;
                }
            }
            match cnode {
                Some(node) => {
                    let prev = node.borrow_mut().prev.upgrade();
                    let next = node.borrow_mut().next.take();
                    match (prev, next) {
                        (Some(prev), Some(next)) => {
                            prev.borrow_mut().next = Some(next.clone());
                            next.borrow_mut().prev = Rc::downgrade(&prev);
                        },
                        (Some(prev), None) => {
                            prev.borrow_mut().next = None;
                            self.tail = Rc::downgrade(&prev);
                        },
                        (None, Some(next)) => {
                            next.borrow_mut().prev = Weak::new();
                            self.head = Some(next.clone());
                        },
                        (None, None) => {}
                    }
                    Rc::try_unwrap(node)
                        .map(|node| node.into_inner().elem)
                        .ok()
                },
                None => None
            }
        }
    }

    #[test]
    pub fn test_push_front() {
        let mut dlist = DList::new();
        dlist.push_front(1);
        assert!(dlist.head.as_ref().map(|n| n.borrow().elem) == Some(1));
        dlist.push_front(2);
        assert!(dlist.tail.upgrade().map(|n| n.borrow().elem) == Some(1));
        assert!(dlist.head.as_ref().map(|n| n.borrow().elem) == Some(2));
    }

    #[test]
    pub fn test_push_back() {
        let mut dlist = DList::new();
        dlist.push_back(1);
        assert!(dlist.head.as_ref().map(|n| n.borrow().elem) == Some(1));
        dlist.push_back(2);
        assert!(dlist.tail.upgrade().map(|n| n.borrow().elem) == Some(2));
        assert!(dlist.head.as_ref().map(|n| n.borrow().elem) == Some(1));
    }

    #[test]
    pub fn test_pop_front() {
        let mut dlist = DList::new();
        dlist.push_front(1);
        dlist.push_front(2);
        assert!(dlist.pop_front() == Some(2));
        assert!(dlist.head.as_ref().map(|n| n.borrow().elem) == Some(1));
        assert!(dlist.pop_front() == Some(1));
        assert!(dlist.pop_front() == None);
    }

    #[test]
    pub fn test_pop_back() {
        let mut dlist = DList::new();
        dlist.push_back(1);
        dlist.push_back(2);
        assert!(dlist.pop_back() == Some(2));
        assert!(dlist.tail.upgrade().map(|n| n.borrow().elem) == Some(1));
        assert!(dlist.pop_back() == Some(1));
        assert!(dlist.pop_back() == None);
    }

    #[test]
    pub fn test_peek_front() {
        let mut dlist = DList::new();
        dlist.push_front(1);
        dlist.push_front(2);
        let front = dlist.peek_front();
        assert!(front.map(|n| *n) == Some(2));
    }

    #[test]
    pub fn test_popn() {
        let mut dlist = DList::new();
        for i in 0..10 {
            println!("pushing {}", i);
            dlist.push_front(i);
        }

        assert!(dlist.popn(0) == Some(9));
        assert!(dlist.popn(1) == Some(7));
        assert!(dlist.popn(0) == Some(8));
        // now we have 6, 5, 4, 3, 2, 1, 0
        assert!(dlist.popn(6) == Some(0));
        assert!(dlist.popn(5) == Some(1));
    }
}

fn main() {}
