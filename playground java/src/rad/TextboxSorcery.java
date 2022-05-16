package rad;
import java.util.concurrent.TimeUnit;
public class TextboxSorcery{

	/**
	 * @param string The string for the method to use
	 * @param delay the delay IN milliseconds for each character
	 * @return Nothing, just prints out character by character.
	 */
	public static void run(String string, int delay) throws InterruptedException 
	{
		
		try {
			for (int i = 0; i < (short) string.length(); i++) {
				System.out.print(string.substring((i), (i + 1))); // basically prints out the string letter by letter.
				TimeUnit.MILLISECONDS.sleep(delay);
			}
		} 
		catch(InterruptedException e) {}
		System.out.println("");
	}


}
