package playground;
import rad.Timer;
import java.nio.file.*;
import java.io.*;
import java.util.List;
import java.util.concurrent.TimeUnit;
import javax.swing.JFrame;
import rad.jFrame.*;
import javax.swing.ImageIcon;
import ttt.*;
public class Main 
{

	public static void main(String[] args) throws InterruptedException, IOException
	{	
		/*
Timer testTimer = new Timer();
		
		System.out.println("Buffering");
		TimeUnit.SECONDS.sleep(1);
		//rad.jFrame.jframeTest.init();
		testTimer.startTimer();
		//TimeUnit.SECONDS.sleep(343);

		
		

		Path pathtest = Paths.get("C:/Users/RadsammyT/eclipse-workspace/playground/test.txt");
		
		FileWriter writer = new FileWriter("test.txt");
		writer.write("Line 1 \nLine 23\n" + rad.Input_Utils.readString("third line or ur gae"));
		
		writer.close();
		List<String> textLine = Files.readAllLines(pathtest);
		String[] test = {"yeet","yoot","æ±ªæ± ã�®ã�µã��ã�¾ã�„ã‚¢ã‚¹ãƒˆãƒƒãƒ—"};
			
		
		
		for(String strTest: textLine)
		{
			rad.TextboxSorcery.run(strTest, 50);
		}
		for(String strTest: test)
		{
			rad.TextboxSorcery.run(strTest, 50);
		}
		 
		testTimer.endTimer();
		rad.TextboxSorcery.run("time: " + Double.toString((testTimer.getElapseDouble())), 60);
	
		*/
		
		
		
		//System.out.println(Game.PlayerXInput());
		
		Timer test = new Timer();
		test.startTimer();
		Game.beginGame();
		test.endTimer();
		System.out.println(test.getElapseDouble());
		
		//rad.ShitShuffler.runLoop(20, 10, true);
	}
}
	
