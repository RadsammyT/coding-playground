import java.util.HashMap;
import java.util.List;
import java.util.Scanner;
import java.lang.*;
import java.util.ArrayList;
import java.util.Arrays;
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
            },
            new Runnable() {
                public void run() {
                        List<Long> test = new ArrayList<Long>();
                        if (test.isEmpty()) {
                            for (int i = 1; i < 100 + 1; i++) {
                                test.add((long) Math.pow((double) i, (double) i));
                            }
                        }
                        System.out.println(Arrays.toString(test.toArray()));
                }
            },
            
            new Runnable() {
                public void run() {
                        int numOfThreads = 10;
                        for (int i = 0; i < numOfThreads; i++) {
                            Thread obj = new Thread(new rad.ShitShufflerThread());
                            obj.start();
                        }
                }    
            }
    };
       
        
        System.out.println("1: shitshuffler \n" +
                            "2: tic-tac-toe old \n" +
                            "3: tic-tac-toe old gui \n" +
                            "4: tic-tac-toe revamp \n" + 
                            "5: test runnable \n" +
                            "6: shitshuffler: multithreading edition");

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
