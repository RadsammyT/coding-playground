package ttt.revamp;

public class Grid {


    public String[][] arr;
    public int size;
    public Grid(int size) {
        this.arr = new String[size][size];
        this.size = size;
        for (int i = 0; i < size; i++) {
            for (int j = 0; j < size; j++) {
                arr[i][j] = "-";
            }
        }
    }

    public int getSize() {
        return this.size;
    }

    public String getCell(int i, int j) {
        return arr[i][j];
    }

    public void setCell(int i, int j, String as) {
        arr[i][j] = as;
    }

    public String toString() {
        String result = "";
        for (int i = 0; i < arr.length; i++) {
            for (int j = 0; j < arr.length; j++) {
                result += arr[i][j] + " ";
            }
            result += "\n";
        }
        return result;
    }


}
