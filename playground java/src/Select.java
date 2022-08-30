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
                        int length = rad.InputUtils.readInt("length?: ");
                        int numOfThreads = rad.InputUtils.readInt("threads?: ");
                        for (int i = 1; i < numOfThreads + 1; i++) {
                            try {
                                Thread.sleep(10, 10);
                            } catch (InterruptedException e) {
                                // Auto-generated catch block
                                // e.printStackTrace();
                            }
                            Thread obj = new Thread(new rad.ShitShufflerThread(System.nanoTime() + (i + 1), length));
                            obj.start();
                        }
                }    
                },
            
            new Runnable() {
                public void run() {
                    rad.ShitShuffler.run();
                }
            },
            new Runnable() {
                public void run() {
                    jFrame.ShitShuffler.init();
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
            
            

    };
       
        
        System.out.println("1: shitshuffler: multithreading edition \n" +
                            "2: shitshuffler \n" +
                            "3: shitshuffler: jFrame edition \n" +
                            "4: tic-tac-toe old \n" +
                            "5: tic-tac-toe old gui \n" +
                            "6: tic-tac-toe revamp \n" + 
                            "7: test runnable");

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
