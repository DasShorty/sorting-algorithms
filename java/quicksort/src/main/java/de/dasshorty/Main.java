package de.dasshorty;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Random;

public class Main {

    public static void main(String[] args) {
        List<Integer> integers = randomData();

        int[] array = integers.stream().mapToInt(Integer::intValue).toArray();

        System.out.println("Unsorted list: " + Arrays.toString(array));

        quickSort(array, 0, integers.size() - 1);

        System.out.println("Sorted list:" + Arrays.toString(array));
    }

    private static List<Integer> randomData() {

        List<Integer> list = new ArrayList<>();

        Random random = new Random(System.currentTimeMillis());

        for (int i = 0; i < 1000; i++) {

            list.add(random.nextInt());

        }

        return list;
    }

    public static void quickSort(int[] array, int left, int right) {
        if (left < right) {
            int pivotIndex = partition(array, left, right);
            quickSort(array, left, pivotIndex - 1);
            quickSort(array, pivotIndex + 1, right);
        }
    }

    private static int partition(int[] array, int left, int right) {
        int pivot = array[right];
        int i = left - 1;
        for (int j = left; j < right; j++) {
            if (array[j] <= pivot) {
                i++;
                swap(array, i, j);
            }
        }
        swap(array, i + 1, right);
        return i + 1;
    }

    private static void swap(int[] array, int i, int j) {
        int temp = array[i];
        array[i] = array[j];
        array[j] = temp;
    }
}