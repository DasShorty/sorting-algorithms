package de.dasshorty;

import java.time.Instant;
import java.util.ArrayList;
import java.util.List;
import java.util.Random;

public class Main {
    public static void main(String[] args) {

        Main main = new Main();

        List<Integer> sortableObjects = main.generateRandomNumbers();

        System.out.println(sortableObjects);

        Instant startTime = Instant.now();

        List<Integer> sortedObjects = main.bubbleSort(sortableObjects);

        System.out.println(sortedObjects);

        long endTime = startTime.compareTo(Instant.now());
        System.out.println("Time elapsed: " + endTime + "ms");


    }

    private List<Integer> generateRandomNumbers() {

        Random random = new Random(System.currentTimeMillis());
        List<Integer> list = new ArrayList<>();

        for (int i = 0; i < 1000; i++) {

            int data = random.nextInt();
            System.out.println("Adding on position " + i + " new element with value " + data);
            list.add(data);

        }

        return list;
    }

    public List<Integer> bubbleSort(List<Integer> input) {

        int size = input.size();
        boolean swapped;

        do {

            swapped = false;

            for (int element = 0; element < size; element++) {

                if ((element + 1) == size) {
                    continue;
                }

                if (input.get(element) > input.get(element + 1)) {
                    this.swap(input, element);
                    swapped = true;
                }

            }

        } while (swapped);


        return input;

    }

    private void swap(List<Integer> input, int dataPosition) {

        int secondDataPosition = dataPosition + 1;

        int sortableObject = input.get(dataPosition);
        int secondSortableObject = input.get(secondDataPosition);

        System.out.println("Swapping " + sortableObject + " => " + secondSortableObject);

        input.set(secondDataPosition, sortableObject);
        input.set(dataPosition, secondSortableObject);

    }

}