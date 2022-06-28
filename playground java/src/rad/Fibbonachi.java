package rad;
import java.util.ArrayList;
import java.util.List;
public class Fibbonachi {

	public static List < Integer > run(int length) {
		List < Integer > out = new ArrayList < Integer > ();

		out.add(0);
		out.add(1);

		for (int i = 0; i < length; i++) {
			out.add(out.get(i) + out.get(i + 1));
		}

		return out;
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