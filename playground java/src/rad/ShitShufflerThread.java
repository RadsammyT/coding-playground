package rad;

import java.util.Random;

public class ShitShufflerThread implements Runnable {

    long seed = 0;
	public ShitShufflerThread(long nanoTime) {
        this.seed = nanoTime;
	}


    @Override
    public void run() {
        try {
            Random rand = new Random(this.seed);
            System.out.println(Thread.currentThread().getId() + " running");
            System.out.println(rad.ShitShuffler.run(15, true, rand));
        } catch (Exception e) {
            System.out.println("THREAD: " + Thread.currentThread().getId() + "\nERR: " + e.getLocalizedMessage());
        }
    }
    
}