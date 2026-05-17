from typing import List, Iterator


class Stack[T]:
    def __init__(self):
        self._items: List[T] = []

    def size(self) -> int:
        return len(self._items)

    def is_empty(self) -> bool:
        return len(self._items) == 0

    def push(self, item: T):
        self._items.append(item)

    def pop(self) -> T:
        if not self._items:
            raise IndexError("pop from empty stack")
        return self._items.pop()

    def peek(self) -> T:
        if not self._items:
            raise IndexError("peek from empty stack")
        return self._items[-1]

    def __iter__(self) -> Iterator[T]:
        for item in reversed(self._items):
            yield item
