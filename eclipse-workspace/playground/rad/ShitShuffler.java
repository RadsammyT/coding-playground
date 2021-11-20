package rad;
import java.util.Random;
import java.util.concurrent.TimeUnit;
import ttt.Game;
import rad.Timer;

/**
 * A little something I worked on. Basically the shit version of shuffling arrays.
 * 
 * fun fact: there is a SmileBASIC 4 version of this, which is pretty much the same thing except
 * its only limited to 7 slots.
 * 
 * @author RadsammyT
 *
 */
public class ShitShuffler {
	static int[] bag; // named, but not actually initialized
	static int failedRolls = 0;
	static int succeededRolls = 0;
	static long seed = 0;
	static int failedRollMarker = 1000000;
	static int lengthStatic; // this is static because we need this value for our methods later on
	
	static boolean markerSwitch = false;
	
	static Random rand = new Random();
	
	
	/**
	 * 
	 * @param length how long the bags length should be
	 * @param repeat how many times this method should run
	 * @throws InterruptedException as a just in case. usually i put this there 
	 * for TimeUnit sleep commands
	 */
	public static void runLoop(int length, long repeat) throws InterruptedException
	{
		bag = new int[length];
		lengthStatic = length; 
		Timer time = new Timer();
		while(succeededRolls != repeat)
		{
			time.startTimer();
			while(!arrayIsUnique())
			{
				randomizeBag();
				failedRolls++;
				if(failedRolls >= failedRollMarker && markerSwitch == true)
				{
					System.out.println((failedRolls /1000000) + "M...");
					failedRollMarker += 1000000;
				}
			}
		time.endTimer();
		System.out.println( arrayToStringFormatted() + " : FR > " + (failedRolls ) + " : SR > " + (succeededRolls + 1) + " : TIME > " + time.getElapseDouble());
		
		failedRolls = 0;
		failedRollMarker = 1000000;
		bag = new int[length];
		succeededRolls++;
		}
	}
	
	/**
	 * Every integer in the bag is randomized from 0 to the bags length via the rand instance 
	 * of the Random class.
	 */
	
	public static void randomizeBag()
	{
		for(int i=0; i < bag.length ; i++)
		{
			bag[i] = rand.nextInt(bag.length);
		}
	}
	
	/**
	 * @deprecated I have no fucking clue how to have different seeds running as the thread runs two 
	 * separate ShitShuffler commands.
	 * ex:
	 * 
	 * runLoop(4,1); // first run
	 * 
	 * outputs:
	 * 
	 * {0,1,2,3}
	 * 
	 * 
	 * runLoop(4,1); // second run, again in the same thread
	 * 
	 * STILL outputs
	 * 
	 * {0,1,2,3}
	 */
	
	public static void randomizeSeed()
	{
			rand.setSeed(rand.nextLong() / 1 + failedRolls);	
	}
	
	
	/**
	 * @deprecated This method returns the string without spaces. Depreciated because the
	 * fact that this method returns without spaces is a flaw in of itself.
	 * @return The string. Without spaces. I am fucking dumb
	 */
	
	public static String arrayToString()
	{
		String aTS = "";
		
		for(int i = 0; i < bag.length ; i++)
		{
			aTS = aTS + bag[i];
		}
		
		return aTS;
	}
	/**
	 * 
	 * @return The string WITH spaces this time.
	 * Only used for output while running this class' runLoop method.
	 */
	public static String arrayToStringFormatted()
	{
		String aTS = "";
		
		for(int i = 0; i < bag.length ; i++)
		{
			aTS = aTS + " " + bag[i];
		}
		
		return aTS;
	}
	
	/**
	 * @deprecated This method has no support for numbers 10 and above. 
	 * @see arrayIsUnique()
	 * @return true if any character in the String is the same as in the preceeding characters in the board.
	 * false if all characters are unique, and no matching ones are encountered in this method
	 */
	public static boolean arrayIsUniqueDEPRECIATED()
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
	/**
	 * 
	 * @return true if all indexs in the array are unique to one another.
	 */

	public static boolean arrayIsUnique()
	{
		
		int[] board = new int[lengthStatic];
		int inRef = 0;
		int boardRef = 0;
		for(int i = 0; i < board.length ; i++)
		{
			board[i] = -1;
		}
		for(int i = 0; i < bag.length ; i++)
		{
			inRef = bag[i];
			for(int j = 0; j < board.length ; j++)
			{
				boardRef = board[j];
				if(inRef == boardRef)
				{
					return false;
				}
			}
			board[i] = inRef;
			
		}
		return true;
	}
	
	
}
