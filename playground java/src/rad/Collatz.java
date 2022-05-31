package rad;

import java.util.Scanner;
import java.lang.Exception;
import rad.NegativeNumberException;
import java.util.List;
import java.util.ArrayList;

public class Collatz {



	/**
	 @param in the number to be tested
	 @param printSteps if true, print the steps taken to reach 1
	 @return array of longs being [odds, evens, steps]
	 @throws NegativeNumberException if in becomes negative during process
	 */

	public static long[] run(long in , boolean printSteps) throws NegativeNumberException

	{


		long iter = 0;
		long odd = 0;
		long even = 0;

		if ( in == -1) {
			Scanner sc = new Scanner(System.in);
			System.out.print("in is -1. set new value: "); in = sc.nextLong();
			sc.close();
		}

		do {
			if ( in % 2 == 1) {
				in = (3 * in ) + 1; // applies 3x + 1 to an odd number
				iter++;
				odd++; // logging update
				if (printSteps == true) {
					System.out.println( in );
				} // prints current number if printSteps is true
			}

			if ( in % 2 == 0) {
				in /= 2; //halves the even number
				iter++;
				even++;
				if (printSteps == true) {
					System.out.println( in );
				}
			}
			if ( in < 0) {
				throw new NegativeNumberException("Number is negative. Are you using an extremely large as f*ck input?");
				//return -3;

			}

		}
		while ( in != 1);
		return new long[] {
			odd,
			even,
			iter
		};
	}
	// uses a shorthand version of .run, returnMode defaults to 0, printSteps defaults to false.
	public static long[] run(long in ) throws NegativeNumberException {
		
		return run( in , false);
		//int returnStatement = Collatz.run(in, 0, false);
		//return returnStatement;

	}


}
/** 
class NegativeNumberException extends Exception
{
	public NegativeNumberException(String errorString)
	{
		super(errorString);
	}
}
*/