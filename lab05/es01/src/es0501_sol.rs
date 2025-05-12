#![allow(warnings)]

pub mod mem_inspect {
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
        let s = "hello".to_string();
        dump_object(&s);

        let b = Box::new(s);
        // before running try to answer:
        // 1. what is the size of b?
        // 2. what is the content of b?
        dump_object(&b);

        // how to the the pointer of the wrapped object?
        let ptr = b.as_ref() as *const String as *const u8;
        println!("Pointer: {ptr:?}");

        assert!(true);
    }
}


pub mod List1 {
    pub enum Node<T> {
        Cons(T, Box<Node<T>>),
        Nil,
    }

    pub struct List<T> {
        head: Node<T>,
    }

    impl<T> List<T> {
        pub fn new() -> Self {
            List {
                head: Node::Nil,
            }
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
            let head = std::mem::replace(&mut self.head, Node::Nil);
            self.head = Node::Cons(elem, Box::new(head));
        }

        // pop the first element of the list and return it
        fn pop(&mut self) -> Option<T> {
            let head = std::mem::replace(&mut self.head, Node::Nil);
            match head {
                Node::Cons(elem, next) => {
                    self.head = *next; // move the next node into self.head
                    Some(elem)
                }
                Node::Nil => None
            }
        }

        // return a referece to the first element of the list
        pub fn peek(&self) -> Option<&T> {
            match &self.head {
                Node::Cons(elem, _) => Some(elem),
                Node::Nil => None
            }
        }

        // uncomment after having implemented the ListIter struct
        // return an interator over the list values
        fn iter(&self) -> ListIter<T> {
            ListIter::new(&self)
        }

        // take the first n elements of the list and return a new list with them
        pub fn take(&mut self, n: usize) -> List<T> {
            let mut new_list = List::new();
            let mut count = 0;
            while count < n {
                match self.pop() {
                    Some(elem) => {
                        new_list.push(elem);
                        count += 1;
                    }
                    None => break
                }
            }
            new_list
        }
    }

    struct ListIter<'a, T> {
        current: &'a Node<T>,
    }

    impl<'a, T> ListIter<'a, T> {
        pub fn new(list: &'a List<T>) -> Self {
            ListIter {
                current: &list.head,
            }
        }
    }

    impl<'a, T> Iterator for ListIter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            match self.current {
                Node::Cons(elem, next) => {
                    // Here next is &Box<Node<T>>, Rust will dereference it for us
                    // when trying to assign it to self.current which is of type &Node<T>
                    // It's possible since we specified that &Node<T> should have the same lifetime as self
                    // therefore we are sure that the Box containing it will not be dropped as long
                    // we are using the iterator
                    self.current = next;
                    Some(elem)
                }
                Node::Nil => None
            }
        }
    }

    #[test]
    fn test_list_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
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

    // for this implementation, since we are using Option, take a look at the take method in Option<T>.
    // It allows to move the value of the option into another option and replace it with None
    // let mut a = Some(5);
    // let b = a.take(); // a is now None and b is Some(5)
    impl<T> List<T> {
        pub fn new() -> Self {
            List {
                head: None,
            }
        }

        pub fn push(&mut self, elem: T) {
            let new_node = Some(Box::new(Node {
                elem,
                next: self.head.take(),
            }));
            self.head = new_node;
        }

        pub fn pop(&mut self) -> Option<T> {

            // take() extracts the value from the Option and replaces it with None
            // therefore when we call map() we don't have to worry about the borrow checker
            // for self.head, map is owning already the extracted value from head
            self.head.take().map(|node| {
                self.head = node.next; // move the next node into self.head
                node.elem
            })
        }

        pub fn peek(&self) -> Option<&T> {
            match &self.head {
                Some(node) => Some(&node.elem),
                None => None
            }
        }

        // take() is trivial as for List1
    }
}

pub mod dlist {
    // *****
    // double linked list suggestions:
    // the node has both a next and a prev link

    // type NodeLink = ???
    // typer NodeBackLink = ???
    // struct DNode<T> {
    //     elem: T,
    //     prev: NodeBackLink,  // which type do we use here?
    //     next: NodeLink, // which type do we use here?
    // }

    // struct DList {
    // head: NodeLink,
    // tail: NodeLink
    // }

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

    use std::{cell::{Ref, RefCell}, rc::{Rc, Weak}};

    // A node that can be modified with interior mutability
    type Node<T> = RefCell<InnerNode<T>>;
    type NodeLink<T> = Rc<Node<T>>;
    type WeakNodeLink<T> = Weak<Node<T>>;

    #[derive(Debug)]
    struct InnerNode<T> {
        elem: T,
        next: Option<NodeLink<T>>,
        prev: WeakNodeLink<T>,
    }

    pub struct DList<T> {
        head: Option<NodeLink<T>>,
        tail: WeakNodeLink<T>,
    }

    impl<T> DList<T> {
        pub fn new() -> Self {
            DList {
                head: None,
                tail: Weak::new(),
            }
        }

        pub fn push_front(&mut self, elem: T) {
            match self.head.take() {
                Some(head) => {
                    let new_node = Rc::new(RefCell::new(InnerNode {
                        elem,
                        next: Some(head.clone()),
                        prev: Weak::new(),
                    }));
                    head.borrow_mut().prev = Rc::downgrade(&new_node);
                    self.head = Some(new_node);
                }
                None => {
                    let new_node = Rc::new(RefCell::new(InnerNode {
                        elem,
                        next: None,
                        prev: Weak::new(),
                    }));
                    self.tail = Rc::downgrade(&new_node);
                    self.head = Some(new_node);
                }
            }
        }

        pub fn push_back(&mut self, elem: T) {
            match self.tail.upgrade() {
                Some(tail) => {
                    let new_node = Rc::new(RefCell::new(InnerNode {
                        elem,
                        next: None,
                        prev: Rc::downgrade(&tail),
                    }));
                    tail.borrow_mut().next = Some(new_node.clone());
                    self.tail = Rc::downgrade(&new_node);
                }
                None => {
                    let new_node = Rc::new(RefCell::new(InnerNode {
                        elem,
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
                    if let Some(next) = next {
                        next.borrow_mut().prev = Weak::new();
                        self.head = Some(next);
                    } else {
                        self.head = None;
                        self.tail = Weak::new();
                    }
                    Rc::try_unwrap(head) // extact the value from the Rc
                        .map(|node| node.into_inner().elem) // extract the value from the RefCell
                        .ok() // transform the Result into an Option
                }
                None => None
            }
        }

        pub fn pop_back(&mut self) -> Option<T> {
            match self.tail.upgrade() {
                Some(tail) => {
                    let prev = tail.borrow_mut().prev.upgrade();
                    if let Some(prev) = prev {
                        prev.borrow_mut().next = None;
                        self.tail = Rc::downgrade(&prev);
                    } else {
                        self.head = None;
                        self.tail = Weak::new();
                    }
                    Rc::try_unwrap(tail) // extact the value from the Rc
                        .map(|node| node.into_inner().elem) // extract the value from the RefCell
                        .ok() // transform the Result into an Option
                }
                None => None
            }
        }

        // With Rc<RefCell<T>> we can't have a direct ref to T, but only a ref like object Ref<T>
        pub fn peek_front(&self) -> Option<Ref<T>> {

            // we need to borrow the head node and then borrow the inner node
            // with borrow() we get a Ref<InnerNode<T>>, not yet to Ref<T>  
            // since this is a common problem, Ref has Ref::map which allow to extract Ref from the content of anothere Ref          
            self.head.as_ref()
                .map(|node| Ref::map(node.borrow(), |inner| &inner.elem))
        }


        // we cannot peek the back node, since we have a weak reference to it
        // we could return a Rc of the full node, but we can't return a Ref<T> 
        // Why? because the Ref<t> is linked to the lifetime of the Rc, and the Rc would be dropped after the function call
        // pub fn peek_back(&self) -> Option<Ref<T>> {
        //     self.tail.upgrade().as_ref().map(|node| Ref::map(node.borrow(), |inner| &inner.elem))
        // }

        // pop the nth node from the list
        pub fn popn(&mut self, n: usize) -> Option<T> {
            // clone() on Option<Rc<T>> with call clone() on the inner Rc<T> and return a new Option<Rc<T>>
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

                    // here we use match pattern matching to test all 4 possible cases with prev/next
                    match (prev, next) {
                        (Some(prev), Some(next)) => {
                            prev.borrow_mut().next = Some(next.clone());
                            next.borrow_mut().prev = Rc::downgrade(&prev);
                        }
                        (Some(prev), None) => {
                            prev.borrow_mut().next = None;
                            self.tail = Rc::downgrade(&prev);
                        }
                        (None, Some(next)) => {
                            self.head = Some(next.clone());
                            next.borrow_mut().prev = Weak::new();
                        }
                        (None, None) => {}
                    }

                    Rc::try_unwrap(node) // extact the value from the Rc
                        .map(|node| node.into_inner().elem) // extract the value from the RefCell
                        .ok() // transform the Result into an Option
                }
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