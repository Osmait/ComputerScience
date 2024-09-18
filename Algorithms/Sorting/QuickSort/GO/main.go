package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func Partition(arr []int, low, high int) int {
	index := low - 1
	pivotElement := arr[high]
	for i := low; i < high; i++ {
		if arr[i] <= pivotElement {
			index += 1
			arr[index], arr[i] = arr[i], arr[index]
		}
	}
	arr[index+1], arr[high] = arr[high], arr[index+1]
	return index + 1
}

func QuickSortRange(arr []int, low, high int) {
	if len(arr) <= 1 {
		return
	}
	if low < high {
		pivot := Partition(arr, low, high)
		QuickSortRange(arr, low, pivot-1)
		QuickSortRange(arr, pivot+1, high)
	}
}

func QuickSort(arr []int) []int {
	QuickSortRange(arr, 0, len(arr)-1)
	return arr
}

func main() {
	reader := bufio.NewReader(os.Stdin)
	fmt.Print("Enter ")
	input, _ := reader.ReadString('\n')
	input = strings.TrimSpace(input)
	list := strings.Split(input, ",")
	listOfNumber := make([]int, 0, len(list))
	for _, s := range list {
		v, err := strconv.Atoi(s)
		if err != nil {
			panic("please enter valid numbers ")
		}
		listOfNumber = append(listOfNumber, v)
	}
	fmt.Println(QuickSort(listOfNumber))
}
