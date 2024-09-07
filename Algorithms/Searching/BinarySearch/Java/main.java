
public class main {

  public static void main(String[] args) {

    int[] arr = { 1, 2, 3, 4 };
    int result = binary_search(arr, 2);
    System.out.println(result);
  }

  public static int binary_search(int[] list, int item) {
    int low = 0;
    int high = list.length - 1;
    while (low <= high) {
      int mid = (low + high) / 2;
      int guess = list[mid];
      if (guess == item) {
        return mid;
      }
      if (guess > item) {
        high = mid - 1;
      } else {
        low = mid + 1;
      }

    }
    return -1;
  }
}
