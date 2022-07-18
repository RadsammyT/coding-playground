package leetcode;

import java.util.*;


public class leetcode {
    // https://leetcode.com/problems/defanging-an-ip-address/
    public static String defangIPAddr(String address) {
        System.out.println(address.replaceAll("[.]", "[.]"));
        return address.replaceAll("[.]", "[.]");
    }
    //https://leetcode.com/problems/two-sum/
    public static int[] twoSum(int[] nums, int target) {
        for (int i = 0; i < nums.length; i++) {
            for (int j = 0; j < nums.length; j++) {
                if (nums[i] + nums[j] == target && i != j) {
                    return new int[] { i, j };
                }
            }
        }
        return new int[] {-1, -1};
    }
    //https://leetcode.com/problems/build-array-from-permutation/
    public static int[] BuildArrayFromPermutation(int[] nums) {
        int[] out = new int[nums.length];
        for (int i = 0; i < nums.length; i++) {
            out[i] = nums[nums[i]];
        }
        // System.out.println(Arrays.toString(nums));
        // System.out.println(Arrays.toString(out) + "\n");
        return out;
    }
    //https://leetcode.com/problems/final-value-of-variable-after-performing-operations/
    public static int finalValueAfterOperations(String[] ops) {
        int x = 0;
        for (String i : ops) {
            if (i.contains("--")) {
                x--;
            }
            if (i.contains("++")) {
                x++;
            }
        }

        return x;
    }
    //https://leetcode.com/problems/running-sum-of-1d-array/
    public static int[] runningSum(int[] nums) {
        int sum = 0;
        int[] out = new int[nums.length];
        for (int i = 0; i < nums.length; i++) {
            sum += nums[i];
            out[i] = sum;
        }

        return out;
    }

    //https://leetcode.com/problems/richest-customer-wealth/
    public static int maximumWealth(int[][] accounts) {
        int highest = 0;
        int sum = 0;
        for (int i = 0; i < accounts.length; i++) {
            for (int j : accounts[i]) {
                sum += j;
            }
            if (sum > highest) {
                highest = sum;
            }
            sum = 0;
        }

        return highest;
    }
    //https://leetcode.com/problems/maximum-number-of-words-found-in-sentences/
    public static int mostWordsFound(String[] sentences) {
        short max = 0;
        short current = 1;
        
        for (String inst : sentences) {
            for (int i = 0; i < inst.length(); i++) {
                if (Character.toString(inst.charAt(i)).equals(" ")) {
                    // System.out.println("CHAR:" + inst.charAt(i) + " INDEX:" + i + " SENTENCE:" + inst);
                    current++;
                }
            }
            // System.out.println("TOTAL: " + current);
            if (current >= max) {
                max = current;
            }
            current = 1;
        }
        
        return max;
    }
}