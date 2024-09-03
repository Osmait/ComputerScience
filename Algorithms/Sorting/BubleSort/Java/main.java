public class Main {
    public static void main(String[] args) {
        System.out.println("hello");
        Integer[] arr = {1, 3, 2, 5, 4}; // Cambié el tipo a Integer
        sort(arr); // Llamada al método estático
        for (int i : arr) {
            System.out.print(i + " ");
        }
    }

    // Cambié a static
    public static <T extends Comparable<T>> T[] sort(T[] array) {
        for (int i = 1, size = array.length; i < size; ++i) {
            boolean swapped = false;
            for (int j = 0; j < size - i; ++j) {
                if (greater(array[j], array[j + 1])) {
                    swap(array, j, j + 1);
                    swapped = true;
                }
            }
            if (!swapped) {
                break;
            }
        }
        return array;
    }

    public static <T> void swap(T[] array, int i, int j) {
        if (i != j) {
            final T temp = array[i];
            array[i] = array[j];
            array[j] = temp;
        }
    }

    public static <T extends Comparable<T>> boolean greater(T firstElement, T secondElement) {
        return firstElement.compareTo(secondElement) > 0;
    }
}
