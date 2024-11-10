#include "stdio.h"

int binarySearch(int *arr, int target, int size) {
  int low = 0;
  int high = size - 1;
  for (int i = 0; i <= high; i++) {
    int mid = (low + high) / 2;
    if (arr[mid] == target) {
      return mid;
    }
    if (arr[mid] < target) {
      low = mid + 1;
    }
    if (arr[mid] > target) {
      high = mid - 1;
    }
  }
  return -1;
}

int main(int argc, char *argv[]) {

  int arr[5] = {1, 2, 3, 4, 5};

  int result = binarySearch(arr, 4, 5);
  printf("result = %d\n", result);
}
