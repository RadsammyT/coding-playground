package rad;


/**
 * Used solely for the Collatz lib.
 * If the number is negative during a Collatz task, this error will throw.
 */
public  class NegativeNumberException extends Exception{
	public NegativeNumberException(String err)
    {
		super(err);
	}

}
