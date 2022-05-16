package jFrame;
import java.awt.*;
import javax.swing.*;
import ttt.*;

import ttt.Grid;
//import ttt.Game;
//import java.util.ArrayList;
public class ttt_gui {
// they are all static so that they can be accessed by all methods within this class
	static Grid grid = new Grid();
	static JFrame frame = new JFrame();
	static JButton zerozero = new JButton();
	static JButton zeroone = new JButton();
	static JButton zerotwo = new JButton();
	static JButton onezero = new JButton();
	static JButton oneone = new JButton();
	static JButton onetwo = new JButton();
	static JButton twozero = new JButton();
	static JButton twoone = new JButton();
	static JButton twotwo = new JButton();
	
	static JButton startX = new JButton();
	static JButton startO = new JButton();
	static JButton reset = new JButton();
	
	static JButton[] gridButton = {zerozero, zeroone, zerotwo, onezero,oneone,onetwo,twozero,twoone,twotwo};
	
	static JLabel turn = new JLabel();
	static JLabel winner = new JLabel();
	
	static JLabel title = new JLabel();
	static String CURRENT_TURN = "???";
	static final String TURN_X = "X";
	static final String TURN_O = "O";
	public static void init()
	{
		//JButton[] gridButton = {zerozero, zeroone, zerotwo, onezero,oneone,onetwo,twozero,twoone,twotwo};
		/* button layout
		 * 
		 * 00 01 02
		 * 10 11 12 but nahhhhh an array should be enough
		 * 20 21 22
		 */
		//javaFonts();
		ImageIcon icon = new ImageIcon("C:/Users/RadsammyT/eclipse-workspace/playground/cypher.jpg");
		ImageIcon titleIcon = new ImageIcon("C:/Users/RadsammyT/eclipse-workspace/playground/cypher2ttt_fix.jpg");
		JFrame frame = new JFrame();
		frame.setLayout(null);
		frame.setSize(550,500);
		frame.setVisible(true);
		frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
		frame.setResizable(false);
		frame.setTitle("tic tac fuck");
		frame.setIconImage(icon.getImage());
		//JButton startX = new JButton();
		startX.setBounds(10,50,200,50);
		startX.setText("play (X FIRST)");
		startX.setVisible(true);
		startX.addActionListener(e->{
			//System.out.println("fuck");
			begin("X");
		});
		
		startO.setBounds(320,50,200,50);
		startO.setText("play (O FIRST)");
		startO.setVisible(true);
		startO.addActionListener(e->{
			//System.out.println("fuck");
			begin("O");
		});
		
		title.setIcon(titleIcon);
		title.setVisible(true);
		title.setBounds(-50,0,500,500);
		
		reset.setVisible(false);
		reset.setBounds(450,420,83,40);
		reset.setText("MENU");
		reset.addActionListener(e->{
			reset();
		});
		
		frame.add(startX);
		frame.add(startO);
		frame.add(winner);
		frame.add(turn);
		frame.add(reset);
		frame.add(title);
		for(JButton i: gridButton)
		{
			frame.add(i);
			i.setVisible(true); // just in case 
		}
	}
	
	public static void begin(String startingTurn)
	{
		CURRENT_TURN = startingTurn;
		
		startX.setVisible(false);
		startO.setVisible(false);
		title.setVisible(false);
		turn.setVisible(true);
		turn.setBounds(480,50,1000,25);
		turn.setText("turn: " + CURRENT_TURN);
		reset.setVisible(true);
		// turn.setForeground(new Color(255,0,0));
		
		for(JButton i: gridButton)
		{
			//frame.add(i);
			i.setFont(new Font("NSimSun", Font.PLAIN, 80));
			i.setText("-");
			i.setVisible(true); // just in case 
		}
		
		for(int i = 0 ; i < gridButton.length ; i++)
		{
			if(i<3)
			{
				gridButton[i].setBounds(i*150,10,150,150);
			}
			
			else if(i<6)
			{
				gridButton[i].setBounds((i-3)*150,(10+150),150,150);
			}
			
			else if(i<9)
			{
				gridButton[i].setBounds((i-6)*150,(10+300),150,150);
			}
			
//			gridButton[i].addActionListener(e->{
//				clickButton(startXV);
//			});
//			startXV++;
		}
		//startXV = 1;
		actionFuck();
	}
	
	public static void javaFonts()
	{
		String fonts[] = 
	 GraphicsEnvironment.getLocalGraphicsEnvironment().getAvailableFontFamilyNames();

			    for (int i = 0; i < fonts.length; i++) {
			      System.out.println(fonts[i]);
			    }
	}
	
	public static void clickButton(int i)
	{
		if(gridButton[i].getText().equals("-"))
		{
			/*
			gridButton[i].setText(CURRENT_TURN); 
			grid.setCell(i, CURRENT_TURN);
			System.out.println(grid.toString());
			if(CURRENT_TURN.equals(TURN_X)) {CURRENT_TURN = TURN_O;}
			else if(CURRENT_TURN.equals(TURN_O)) {CURRENT_TURN = TURN_X;}
			turn.setText("turn: "+CURRENT_TURN);
			*/
				if(getWinner().equals("false"))
				{
				gridButton[i].setText(CURRENT_TURN); 
				grid.setCell(i, CURRENT_TURN);
				//System.out.println(grid.toString());
				
				if(CURRENT_TURN.equals(TURN_X)) {CURRENT_TURN = TURN_O;}
				else if(CURRENT_TURN.equals(TURN_O)) {CURRENT_TURN = TURN_X;}
				
				turn.setText("turn: "+CURRENT_TURN);
				}
				if(!getWinner().equals("false")) {
					winner.setBounds(480,150,100,100);
					if(getWinner().equals("TIE"))
					{
					//System.out.println("WIN");
					winner.setText(getWinner());
					}
					else
					{
						winner.setText(getWinner() + " WINS.");
					}
					
				winner.setVisible(true);
			}
				
		}
	}
	
	/**
	 * <p>
	 * the reason why it looks like this shit is because for loops don't fix shit for this, it just sets their clickButton values to one number only
	 * and i fucking hate it
	 * </p>
	 * CURSE YOU LAMBDA EXPRESSIONS (if thats really whats causing it idfk)
	 */
	public static void actionFuck()
	{
		
		gridButton[0].addActionListener(e->{clickButton(0);});
		gridButton[1].addActionListener(e->{clickButton(1);});
		gridButton[2].addActionListener(e->{clickButton(2);});
		gridButton[3].addActionListener(e->{clickButton(3);});
		gridButton[4].addActionListener(e->{clickButton(4);});
		gridButton[5].addActionListener(e->{clickButton(5);});
		gridButton[6].addActionListener(e->{clickButton(6);});
		gridButton[7].addActionListener(e->{clickButton(7);});
		gridButton[8].addActionListener(e->{clickButton(8);});
		
		
	}
	
	public static void reset()
	{
		for(JButton i: gridButton)
		{
			i.setVisible(false);
			i.setText("-");
			
		}
		for(int i = 0; i < grid.grid.length;i++)
		{
			grid.setCell(i, Integer.toString(i));
		}
		winner.setVisible(false);
		turn.setVisible(false);
		reset.setVisible(false);
		
		startX.setVisible(true);
		startO.setVisible(true);
		title.setVisible(true);
	}
	// BELOW ARE OBLIGATORY COPY AND PASTES FROM ttt.Game because I can't somehow import em properly
	public static String getWinner()
	{
		//tieCount = 0;
		String pee = "";
		for(int i = 0; i < 8; i++)
		{	
			switch(i)
			{
			case 0:
				pee = grid.getCell(0) + grid.getCell(1) + grid.getCell(2);
				break;
			case 1:
				pee = grid.getCell(3) + grid.getCell(4) + grid.getCell(5);
				break;
			case 2:
				pee = grid.getCell(6) + grid.getCell(7) + grid.getCell(8);
				break;
			case 3:
				pee = grid.getCell(0) + grid.getCell(3) + grid.getCell(6);
				break;
			case 4:
				pee = grid.getCell(1) + grid.getCell(4) + grid.getCell(7);
				break;
			case 5:
				pee = grid.getCell(2) + grid.getCell(5) + grid.getCell(8);
				break;
			case 6:
				pee = grid.getCell(2) + grid.getCell(4) + grid.getCell(6);
				break;
			case 7:
				pee = grid.getCell(0) + grid.getCell(4) + grid.getCell(8);
				break;
			}
			if(pee.equals("XXX"))
			{
				return "X";
			}
			else if(pee.equals("OOO"))
			{
				return "O";
			}
			else if(containsIgnoreSequence(pee,"012345678")){}
			
			
		}
		
		if(tieCheck())
		{
			return "TIE";
		}
		
		return "false";
		
		
	}
	
	public static boolean tieCheck()
	{
		int tie = 0;
		for(String i: grid.grid)
		{
			if(!(containsIgnoreSequence(i,"012345678")))
			{
				tie++;
			}
		}
		if(tie >= 9) {return true;}
		else {return false;}
	}
	
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
}
