package rad;
import rad.TextboxSorcery;
import java.util.concurrent.TimeUnit;
public class Credits {
	public static void run() throws InterruptedException
	{
		write("project BLOCK WAR", 100);
		TimeUnit.SECONDS.sleep(1);
		write("by RadsammyT \n", 100);
		TimeUnit.SECONDS.sleep(1);
		write("inspired by:", 100);
		TimeUnit.SECONDS.sleep(1);
		write("RETURNAL | NOITA | INSCRYPTION | AURAWATCH", 100);
		write("BY HOUSEMARQUE | BY NOLLA GAMES | BY DANIEL MULLINS | BY BOB GOOD & PEDROSANTOS53", 25);
		
	}
	
	
	public static void write(String string, int delay) throws InterruptedException 
	{
		
		try {
		for(int i = 0; i < (short) string.length() ; i++)
		{
			System.out.print(string.substring((i), (i+1)));	 	// basically prints out the string letter by letter.
			TimeUnit.MILLISECONDS.sleep(delay);
		}
		} 
		catch(InterruptedException e) {}
		System.out.println("");
	}
}
