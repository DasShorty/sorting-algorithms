package de.dasshorty;

import java.util.List;

public class Main {
    public static void main(String[] args) {
        int[] array = {10, 3, 5, 7, 8, 2, 1};

        AVLTree tree = new AVLTree();

        List<Integer> sortedList = tree.treeSort(array);

        System.out.println(sortedList);

    }
}