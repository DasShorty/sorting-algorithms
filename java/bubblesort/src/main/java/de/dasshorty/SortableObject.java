package de.dasshorty;

public record SortableObject(Object data, int weight) {

    @Override
    public String toString() {
        return data.toString();
    }
}
