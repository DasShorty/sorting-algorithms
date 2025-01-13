package de.dasshorty;

import java.util.ArrayList;
import java.util.List;

public class AVLTree {

    Node root;

    int height(Node node) {
        if (node == null) {
            return 0;
        }

        return node.height;
    }

    int max(int a, int b) {
        return Math.max(a, b);
    }

    Node rightRotate(Node node) {
        Node nodeLeft = node.left;
        Node t2 = nodeLeft.right;

        nodeLeft.right = node;
        node.left = t2;

        node.height = max(height(node.left), height(node.right)) + 1;
        nodeLeft.height = max(height(nodeLeft.left), height(nodeLeft.right)) + 1;

        return nodeLeft;
    }

    Node leftRotate(Node node) {
        Node nodeRight = node.right;
        Node t2 = nodeRight.left;

        nodeRight.left = node;
        node.right = t2;

        node.height = max(height(node.left), height(node.right)) + 1;
        nodeRight.height = max(height(nodeRight.left), height(nodeRight.right)) + 1;

        return nodeRight;
    }

    int getBalance(Node node) {
        if (node == null) {
            return 0;
        }

        return height(node.left) - height(node.right);
    }

    Node insert(Node node, int number) {
        if (node == null) {
            return new Node(number);
        }

        if (number < node.number) {
            node.left = insert(node.left, number);
        } else if (number > node.number) {
            node.right = insert(node.right, number);
        } else {
            return node;
        }

        node.height = 1 + max(height(node.left), height(node.right));

        int balance = getBalance(node);

        if (balance > 1 && number < node.left.number) {
            return rightRotate(node);
        }

        if (balance < -1 && number > node.right.number) {
            return leftRotate(node);
        }

        if (balance > 1 && number > node.left.number) {
            node.left = leftRotate(node.left);
            return rightRotate(node);
        }

        if (balance < -1 && number < node.right.number) {
            node.right = rightRotate(node.right);
            return leftRotate(node);
        }

        return node;

    }

    void inOrder(Node node, List<Integer> sortedList) {
        if (node != null) {
            inOrder(node.left, sortedList);
            sortedList.add(node.number);
            inOrder(node.right, sortedList);
        }
    }

    List<Integer> treeSort(int[] array) {
        for (int key : array) {
            root = insert(root, key);
        }

        List<Integer> sortedList = new ArrayList<>();
        inOrder(root, sortedList);

        return sortedList;
    }

}