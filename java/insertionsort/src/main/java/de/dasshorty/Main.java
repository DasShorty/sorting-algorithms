package de.dasshorty;

import java.time.Instant;
import java.util.ArrayList;
import java.util.List;
import java.util.Random;

public class Main {
    public static void main(String[] args) {

        Main main = new Main();

        List<Integer> integers = main.generateData();

        System.out.println(integers);

        Instant startTime = Instant.now();

        List<Integer> sortedList = main.insertionSort(integers);
        System.out.println(sortedList);

        long endTime = startTime.compareTo(Instant.now());

        System.out.println("Time elapsed: " + endTime + "ns");

    }

    private List<Integer> generateData() {
        List<Integer> list = new ArrayList<>();
        Random random = new Random(System.currentTimeMillis());

        for (int i = 0; i < 1000; i++) {

            list.add(random.nextInt());

        }

        return list;
    }

    private List<Integer> insertionSort(List<Integer> list) {

        for (int elementPosition = 0; elementPosition < list.size(); elementPosition++) {

            int value = list.get(elementPosition);
            int currentElementPosition = elementPosition;

            while ((currentElementPosition > 0) &&
                    (list.get(currentElementPosition - 1) > value)) {

                Integer tempValue = list.get(currentElementPosition - 1);
                list.set(currentElementPosition, tempValue);

                currentElementPosition--;

            }

            list.set(currentElementPosition, value);


        }

        return list;
    }
}