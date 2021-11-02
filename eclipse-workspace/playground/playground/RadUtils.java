package playground;

import java.util.Scanner;
public class RadUtils
{
	//static Scanner sc = new Scanner(System.in);
	private static Scanner sc;
	/**
	 * user can input any with read(X) method
	 * failsafes ARE included, if any exceptions are thrown,
	 * return -1 OR "Error".
	 */
	public static int readInt(String input)
	{
		
		try {
		System.out.print(input);
		int i = sc.nextInt();
		return i;
		}
		catch(Exception ex) {
			System.out.println("invalid");
			return -1;
		}
		
		
	}
	
	public static long readLong(String input) {
		try {
			System.out.print(input);
			long i = sc.nextLong();
			return i;
			}
			catch(Exception ex)
			{
				System.out.println("invalid");
				return -1;
			}
	}
	
	public static String readString(String input) {
		try {
			System.out.print(input);
			String i = sc.nextLine();
			return i;
			}
			catch(Exception ex)
			{
				System.out.println("invalid");
				return "Error";
			}
	}
	
	public static double readDouble(String input) {
		try {
			System.out.print(input);
			double i = sc.nextDouble();
			return i;
			}
			catch(Exception ex)
			{
				System.out.println("invalid");
				return -1;
			}
	}
}
