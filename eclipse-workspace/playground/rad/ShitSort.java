package rad;
import java.util.Random;
import java.util.concurrent.TimeUnit;
import ttt.Game;
import rad.Timer;
public class ShitSort {
	static int[] bag = {0,0,0,0,0,0,0};
	static int failedRolls = 0;
	static int succeededRolls = 0;
	static long seed = 0;
	
	static Random test = new Random();
	public static void runLoop(long repeat) throws InterruptedException
	{
		Timer time = new Timer();
		while(!(succeededRolls == repeat))
		{
		time.startTimer();
		 Random test = new Random();
		while(!arrayIsUnique())
		{
			randomizeBag();
			randomizeSeed();
//			if(failedRolls == 50)
//			{
//				bag = new int[] {5,4,3,2,1,0,6};
//			}
			
			//System.out.println(arrayToString().compareTo("0123456"));
			//System.out.println(arrayToString() + " : " + arrayIsUnique() + " : " + failedRolls);
			failedRolls++;
			//TimeUnit.MILLISECONDS.sleep(10);
			
		}
		time.endTimer();
		System.out.println( arrayToString() + " : " + (failedRolls - 1) + " : " + (succeededRolls + 1) + " : " + time.getElapseNano());
		failedRolls = 0;
		bag = new int[] {0,0,0,0,0,0,0};
		succeededRolls++;
		TimeUnit.MILLISECONDS.sleep(100);
		}
	}
	
	
	public static void run(long repeat, long seed) throws InterruptedException
	{
		
		test.setSeed(seed);
		for(int i = 0; i < repeat ; i++)
		{
		while(!arrayIsUnique())
		{
			randomizeBag();
			randomizeSeed();
			failedRolls++;
			
		}
		
		System.out.println( arrayToString() + " : " + (failedRolls - 1));
		}
	}
	
	public static void randomizeBag()
	{
		for(int i=0; i < bag.length ; i++)
		{
			bag[i] = test.nextInt(7);
		}
	}
	
	public static void randomizeSeed()
	{
			test.setSeed(test.nextLong() / 1 + failedRolls);	
	}
	
	public static String arrayToString()
	{
		String aTS = "";
		
		for(int i = 0; i < bag.length ; i++)
		{
			aTS = aTS + bag[i];
		}
		
		return aTS;
	}
	/*
	 * lets say we have an input called "1234560"
	 * 
	 * our board is empty, so were on a clean slate.
	 * 
	 * for every character in the input string:
	 * 		check if the character is in the board.
	 * 		if it isn't:
	 * 			add the character to the board.
	 * 		if it is:
	 * 			end the process, return false.
	 * return true.
	 * 
	 */
	public static boolean arrayIsUnique()
	{
		String input = arrayToString();
		String board = "";
		
		for(int i = 0 ; i < input.length() ; i++)
		{
			if(!Game.containsIgnoreSequence(Character.toString(input.charAt(i)), board))
			{
				board = board + input.charAt(i);
			}
			else
			{
				return false;
			}
		}
		
		return true;
	}
}
