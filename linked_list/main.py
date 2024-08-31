class Node:
    "TODO"


class DoublyLinkedList:
    "TODO"


if __name__ == "__main__":
    l = DoublyLinkedList()

    l.push_back(1)
    l.push_back(2)
    l.push_back(3)
    print(l)  # 1<--->2<--->3

    l.push_back(4)
    l.push_back(5)
    print(l)  # 5<--->4<--->1<--->2<--->3
