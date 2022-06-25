//import shit, don't mind this garbage
package rad;

import java.util.Arrays;
import java.util.Random;
import ttt.Game;
import java.util.Scanner;

/**
 * A little something I worked on. Basically the shit version of shuffling
 * arrays.
 * 
 * fun fact: there is a SmileBASIC 4 version of this, which is pretty much the
 * same thing except
 * its only limited to 7 slots.
 * 
 *
 *
 * <p>
 * HOW IT WORKS:
 * </p>
 *
 * <p>
 * Lets say when running, you specify that the bag is 10 indexes long, and that
 * there must be 10 successful rolls before stopping.
 * What ShitShuffler does is that for every index, it randomizes every one of
 * them from 0 to the bags length (exclusive, so 9 in this case).
 * this shufflers method, arrayIsUnique(), checks to see if there are any
 * duplicates found in the bag.
 * If there are ANY duplicates, throw the whole thing away, and redo the rolling
 * process.
 * If not, then tick the successful roll count to 1, and do it all again until
 * the requirement of successful rolls has been satisfied.
 *
 *
 * @author RadsammyT
 * @category pain
 */
public class ShitShuffler {
	static int[] bag; // named, but not actually initialized
	static int failedRolls = 0;
	static int succeededRolls = 0;
	static int failedRollMarker = 5000;
	static int failedRollStep = failedRollMarker;
	static int failedRollSum = 0;
	
	static boolean markerSwitch = false; 
	static Random rand = new Random();

	/**
	 * 
	 * @param length - how long the bags length should be
	 * @param repeat - how many times this method rolls successfully.
	 * @throws InterruptedException as a just in case. usually i put this there
	 *                              for TimeUnit sleep commands
	 */
	public static void runLoop(int length, long repeat) {
		//System.out.println("Buffering..."); // this was for eclipse's wierd bug with terminal outputs
		//TimeUnit.SECONDS.sleep(1);
		bag = new int[length];
		// lengthStatic = length;
		// fRLength = Integer.toString(failedRollStep).length();
		Timer time = new Timer();

		while (succeededRolls != repeat) {
			time.startTimer();
			while (!arrayIsUnique()) {
				randomizeBag();

				if (arrayIsUnique()) {
					break;
				}

				failedRolls++;
				if (failedRolls >= failedRollMarker && markerSwitch == true) {
					// System.out.print((failedRolls /(1000000f)) + "M..." + "\r");
					System.out.print(failedRolls + " FAILED          " + "\r");
					// 0 FAILED
					failedRollMarker += failedRollStep;
				}
			}
			time.endTimer();
			System.out.println(Arrays.toString(bag) + " : FR > " + (failedRolls) + " : SR > " + (succeededRolls + 1)
					+ " : TIME > " + time.getElapseDouble());
			// BAG : FR > 0 : SR > 1 : TIME > 0.0
			System.out.println((int)(failedRolls / ((double) time.getElapse() / 1000)) + " failed rolls per second");

			failedRollSum += failedRolls;			

			failedRolls = 0;
			failedRollMarker = 1000000;
			bag = new int[length];
			succeededRolls++;
		}
		
		// print the average of failedRollsSum
		System.out.println("Average failed rolls: " + (failedRollSum / succeededRolls));
	}

	/**
	 * Every integer in the bag is randomized from 0 to the bags length via the rand
	 * instance
	 * of the Random class.
	 * <p>
	 * PLEASE NOTE: This is an overload of runLoop(int, long). Please see that for
	 * details on the other parameters.
	 * </p>
	 * 
	 * @param printMarkers if true, print the number of failed rolls (ex: 4M...)
	 *                     every 1 million.
	 * @throws InterruptedException
	 */
	public static void runLoop(int length, long repeat, boolean printMarkers) {
		markerSwitch = printMarkers;
		runLoop(length, repeat);
		markerSwitch = false; // as a just-in-case.
	}

	/**
	 * This is a version of ShitShuffler that ACTUALLY returns an array instead of
	 * printing it.
	 * 
	 * @param length the length of the array to return
	 * @return the shuffled array
	 */
	public static int[] run(int length) {
		bag = new int[length];
		// lengthStatic = length;
		while (!arrayIsUnique()) {
			randomizeBag();
		}
		return bag;
	}

	public static Runnable run() {
		// input of length and repeat
		Scanner sc = new Scanner(System.in);
		System.out.print("How long should the bag be?     ");
		int len = sc.nextInt();

		System.out.print("How many times should the bag be shuffled?     ");
		long rep = sc.nextLong();

		sc.close();
		runLoop(len, rep, true);
		return null;

	}

	public static void randomizeBag() {
		for (int i = 0; i < bag.length; i++) {
			bag[i] = rand.nextInt(bag.length);
		}
	}

	/**
	 * @deprecated I have no fucking clue how to have different seeds running as the
	 *             thread runs two
	 *             separate ShitShuffler commands.
	 *             ex:
	 * 
	 *             runLoop(4,1); // first run
	 * 
	 *             outputs:
	 * 
	 *             {0,1,2,3}
	 * 
	 * 
	 *             runLoop(4,1); // second run, again in the same thread
	 * 
	 *             STILL outputs
	 * 
	 *             {0,1,2,3}
	 */

	public static void randomizeSeed() {
		rand.setSeed(rand.nextLong() / 1 + failedRolls);
	}

	/**
	 * @deprecated This method returns the string without spaces. Depreciated
	 *             because the
	 *             fact that this method returns without spaces is a flaw in of
	 *             itself.
	 * @return The string. Without spaces. I am fucking dumb
	 */

	public static String arrayToString() {
		String aTS = "";

		for (int i = 0; i < bag.length; i++) {
			aTS = aTS + bag[i];
		}

		return aTS;
	}

	/**
	 * @deprecated i found a better way to do this. This method is no longer used. It is here for reference.
	 * @return The string WITH spaces this time.
	 *         Only used for output while running this class' runLoop method.
	 */
	public static String arrayToStringFormatted() {
		String aTS = "";

		for (int i = 0; i < bag.length; i++) {
			aTS = aTS + " " + bag[i];
			/*
			 * if(Integer.toString(bag[i]).length() == 1)
			 * {
			 * aTS = aTS + "  " + bag[i];
			 * }
			 * else
			 * {
			 * aTS = aTS + " " + bag[i];
			 * }
			 */ // please do not ask why I did this
		}

		return aTS;
	}

	/**
	 * @deprecated This method has no support for numbers 10 and above.
	 * @see arrayIsUnique()
	 * @return true if any character in the String is the same as in the preceeding
	 *         characters in the board.
	 * @return if all characters are unique, and no matching ones are encountered in
	 *         this method
	 * I'm keeping this function here since 
	 */
	public static boolean arrayIsUniqueDEPRECIATED() {
		String input = arrayToString();
		String board = "";

		for (int i = 0; i < input.length(); i++) {
			if (!Game.containsIgnoreSequence(Character.toString(input.charAt(i)), board)) {
				board = board + input.charAt(i);
			} else {
				return false;
			}
		}

		return true;
	}

	/**
	 * 
	 * @return true if all indexs in the array are unique to one another.
	 */

	public static boolean arrayIsUnique() {

		for (int i = 0; i < bag.length; i++) {
			for (int j = 0; j < bag.length; j++) {
				if (i != j && bag[i] == bag[j]) {
					return false;
				}
			}
		}
		return true;
	}

	//let see if copilot can optimize arrayIsUnique()


}
