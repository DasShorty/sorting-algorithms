package de.dasshorty;

import java.util.ArrayList;
import java.util.List;
import java.util.Random;

public class Main {
    public static void main(String[] args) {

        Main main = new Main();

        List<SortableObject> sortableObjects = main.generateRandomNumbers();

        System.out.println(sortableObjects);

        long startTime = System.currentTimeMillis();

        List<SortableObject> sortedObjects = main.bubbleSort(sortableObjects);

        System.out.println(sortedObjects);

        long endTime = System.currentTimeMillis();
        System.out.println((endTime - startTime) + "ms sort time");


    }

    private List<SortableObject> generateRandomNumbers() {

        Random random = new Random(System.currentTimeMillis());
        List<SortableObject> list = new ArrayList<>();

        for (int i = 0; i < 1000; i++) {


            int data = random.nextInt();
            System.out.println("Adding on position " + i + " new element with value " + data);
            list.add(new SortableObject(data, data));

        }


        return list;
    }

    public List<SortableObject> bubbleSort(List<SortableObject> input) {

        int size = input.size();
        boolean swapped;

        do {

            swapped = false;

            for (int element = 0; element < size; element++) {

                if ((element + 1) == size) {
                    continue;
                }

                if (input.get(element).weight() > input.get(element + 1).weight()) {
                    this.swap(input, element);
                    swapped = true;
                }

            }

        } while (swapped);


        return input;

    }

    private void swap(List<SortableObject> input, int dataPosition) {

        int secondDataPosition = dataPosition + 1;

        SortableObject sortableObject = input.get(dataPosition);
        SortableObject secondSortableObject = input.get(secondDataPosition);

//        System.out.println("Swapping " + sortableObject.data() + " => " + secondSortableObject.data());

        input.set(secondDataPosition, sortableObject);
        input.set(dataPosition, secondSortableObject);

    }

}