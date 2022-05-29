import java.util.HashMap;
import java.util.Scanner;
import java.util.ArrayList;
import java.util.List;
import jFrame.ttt_gui;

public class Select extends Exception {
    public Select(String err) {
        super(err);
    }

    public static Runnable run() throws Select {
        // HashMap<Integer, Runnable> h = new HashMap<>();
        // h.put(1, ttt_gui.run());
        // Scanner sc = new Scanner(System.in);
        
        // h.get(sc.nextInt());
        // sc.close();
        Scanner sc = new Scanner(System.in);
        
        System.out.println("0: ttt_gui \n" +
                            "1: ttt CLI \n" +
                            "2: ShitShuffler");

        int selection = sc.nextInt();

        switch (selection) {
            case 0:
                return ttt_gui.run();
            case 1:
                return ttt.Game.run();
            case 2:
                return rad.ShitShuffler.run();
        
            default:
                throw new Select("invalid selection");
        }
        
    }


    
}
