package leetcode;

import java.util.*;


public class leetcode {
    // https://leetcode.com/problems/defanging-an-ip-address/
    public static String defangIPAddr(String address) {
        // System.out.println(address.replaceAll("\\.", "[.]"));
        return address.replaceAll("\\.", "[.]");
    }
}
