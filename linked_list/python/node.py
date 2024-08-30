from typing import Generics, Optional, TypeVar

T = TypeVar("T")


class Node(Generics[T]):
    def __init__(self, value: T, next_: Optional["Node"] = None):
        self.value = value
        if next_ and type(value) is not type(next_.value):
            raise ValueError(f"type of {self} and {next_} are different.")

        self.next = next_

    def __bool__(self):
        return self is not None
    
    def __str__(self):
        return f"Node({self.value})"

    def __repr__(self):
        return self.__str__()
