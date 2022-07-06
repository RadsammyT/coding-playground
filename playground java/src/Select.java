import java.util.HashMap;
import java.util.Scanner;
import java.lang.*;
public class Select extends Exception {
    public Select(String err) {
        super(err);
    }

    public static Runnable run() throws Select {
        Runnable[] arr = {
            new Runnable() {
                public void run() {
                    rad.ShitShuffler.run();
                }
            },
            new Runnable() {
                public void run() {
                        ttt.Game.run();
                }
            },
                
                new Runnable() {
                public void run() {
                    jFrame.ttt_gui.run();
                }
            },
            new Runnable() {
                public void run() {
                    ttt.revamp.Game.begin();
                }
            }
    };
       
        
        System.out.println("1: shitshuffler \n" +
                            "2: tic-tac-toe old \n" +
                            "3: tic-tac-toe old gui \n" +
                            "4: tic-tac-toe revamp \n");

        try {
            arr[rad.InputUtils.readInt("") - 1].run();
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Not in range");
        } catch (Exception e) {
            System.out.println("Unknown Error");
            System.out.println(e.getLocalizedMessage());
        }
        return null;

        
    }


    
}
