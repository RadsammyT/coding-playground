package ttt;

public class Grid extends Game{

	public String[] grid = {"0","1","2","3","4","5","6","7","8",};
	
	public String getCell(int i)
	{
		return grid[i];
	}
	
	public void setCell(int i, String as)
	{
		grid[i] = as;
	}
	
	public String getAllCells()
	{
		/*
		return   grid[0] +   grid[1] +   grid[2]
				+grid[3] +   grid[4] +   grid[5] 
				+grid[6] +   grid[7] +   grid[8]  ;
		*/
		String result = null;
		for(String g: grid)
		{
			result = result + g;
		}
		return result;
	}
	
	public String toString()
	{
		return   grid[0] + " " + grid[1] + " " + grid[2] + "\n"
				+grid[3] + " " + grid[4] + " " + grid[5] + "\n"
				+grid[6] + " " + grid[7] + " " + grid[8]  ;
		
	}
}
