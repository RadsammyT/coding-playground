package rad;

/**
 * 
 * @author RadsammyT
 * @note not actually a timer
 * just gets system time in each method, so its
 * not actually 'ticking' while program is running
 * which now that I think of it its kinda efficient.
 */

public class Timer {

	long startTime = 0;
	long endTime = 0;
	long startTimeNano = 0;
	long endTimeNano = 0;
	public Timer() {

	}
	/**
	 * Sets both startTime and Nano.
	 */
	public void startTimer() {
		this.startTime = System.currentTimeMillis(); // sets startTimer to current time in milliseconds
		this.startTimeNano = System.nanoTime();
	}
	/**
	 * @return startTime in milliseconds
	 */
	public long getStart() {
		if (startTime != 0) {
			return startTime;
		}
		return -1;
	}
	/**
	 * Sets both endTime and Nano.
	 */
	public void endTimer() {
		this.endTime = System.currentTimeMillis(); // sets endTimer to current time in milliseconds
		this.endTimeNano = System.nanoTime();
	}
	/**
	 * @return endTime in milliseconds
	 */
	public long getEnd() {
		if (endTime != 0) {
			return endTime;
		}
		return -1;
	}
	/**
	 * 
	 * @return elapsed time (calculated with endTime - startTime).
	 * -1 if either start or endTimes are 0/unset.
	 */
	public long getElapse() {

		if (this.endTime == 0 || this.startTime == 0) {
			return -1;
		}
		return this.endTime - this.startTime; // returns the difference


	}
	/**
	 * 
	 * @return the difference as a double. while also divide it by 1000 because its a double.
	 * so why the fuck not. will also return -1 if either start or endTimes are 0/Unset.
	 */

	public double getElapseDouble() {

		if (this.endTime == 0 || this.startTime == 0) {
			return -1;
		}

		return (double)(this.endTime - this.startTime) / 1000;
	}
	/**
	 * 
	 * @return the difference in nanoseconds.
	 */
	public long getElapseNano() {
		return (this.endTimeNano - this.startTimeNano);
	}
	/**
	 * ahh, the classic and overused override in every fucking java class.
	 * basically returns starting time, ending time, along with elapsed time.
	 */
	public String toString() {
		return "startTime: " + this.startTime + " |  endTime: " + this.endTime + " | ELAPSE: " + (float)(this.getElapse() / 1000f) + " seconds";
		// returns the starting time, ending time and the elapsed time
	}

}