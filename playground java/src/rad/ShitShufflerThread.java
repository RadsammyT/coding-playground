package rad;

import java.util.Random;

public class ShitShufflerThread implements Runnable {

    long seed = 0;
    int length = 0;
	public ShitShufflerThread(long nanoTime, int length) {
        this.seed = nanoTime;
        this.length = length;
	}


    @Override
    public void run() {
        try {
            Random rand = new Random(this.seed);
            System.out.println(Thread.currentThread().getId() + " running");
            System.out.println(rad.ShitShuffler.run(this.length, true, rand));
        } catch (Exception e) {
            System.out.println("THREAD: " + Thread.currentThread().getId() + "\nERR: " + e.getLocalizedMessage());
        }
    }
    
}