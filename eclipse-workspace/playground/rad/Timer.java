package rad;

/**
 * 
 * @author RadsammyT
 * note: not actually a timer
 * just gets system time in milliseconds, so its
 * not actually 'ticking' while program is running
 * which now that I think of it its kinda efficient.
 */

public class Timer {

	long startTime = 0;
	long endTime = 0;
	public Timer(){
		
	}

	public void startTimer()
	{
		this.startTime = System.currentTimeMillis(); // sets startTimer to current time in milliseconds
	}
	
	public long getStart()
	{
		if(startTime != 0)
		{
			return startTime;
		}
		return -1;
	}
	
	public void endTimer()
	{
		this.endTime = System.currentTimeMillis(); // sets endTimer to current time in milliseconds
	}
	
	public long getEnd()
	{
		if(endTime != 0)
		{
			return endTime;
		}
		return -1;
	}
	
	public long getElapse() // returns the elapsed time. if either 
							// start or endTimes are 0/Unset, return -1
	{
		
		if(this.endTime == 0 || this.startTime == 0 )
		{
			return -1;
		}
		return this.endTime - this.startTime; // returns the difference
		
		
	}
	public double getElapseDouble() // returns the difference as a double
									// while also dividing it by 1000 because
									// its a double so why the fuck not.
									// if either start or endTimes are 0/Unset, return -1
	{
		
		if(this.endTime == 0 || this.startTime == 0 )
		{
			return -1;
		}
		
		return (double) (this.endTime - this.startTime) / 1000;	
	}
	
	public String toString()
	{
		return "startTime: " + this.startTime + " |  endTime: " + this.endTime + " | ELAPSE: " + (float) (this.getElapse() / 1000f) + " seconds";
		// returns the starting time, ending time and the elapsed time
	}
	
}
