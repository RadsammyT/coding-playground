package ttt.revamp;

import java.io.PrintStream;

public class Game {
    public static void begin() {
        PrintStream ps = System.out;
        ps.println("inputs for the grid are 1-based, not 0-based. \n coordinates are handled as if they are placed in the 4th quadrant of a coordinate plane. \n");        
        Grid grid = new Grid(rad.InputUtils.readInt("input size of grid (squared) : "));
        // System.out.println(grid.getSize());
        // System.out.println(checkWinner(grid));
        // System.out.println(checkRow(grid, 1));
        // System.out.println(checkColumn(grid, 1));
        // System.out.println(checkDiag(grid, true));
        // System.out.println(checkDiag(grid, false));
        // System.out.println(grid.toString());
        
        ps.println(grid.toString());

        boolean xFirst = false;
        xFirst = rad.InputUtils.readBool("x goes first? ");
        if (xFirst) {
            ps.println("x goes first");
            getInput(grid, "X");
            ps.println("----------------------------------------------------");
            ps.println(grid.toString());
        } else {
            ps.println("o goes first");
        }

        while (true) {
            ps.println("O's turn");
            getInput(grid, "O");
            ps.println("----------------------------------------------------");
            ps.println(grid.toString());
            if (checkWinner(grid).equals("O")) {
                ps.println("O wins!");
                break;
            } else if (checkWinner(grid).equals("tie")) {
                ps.println("It's a tie!");
                break;
            }
            ps.println("X's turn");
            getInput(grid, "X");
            ps.println("----------------------------------------------------");
            ps.println(grid.toString());
            if (checkWinner(grid).equals("X")) {
                ps.println("X wins!");
                break;
            } else if(checkWinner(grid).equals("tie")) {
                ps.println("It's a tie!");
                break;
            }
        }
    }

    public static int getInput(Grid grid, String player) {
        while (true) {
            try {
                int inputRow = rad.InputUtils.readInt("input X-axis of " + player + ": ");
                int inputCol = rad.InputUtils.readInt("input Y-axis of " + player + ": ");

                if (grid.getCell(inputCol -1, inputRow -1).equals("-"))
                    grid.setCell(inputCol -1, inputRow -1, player);
                else {
                    System.out.println("invalid input, cell already taken");
                    continue;
                }
                //grid.setCell(inputRow - 1, inputCol -1, player);
                break;
            } catch (Exception e) {
                continue;
            }
        }
        return 0;
    }

    public static String checkWinner(Grid grid) {
        for (int i = 0; i < grid.getSize(); i++) {
            switch (checkRow(grid, i)) {
                case "X":
                    return "X";
                case "O":
                    return "O";
            }
            switch (checkColumn(grid, i)) {
                case "X":
                    return "X";
                case "O":
                    return "O";
            }
        }
        switch (checkDiag(grid, true)) {
            case "X":
                return "X";
            case "O":
                return "O";
        }
        switch (checkDiag(grid, false)) {
            case "X":
                return "X";
            case "O":
                return "O";
        }

        for(int i = 0; i < grid.getSize(); i++) {
            for(int j = 0; j < grid.getSize(); j++) {
                if (grid.getCell(i, j).equals("-"))
                    return "-";
            }
        }

        return "tie";
    }
    
    public static String checkRow(Grid grid, int row) {
        String res = "";
        String oEx = "";
        String xEx = "";
        for (int i = 0; i < grid.getSize(); i++) {
            res += grid.getCell(row, i);
            oEx += "O";
            xEx += "X";
        }
        if (res.equals(oEx)) {
            return "O";
        } else if (res.equals(xEx)) {
            return "X";
        } else {
            return "-";
        }
        
    }
    
    public static String checkColumn(Grid grid, int column) {
        String res = "";
        String oEx = "";
        String xEx = "";
        for (int i = 0; i < grid.getSize(); i++) {
            res += grid.getCell(i, column);
            oEx += "O";
            xEx += "X";
        }
        if (res.equals(oEx)) {
            return "O";
        } else if (res.equals(xEx)) {
            return "X";
        } else {
            return "-";
        }
    }
    // reverse notes: true = top left to bottom right, false = bottom left to top right
    public static String checkDiag(Grid grid, boolean reverse) {
        String res = "";
        String oEx = ""; 
        String xEx = ""; // oEx / xEx are there so that we can check the result against these two later
        for (int i = 0; i < grid.getSize(); i++) {
            oEx += "O";
            xEx += "X";
            if (reverse) {
                res += grid.getCell(i, i);
            } else {
                res += grid.getCell(i, grid.getSize() - i - 1);
            }
        }
        if (res.equals(oEx)) {
            return "O";
        } else if (res.equals(xEx)) {
            return "X";
        } else {
            return "-";
        }
    }
}
