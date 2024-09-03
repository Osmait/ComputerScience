from collections.abc import Iterable
from dataclasses import dataclass
from typing import Any


@dataclass
class Node:
    data: Any
    next_node: Node | None = None

    def __repr__(self) -> str:
        return f"Node({self.data})"


class LinkedList:
    def __init__(self) -> None:
        self.head = None

    def __iter__(self) -> Iterable[Any]:
        node = self.head
        while node:
            yield node.data
            node = node.next_node

    def __len__(self) -> int:
        return sum(i for _ in self)

    def __repr__(self) -> str:
        return "->".join([str(item) for item in self])

    def __getitem__(self, index: int) -> Any:
        if not 0 <= index < len(self):
            raise ValueError("list index out of range")
        for i, node in enumerate(self):
            if i == index:
                return node
        return None
