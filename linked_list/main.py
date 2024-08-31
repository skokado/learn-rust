from typing import Optional, TypeVar

T = TypeVar("T")


class Node:
    def __init__(self, data: T, prev: Optional["Node"] = None, next: Optional["Node"] = None):
        self.data = data
        self.prev = prev
        self.next = next

    def __str__(self):
        return f"Node({self.data})"

    def __repr__(self):
        return self.__str__()


class DoublyLinkedList:
    def __init__(self):
        self.head = None
        self.tail = None

    def __str__(self):
        if self.is_empty():
            return
        
        node = self.head
        res = str(node)
        while node.next is not None:
            node = node.next
            res += f"<--->{node}"

        return res

    def __repr__(self) -> str:
        return self.__str__()

    def is_empty(self) -> bool:
        return self.head is None
    
    def push_back(self, data: T):
        new_node = Node(data)
        if self.is_empty():
            self.head = new_node
            self.tail = new_node
            return
        
        new_node.prev = self.head
        self.tail.next = new_node
        self.tail = new_node

    def push_front(self, data: T):
        new_node = Node(data)
        if self.is_empty():
            self.head = new_node
            self.tail = new_node
            return

        new_node.next = self.head
        self.head.prev = new_node
        self.head = new_node

if __name__ == "__main__":
    l = DoublyLinkedList()

    l.push_back(1)
    l.push_back(2)
    l.push_back(3)
    print(l)  # 1<--->2<--->3

    l.push_front(4)
    l.push_front(5)
    print(l)  # 5<--->4<--->1<--->2<--->3
