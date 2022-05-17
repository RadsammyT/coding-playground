package rad;

import java.util.Scanner;
import java.lang.Exception;
import rad.NegativeNumberException;
import java.util.List;
import java.util.ArrayList;

public class Collatz{
	
	
	
	/**
	 *  @param in the number being applied with the Collatz Conjecture
	 * {
	 * integers are used within this parameter. Numbers beyond the limit are not possible
	 * If you wish to set 'in' with a value larger than the int limit...
	 * Set 'in' to -1. java.util.Scanner will prompt you for the value.
	 * }
	 * 
	 * @param returnMode (int) = what is returned on call
	 * {
	 *  0 = number of iterations
	 *  1 = number of ODD operations done
	 *  2 = number of EVEN operations done
	 *  3 = fuck return, print all iter, even and odd IN ORDER. returnMode 3 returns '-1' 
	 * }
	 * 
	 * @param printSteps (boolean) = determines if the method prints numbers for each step
	 * {
	 * 
	 * }
	 * @throws NegativeNumberException
	 */
	
	public static long[] run(long in, boolean printSteps) throws NegativeNumberException   
	
	{
		
		
		long iter = 0;
		long odd = 0;
		long even = 0;
		
		if(in == -1)
		{
			Scanner sc = new Scanner(System.in);
			System.out.print("in is -1. set new value: ");
			in = sc.nextLong();
			sc.close();
		}
		
		do
		{
			if(in % 2 == 1)
			{
				in = (3 * in) + 1; // applies 3x + 1 to an odd number
				iter++; odd++;	   // logging update
				if(printSteps==true) {System.out.println(in);} // prints current number if printSteps is true
			}
			
			if(in % 2 == 0)
			{
				in /= 2; // halves the even number
				iter++;
				even++;
				if (printSteps == true) {
					System.out.println(in);
				}
			}
			if( in < 0)
			{
				throw new NegativeNumberException("Number is negative. Are you using an extremely large as f*ck input?");
				//return -3;
				
			}

		}
		while(in != 1);
		return new long[] {odd, even, iter}; 
	}
	
	public static long[] run(long in) throws NegativeNumberException 
	// uses a shorthand version of .run, returnMode defaults to 0, printSteps defaults to false.
	{
		
		return run(in, false);
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