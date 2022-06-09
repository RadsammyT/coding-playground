package rad;
import java.util.ArrayList;
import java.util.List;
public class Fibbonachi {

	public static List < Integer > run(int length) {
		List < Integer > test = new ArrayList < Integer > ();

		test.add(0);
		test.add(1);

		for (int i = 0; i < length; i++) {
			test.add(test.get(i) + test.get(i + 1));
		}

		return test;
	}
	/*
	public static String listToString(List<Integer> test)
	{
		String result = "[";
		for(int i: test)
		{
			result = result + " " +Integer.toString(i);
		}
		
		result = result + " ]";
		return result;
	}
	*/

}