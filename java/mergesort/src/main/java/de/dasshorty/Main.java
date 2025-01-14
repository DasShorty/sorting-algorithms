package de.dasshorty;

import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;
import java.util.Random;

public class Main {
    public static void main(String[] args) {

        Main main = new Main();

        List<Integer> integers = main.generateData();

        LinkedList<Integer> sortedList = main.mergeSort(integers);

        System.out.println(sortedList);


    }

    private List<Integer> generateData() {

        List<Integer> data = new ArrayList<>();
        Random random = new Random(System.currentTimeMillis());

        for (int i = 0; i < 1000; i++) {

            data.add(random.nextInt());

        }

        return data;
    }

    private LinkedList<Integer> mergeSort(List<Integer> list) {

        LinkedList<LinkedList<Integer>> groups = new LinkedList<>();

        for (Integer integer : list) {
            LinkedList<Integer> newList = new LinkedList<>();
            newList.add(integer);
            groups.add(newList);
        }

        while (groups.size() > 1) {
            groups.addLast(merge(groups.removeFirst(), groups.removeFirst()));
        }

        return groups.getFirst();

    }

    private LinkedList<Integer> merge(LinkedList<Integer> leftList, LinkedList<Integer> rightList) {
        LinkedList<Integer> temp = new LinkedList<>();

        while (!leftList.isEmpty() && !rightList.isEmpty()) {
            if (leftList.getFirst() < rightList.getFirst()) {
                temp.addLast(leftList.removeFirst());
            } else {
                temp.addLast(rightList.removeFirst());
            }
        }

        temp.addAll(leftList);
        temp.addAll(rightList);

        return temp;
    }
}