import java.util.HashMap;
import java.util.Scanner;

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
                jFrame.ttt_gui.run();
            }
        });
        map.put(4, new Runnable() {
            public void run() {
                ttt.revamp.Game.begin();
            }
        });
        
        System.out.println("1: shitshuffler \n" +
                            "2: tic-tac-toe old \n" +
                            "3: tic-tac-toe old gui \n" +
                            "4: tic-tac-toe revamp \n");


        map.get(rad.InputUtils.readInt(
            ""
        )).run();

        return null;

        
    }


    
}
