package rad;
import java.util.Scanner;

/**
 * @author RadsammyT
 * This class is used to get user input from the console.
 * You may ask why I couldn't have just use the scanner in all .java files where input is needed,
 * but to me its more easier/convenient to use this class.
 * If the scanner throws an error, it will be caught and the user will be prompted to try again.
 * 
 */
public class InputUtils {


	static Scanner sc = new Scanner(System.in);

	public static int readInt(String input) {
		while (true) {
			try {System.out.print(input);
				int i = sc.nextInt();
				return i;
			} catch (Exception ex) {
				sc.nextLine();
			}
		}

	}

	public static long readLong(String input) {
		while (true) {
			try {
				System.out.println(input);
				long i = sc.nextLong();
				return i;
			} catch (Exception ex) {
				sc.nextLine();
			}
		}
	}

	public static String readString(String input) {
		while (true) {
			try {
				System.out.println(input);
				String i = sc.nextLine();
				return i;
			} catch (Exception e) {
				sc.nextLine();
			}

		}
	}

	public static double readDouble(String input) {
		while (true) {
			try {
				System.out.println(input);
				String i = sc.nextLine();
				return Double.parseDouble(i);
			} catch (Exception e) {
				sc.nextLine();
			}
		}
	}

/**
	 * TODO: redo readBool so that any non-boolean input will be caught and the user will be prompted to try again.
	 * Boolean.parseBoolean(input) will return false for any non-boolean input.
	 * This is literally their implementation, I shit you not:
	 * "true".equalsIgnoreCase(input);
	 * end implementation. holy fucking shit.
	 * 
	 */

	public static boolean readBool(String input) {
		/* 
		while (true) {
			try {
				System.out.print(input);
				String i = sc.nextLine();
				return Boolean.parseBoolean(i);
			} catch (Exception e) {
				System.out.println(e.getLocalizedMessage());
				
				sc.nextLine();
			}
		}
		*/
		// old implementation
		
		while (true) {
			try {
				System.out.println(input);
				String i = sc.nextLine();
				if (i.equalsIgnoreCase("true") ||
						i.equalsIgnoreCase("yes")
					) {
					return true;
				}
				
				if (i.equalsIgnoreCase("false")) {
					return false;
				}
			} catch (Exception e) {
				continue;
			}
		}
	}

	

}