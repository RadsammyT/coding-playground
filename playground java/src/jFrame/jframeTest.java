package jFrame;
import java.util.concurrent.TimeUnit;
import java.awt.*;
import javax.swing.*;
import javax.swing.border.*;
// TODO change everything.
// bruh moment incoming
public class jframeTest {

	
	
	public static void init() throws InterruptedException
	{
		int codeCount = 0;
		int upgrade1 = 1;
		
		ImageIcon icon = new ImageIcon("cypher.jpg");
		//JLayeredPane layeredPane = new JLayeredPane();
		//layeredPane.setBounds(0,0,1000,1000);
		
		JButton upgrade1Button = new JButton();
		JButton codeWrite = new JButton();
		JLabel codeCountLabel = new JLabel();
		JLabel titleLabel = new JLabel();
		titleLabel.setText("bob good simulator"
				+ " now in java!... which"
				+" is the language of p a i n");
		titleLabel.setForeground(new Color(255,0,0));
		titleLabel.setBounds(50,0,500,100);
		
		
		
		JButton testButton;
		testButton = new JButton();
		
		testButton.setBounds(150,400,150,50);
		//testButton.addActionListener(e-> onTestButtonClick(testButton));
		testButton.setText("PLAY");
		testButton.setFont(new Font("Fixedsys",Font.BOLD,15));
		
		//Border border = BorderFactory.createLineBorder(new Color(255,255,255), 3);
		
		
		
		

		JLabel countdown = new JLabel();
		countdown.setText(Integer.toString(23));
		countdown.setForeground(new Color(255,0,0));
		countdown.setBounds(100,50,50,50);
		countdown.setVisible(false);
		
		
		JFrame frame = new JFrame();
		frame.setSize(500,500);
		frame.setTitle("yeeter");
		frame.setResizable(false);
		frame.setVisible(true);
		frame.setLayout(null);
		frame.setIconImage(icon.getImage());
		
		
		frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
		
		frame.add(titleLabel);
		frame.add(countdown);
		frame.add(testButton);
		frame.add(codeCountLabel);
		frame.add(upgrade1Button);
		frame.add(codeWrite);
		testButton.addActionListener(e->{
			
			System.out.println("TEST");			
			testButton.setVisible(false);
			titleLabel.setVisible(false);
			
			
			
			
			


			// sets up the buttons			
			codeCountLabel.setBounds(100,100,50,10);
			codeCountLabel.setText("code: " + codeCount);
			codeCountLabel.setVisible(true);

			codeWrite.setBounds(100,125,200,50);
			codeWrite.setText("smack keyboard");
			codeWrite.setVisible(true);
			
			upgrade1Button.setBounds(400,0,100,50);
			upgrade1Button.setVisible(true);
			
			// WOOHOO LAMBDA EXPRESSIONS AMIRITE
	}); 
		
		
		
		
		
	}
	
	
	
	public void beginGame()
	{
		
	}
	
	
	/**
	public static void onTestButtonClick(JButton other)
	{
		
		
		
		System.out.println("TEST");			
		other.setVisible(false);
		if(other.getText().contains("i hate")) // if the text is similar to testButton
		{
			System.out.println("works");
			titleLabel.setVisible(false);
		}
		
	}
	**/
	
}
