use std::fmt::{Display, Formatter, Result};

#[derive(Clone)]
struct Node<T> {
    data: T,
    prev: Option<Box<Node<T>>>,
    next: Option<Box<Node<T>>>,
}

impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Node({})", self.data)
    }
}

struct DoublyLinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
}

impl<T: Clone> DoublyLinkedList<T> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn push_back(&mut self, data: T) {
        let mut new_node = Box::new(Node {
            data: data.clone(),
            prev: None,
            next: None,
        });
        if self.head.is_none() {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
        } else {
            new_node.prev = self.tail.clone();
            if let Some(tail) = &mut self.tail {
                tail.next = Some(new_node.clone());
            }
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
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut current = self.head.clone();
        write!(f, "(")?;
        while let Some(node) = current {
            write!(f, "{}", node.data)?;
            current = node.next.clone();
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
    list.push_back(3);

    println!("{}", list); // 1<--->2<--->3

    list.push_front(4);
    list.push_front(5);

    println!("{}", list); // 5<--->4<--->1<--->2<--->3
}
