package jFrame;
import java.awt.*;
import javax.swing.*;
import ttt.*;

import ttt.Grid;
//import ttt.Game;
//import java.util.ArrayList;
public class ttt_gui {
	// they are all static so that they can be accessed by all funcs within this class
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

	static JButton[] gridButton = { zerozero, zeroone, zerotwo, onezero, oneone, onetwo, twozero, twoone, twotwo };

	static JLabel turn = new JLabel();
	static JLabel winner = new JLabel();

	static JLabel title = new JLabel();
	static String CURRENT_TURN = "???";
	static final String TURN_X = "X";
	static final String TURN_O = "O";

	public static void init() {
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
		frame.setSize(550, 500);
		frame.setVisible(true);
		frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
		frame.setResizable(false);
		frame.setTitle("tic tac fuck");
		frame.setIconImage(icon.getImage());
		//JButton startX = new JButton();
		startX.setBounds(10, 50, 200, 50);
		startX.setText("play (X FIRST)");
		startX.setVisible(true);
		startX.addActionListener(e -> {
			//System.out.println("fuck");
			begin("X");
		});

		startO.setBounds(320, 50, 200, 50);
		startO.setText("play (O FIRST)");
		startO.setVisible(true);
		startO.addActionListener(e -> {
			//System.out.println("fuck");
			begin("O");
		});

		title.setIcon(titleIcon);
		title.setVisible(true);
		title.setBounds(-50, 0, 500, 500);

		reset.setVisible(false);
		reset.setBounds(450, 420, 83, 40);
		reset.setText("MENU");
		reset.addActionListener(e -> {
			reset();
		});

		frame.add(startX);
		frame.add(startO);
		frame.add(winner);
		frame.add(turn);
		frame.add(reset);
		frame.add(title);
		for (JButton i : gridButton) {
			frame.add(i);
			i.setVisible(true); // just in case 
		}
	}

	public static void begin(String startingTurn) {
		CURRENT_TURN = startingTurn;

		startX.setVisible(false);
		startO.setVisible(false);
		title.setVisible(false);
		turn.setVisible(true);
		turn.setBounds(480, 50, 1000, 25);
		turn.setText("turn: " + CURRENT_TURN);
		reset.setVisible(true);
		// turn.setForeground(new Color(255,0,0));

		for (JButton i : gridButton) {
			//frame.add(i);
			i.setFont(new Font("NSimSun", Font.PLAIN, 80));
			i.setText("-");
			i.setVisible(true); // just in case 
		}

		for (int i = 0; i < gridButton.length; i++) {

			if (i < 3) {
				gridButton[i].setBounds(i * 150, 10, 150, 150);
			}

			else if (i < 6) {
				gridButton[i].setBounds((i - 3) * 150, (10 + 150), 150, 150);
			}

			else if (i < 9) {
				gridButton[i].setBounds((i - 6) * 150, (10 + 300), 150, 150);
			}

			//			gridButton[i].addActionListener(e->{
			//				clickButton(startXV);
			//			});
			//			startXV++;
		}
		//startXV = 1;
		act2();
	}

	public static void javaFonts() {
		String fonts[] = GraphicsEnvironment.getLocalGraphicsEnvironment().getAvailableFontFamilyNames();

		for (int i = 0; i < fonts.length; i++) {
			System.out.println(fonts[i]);
		}
	}

	public static void clickButton(int i) {
		if (gridButton[i].getText().equals("-")) {
			/*
			gridButton[i].setText(CURRENT_TURN); 
			grid.setCell(i, CURRENT_TURN);
			System.out.println(grid.toString());
			if(CURRENT_TURN.equals(TURN_X)) {CURRENT_TURN = TURN_O;}
			else if(CURRENT_TURN.equals(TURN_O)) {CURRENT_TURN = TURN_X;}
			turn.setText("turn: "+CURRENT_TURN);
			*/
			if (Game.getWinner(grid).equals("false")) {
				gridButton[i].setText(CURRENT_TURN);
				grid.setCell(i, CURRENT_TURN);
				//System.out.println(grid.toString());

				if (CURRENT_TURN.equals(TURN_X)) {
					CURRENT_TURN = TURN_O;
				} else if (CURRENT_TURN.equals(TURN_O)) {
					CURRENT_TURN = TURN_X;
				}

				turn.setText("turn: " + CURRENT_TURN);
			}
			if (!Game.getWinner(grid).equals("false")) {
				winner.setBounds(480, 150, 100, 100);
				if (Game.getWinner(grid).equals("TIE")) {
					//System.out.println("WIN");
					winner.setText(Game.getWinner(grid));
				} else {
					winner.setText(Game.getWinner(grid) + " WINS.");
				}

				winner.setVisible(true);
			}

		}
	}

	private static void act1(JButton[] arr, int i) {
		arr[i].addActionListener(e -> {
			clickButton(i);
		});
	}

	public static void act2() // turning a 9-line function into two functions that take up 4 (or 6 ig) lines total. i am not sure how to feel about this
	{
		for (int i = 0; i < gridButton.length; i++) {
			act1(gridButton, i);
		}
	}
	/*
		for context:
		testFunc() {
			for(int i = 0 to ex 9) {
				addListener(e -> {clickButton(i);});
			}
		}
		this wouldn't work since i needs to be "final". 
		it would work if you pasted 8 more addListener's with incrementing i's, from 0 to ex 9.
		and yet doing a hacky two function solution like this works
		maybe because act1's int i param is considered final in the first place?
	*/

	public static void reset() {
		for (JButton i : gridButton) {
			i.setVisible(false);
			i.setText("-");
		}

		for (int i = 0; i < grid.grid.length; i++) {
			grid.setCell(i, Integer.toString(i));
		}

		winner.setVisible(false);
		turn.setVisible(false);
		reset.setVisible(false);

		startX.setVisible(true);
		startO.setVisible(true);
		title.setVisible(true);
	}

	// copy pasted functions from ttt.Game *were* here to make the game work.
	// apparently the imported functions might have referenced a STATIC grid in ttt.Game, which would overshadow ttt_gui's grid, i think.
	// the fix for this would be to add a grid param to the imported functions so that the proper grid would be referenced correctly.
	
	public static Runnable run() {
		init();
		return null;
	}
}
