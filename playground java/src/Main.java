import java.math.*;
import java.util.Arrays;
import rad.NegativeNumberException;
import rad.Timer;
import javax.swing.*;
import jFrame.ttt_gui;
import leetcode.leetcode;
import java.io.*;
public class Main {
	public static void main(String args[]) throws InterruptedException, NegativeNumberException, Select {
		// Select.run();
		System.out.println(Arrays.toString(leetcode.shuffle(new int[] { 2, 5, 1, 3, 4, 7 }, 3)));
	}
}
