// DEPRECIATED
// DO NOT USE
// STINKY NIG-

package rad;

public class LongCompress {

	public static double run(long value)
	{
		
		long recipe = (2147483000 + 49 + 600) * 2;
		long compact = value / recipe;
		return (double) compact;
		
	}
	
}
