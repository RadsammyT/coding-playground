package ttt;
import rad.Input_Utils;
import java.util.Random;
public class Game {

	private static int tieCount;
	private static String checks;
	private static String checks2;
	private static String checksGrid;
	static Grid grid = new Grid();
	public static void beginGame()
	{
		
		
		System.out.println("i will tic tac your fucking toes");
		System.out.println(grid);
		
		boolean firstTurn = Input_Utils.readBool("Should X go first? true/false: ");
		
		if(firstTurn)
		{
			System.out.println(grid);
			grid.setCell(PlayerXInput(), "X");
		}
		
		//while(getWinner().equals("false"))
		while(true)
		{
			System.out.println(grid);
			grid.setCell(PlayerOInput(), "O");
			if(!getWinner().equals("false")) {break;}
			System.out.println("-------------------");
			//System.out.println(getWinner());
			System.out.println(grid);
			grid.setCell(PlayerXInput(), "X");
			System.out.println("-------------------");
			if(!getWinner().equals("false")) {break;}
			//System.out.println(getWinner());
		}
		
		if(getWinner().equals("TIE"))
		{
			System.out.println("Tie");
		}
		else
		{
			System.out.println(getWinner() + " won");
		}
		System.out.println(grid);
		
	}
	
	/**
	 * @author RadsammyT
	 *  Gets Player X's input. only accepts 0-8 that serves as index numbers.
	 * 
	 * @return the number inputted by the user.
	 * @see PlayerOInput
	 */
	public static int PlayerXInput() 
	{
		System.out.println("X's turn!");
		checks ="012345678";
		checks2 = "XO";
		checksGrid = grid.getAllCells();
		String input = "999";
		while(!(containsIgnoreSequence(input,checksGrid,1,1)))
		{
			input = Input_Utils.readString("enter a number on the grid: ");
			
		}
		
		if(containsIgnoreSequence(input,checksGrid,1,1))
		{
			
			return Integer.parseInt(input);
		}
		return (Integer) null;
		
		/*
		 * FINALLY, I GOT THIS WORKING AFTER LIKE A SHIT TON OF HOURS
		 * AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
		 * -RadsammyT
		 */
		
	}
	/**
	 * @author RadsammyT
	 *  Gets Player O's input. only accepts 0-8 that serves as index numbers
	 * @return the number inputted by the user.
	 * @see PlayerXInput
	 */
	public static int PlayerOInput()
	{
		System.out.println("O's turn!");
		checks ="012345678";
		checksGrid = grid.getAllCells();
		String input = "999";
		while(!(containsIgnoreSequence(input,checksGrid,1,1)))
		{
			input = Input_Utils.readString("enter a number on the grid: ");
		}
		
		if(containsIgnoreSequence(input,checks,1,1))
		{
			return Integer.parseInt(input);
		}
		return (Integer) null;
	}
	
	/**
	 * @author RadsammyT
	 *  based on presets, determines the winner
	 * of the current game	 
	 */
	public static String getWinner()
	{
		String pee = "";
		for(int i = 0; i < 8; i++)
		{	
			//System.out.println(i);
			switch(i)
			{
			case 0:
				pee = grid.getCell(0) + grid.getCell(1) + grid.getCell(2);
				//System.out.println(i);
				break;
			case 1:
				pee = grid.getCell(3) + grid.getCell(4) + grid.getCell(5);
				//System.out.println(i);
				break;
			case 2:
				pee = grid.getCell(6) + grid.getCell(7) + grid.getCell(8);
				//System.out.println(i);
				break;
			case 3:
				pee = grid.getCell(0) + grid.getCell(3) + grid.getCell(6);
				//System.out.println(i);
				break;
			case 4:
				pee = grid.getCell(1) + grid.getCell(4) + grid.getCell(7);
				//System.out.println(i);
				break;
			case 5:
				pee = grid.getCell(2) + grid.getCell(5) + grid.getCell(8);
				//System.out.println(i);
				break;
			case 6:
				pee = grid.getCell(2) + grid.getCell(4) + grid.getCell(6);
				//System.out.println(i);
				break;
			case 7:
				pee = grid.getCell(0) + grid.getCell(4) + grid.getCell(8);
				//System.out.println(i);
				break;
			
				
			}
			if(pee.equals("XXX"))
			{
				//System.out.println("hmm X");
				return "X";
				
			}
			else if(pee.equals("OOO"))
			{
				//System.out.println("hmm O");
				return "O";
			}
			else if(containsIgnoreSequence(pee,"012345678"))
			{
				
			}
			else if(tieCount == 8)
			{
				//System.out.println("hmm tie");
				return "TIE";
			}
			else
			{
				tieCount++;
			}
		}
		tieCount = 0;
		return "false";
	
	}
	/**
	 * Checks the input for ANY letters that occur in the ref string. 
	 * @param input The String that is checked in this method by the reference String
	 * @param ref The String that determines the reference characters for this method
	 * @param charLimit if the result of characters found is GREATER than this number,
	 * it returns false. 
	 * @return True if input contains ANY character in the reference AND the appropriate amount of letter
	 */
	public static boolean containsIgnoreSequence(String input, String ref, int charLimit, int lengthLimit)
	{
		int result = 0;
		char checkedInput;
		char checkedRef;
		for(int i = 0; i<input.length(); i++)
		{
			checkedInput = input.charAt(i);
			for( int j = 0; j < ref.length(); j++)
			{
				checkedRef = ref.charAt(j);
				if(checkedInput == checkedRef)
				{
					result++;
				}
			}
		}
		
		if(result <= charLimit && result != 0 && input.length() <= lengthLimit)
		{
			return true;
		}
		else
		{
			return false;
		}
	}
	
	public static boolean containsIgnoreSequence(String input, String ref)
	{
		return containsIgnoreSequence(input,ref, Integer.MAX_VALUE, Integer.MAX_VALUE);
	}
	
	public static boolean choiceCheck(String i)
	{
		checks2 = "XO";
		checks = "012345678";
		int index = Integer.parseInt(i);
		
		try
		{
			String gridChar = grid.getCell(index);
			if(!(containsIgnoreSequence(gridChar, checks2)) && !(containsIgnoreSequence(gridChar,checks)))
			{
				return true;
			}
		}
		catch(Exception e)
		{
			return false;
		}
		return  (Boolean) null;
	}
}
