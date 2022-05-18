package rad;

import java.util.Random;

public class StringRandom {
    public static String run(String in) {
        Random r = new Random();
        r.setSeed(System.currentTimeMillis());
        return run(in, r);
    }
    
    public static String run(String in, Random r) {
        
		int begin = 0;
		
		begin = r.nextInt(in.length());
        return in.substring(
                begin,
                r.nextInt(begin, in.length() + 1)
            );
    }
}
