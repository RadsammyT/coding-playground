package rad;
import java.util.Scanner;

/**
 * @author RadsammyT
 * This class is used to get user input.
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

	public static boolean readBool(String input) {
		while (true) {
			try {
				System.out.print(input);
				String i = sc.nextLine();
				return Boolean.parseBoolean(i);
			} catch (Exception e) {
				continue;
			}
		}
	}


}