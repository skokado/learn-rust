#[derive(Clone)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

type OptionalNode<T> = Option<Box<Node<T>>>;


struct LinkedList<T> {
    head: OptionalNode<T>,
}

impl<T> LinkedList<T> {
    fn new(head: OptionalNode<T>) -> Self {
        LinkedList{ head: head }
    }

    fn get_head(&self) -> &OptionalNode<T> {
        &self.head
    }

    fn get_tail(&self) -> &OptionalNode<T> {
        let mut node = &self.head;
        loop {
            if node.next.is_none() { break; }
            node = node.next;
        }
        node
    }

    fn lpush(&self) { unimplemented!() }
    fn rpush(&self) { unimplemented!() }
    fn delete(&self) { unimplemented!() }
    fn find(&self) { unimplemented!() }
    fn print(&self) { unimplemented!() }
}



fn main() {
    let node = Node{ value: 1 , next: None };

    let ll = LinkedList::new(node);
    ll.hello();
}
