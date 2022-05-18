package rad;

import java.util.Random;

public class StringRandom {
    public static String run(String in) {
        var r = new Random();
		int begin = 0;
		r.setSeed(System.currentTimeMillis());
		begin = r.nextInt(in.length());
        return in.substring(
                begin,
                r.nextInt(begin, in.length() + 1)
            );
			
		
    }
}
