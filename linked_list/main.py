from .python import LinkedList, Node


linked_list = LinkedList(
    head=Node(
        1, next=Node(
            2, next=Node(
                3, next=Node(4)
            )
        )
    )
)
