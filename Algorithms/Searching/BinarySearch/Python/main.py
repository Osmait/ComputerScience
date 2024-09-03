def binary_search(list: list[int], item: int) -> int | None:
    low: int = 0
    high: int = len(list) - 1

    while low <= high:
        mid = (low + high) // 2
        guess = list[mid]
        if guess == item:
            return mid
        if guess > item:
            high = mid - 1
        else:
            low = mid + 1
    return None


if __name__ == "__main__":
    print(binary_search([1, 2, 3, 4], 3))
