from typing import Any


def bubble_sort_iterative(collection: list[Any]) -> list[Any]:
    length = len(collection)
    for i in reversed(range(length)):
        swapped = False
        for j in range(i):
            if collection[j] > collection[j + 1]:
                swapped = True
                collection[j], collection[j + 1] = collection[j + 1], collection[j]
        if not swapped:
            break
    return collection


if __name__ == "__main__":
    result = bubble_sort_iterative([2, 1, 4, 3, 5])
    print(result)
