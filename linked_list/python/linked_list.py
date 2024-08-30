from typing import Optional, TypeVar

from .node import Node

T = TypeVar("T")


class LinkedList:
    def __init__(self, head: Node[T]):
        self.head = head

    def get_head(self) -> Node[T]:
        return self.head
    
    def get_tail(self) -> Node[T]:
        node = self.head
        while node.next is not None:
            node = node.next
        return node
    
    def lpush(self, value: T):
        """Insert a Node at beginning"""
        new_head = Node(value, next_=self.head)
        self.head = new_head
    
    def rpush(self, value: T):
        """Insert a Node at end"""
        current_tail: Node[T] = self.get_tail()
        new_tail = Node(value)
        current_tail.next = new_tail
    
    def delete(self, value: T):
        """Delete a Node that has specified value"""
        if self.head.value == value:
            if self.head.next is None:
                raise ValueError("Cannot delete Node because Length is 1.")

            old_head = self.head
            self.head = self.head.next
            del old_head
            return

        node = self.head
        while node.next is not None:
            if node.next.value == value:
                old_node = node.next
                node.next = old_node.next
                del old_node
                return

            node = node.next

        print(f"# Node({value}) was not found. Nothing to do.")
    
    def find(self, value: T) -> Optional[Node[T]]:
        """Returns a Node that has specified value"""
        node = self.head
        while node.next is not None:
            if node.value == value:
                return node
            node = node.next
        return None
     
    def print(self):
        if self.head is None:
            print("()")
            return

        node = self.head
        print(f"({node.value})", end="")

        while node.next is not None:
            node = node.next
            print(f"-->({node.value})", end="")
        
        print()
