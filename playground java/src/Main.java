import java.math.*;
import java.util.Arrays;
import rad.NegativeNumberException;
import rad.Timer;
import javax.swing.*;
import jFrame.ttt_gui;
import leetcode.leetcode;
public class Main {
	public static void main(String args[]) throws InterruptedException, NegativeNumberException, Select {
		// Select.run();
		System.out.println(Arrays.toString(
			leetcode.runningSum(new int[] {1,1,1,1,1})));
	}
}
