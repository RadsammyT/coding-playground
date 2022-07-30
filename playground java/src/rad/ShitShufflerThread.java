package rad;

import java.util.Random;

public class ShitShufflerThread implements Runnable {

    @Override
    public void run() {
        try {
            Random rand = new Random(System.currentTimeMillis());
            System.out.println(Thread.currentThread().getId() + " running");
            System.out.println(rad.ShitShuffler.run(15, true, rand));
        } catch (Exception e) {
            System.out.println("THREAD: " + Thread.currentThread().getId() + "\nERR: " + e.getLocalizedMessage());
        }
    }
    
}