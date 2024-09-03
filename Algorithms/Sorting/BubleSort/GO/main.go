package main

import "fmt"

func Bubble(arr []int) []int {
	swapped := true
	for swapped {
		swapped = false
		for i := 0; i < len(arr)-1; i++ {
			if arr[i+1] < arr[i] {
				arr[i+1], arr[i] = arr[i], arr[i+1]
				swapped = true
			}
		}
	}
	return arr
}

func main() {
	arr := []int{2, 1, 3, 5, 4}
	result := Bubble(arr)
	fmt.Println(result)
}
