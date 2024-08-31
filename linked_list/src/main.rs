use std::cell::RefCell;
use std::fmt::{Display, Formatter, Result};
use std::rc::Rc;

#[derive(Clone)]
struct Node<T: Clone> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}

struct DoublyLinkedList<T: Clone> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Clone> DoublyLinkedList<T> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn push_back(&mut self, data: T) {
        let new_node = Node {
            data: data,
            prev: None,
            next: None,
        };

        let new_node = Rc::new(RefCell::new(new_node));
        if self.head.is_none() {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
        } else {
            if let Some(tail) = &mut self.tail {
                tail.borrow_mut().next = Some(new_node.clone());
            }
            new_node.borrow_mut().prev = self.tail.clone();
            self.tail = Some(new_node);
        }
    }

    fn push_front(&mut self, data: T) -> T {
        unimplemented!()
    }

    fn pop_back(&mut self, data: T) -> T {
        unimplemented!()
    }

    fn pop_front(&mut self) -> T {
        unimplemented!()
    }
}

impl<T: Clone> Display for DoublyLinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut current = self.head.clone();
        write!(f, "head: (")?;
        while let Some(node) = current {
            let n = node.borrow();
            write!(f, "{}", n.data)?;
            write!(f, "@{:p}", &n.data)?;
            current = n.next.clone();
            if current.is_some() {
                write!(f, "<--->")?;
            }
        }
        write!(f, ") ")?;

        let mut current = self.tail.clone();
        write!(f, "tail: (")?;
        while let Some(node) = current {
            let n = node.borrow();
            write!(f, "{}", n.data)?;
            write!(f, "@{:p}", &n.data)?;
            current = n.prev.clone();
            if current.is_some() {
                write!(f, "<--->")?;
            }
        }
        write!(f, ")")?;

        Ok(())
    }
}

fn main() {
    let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
    println!("{}", list);

    list.push_back(1);
    println!("{}", list);

    list.push_back(2);
    println!("{}", list);

    list.push_back(3);
    println!("{}", list); // 1<--->2<--->3

    list.push_front(4);
    list.push_front(5);

    println!("{}", list); // 5<--->4<--->1<--->2<--->3
}
