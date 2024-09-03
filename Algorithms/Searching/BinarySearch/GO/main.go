package main

import "fmt"

func BinarySearch(arr []int, target int) int {
	low := 0
	high := len(arr) - 1

	for low <= high {
		mid := (low + high) / 2
		if arr[mid] == target {
			return mid
		}
		if arr[mid] < target {
			low = mid + 1
		}
		if arr[mid] > target {
			high = mid - 1
		}
	}
	return -1
}

func main() {
	result := BinarySearch([]int{1, 2, 3, 4, 5}, 3)
	fmt.Println(result)
}
