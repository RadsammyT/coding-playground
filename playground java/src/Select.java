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
        HashMap<Integer, Runnable> map = new HashMap<Integer, Runnable>();
        map.put(1, new Runnable() {
            public void run() {
                rad.ShitShuffler.run();
            }
        });
        map.put(2, new Runnable() {
            public void run() {
                ttt.Game.run();
            }
        });
        map.put(3, new Runnable() {
            public void run() {
                ttt_gui.init();
            }
        });

        Scanner sc = new Scanner(System.in);
        
        System.out.println("1: shitshuffler \n" +
                            "2: tic-tac-toe \n" +
                            "3: tic-tac-toe gui");


        try {
            map.get(sc.nextInt()).run();
        } catch (Exception e) {
            System.out.println("INVALID:" + e.getLocalizedMessage());
        }
        sc.close();

        return null;

        
    }


    
}
