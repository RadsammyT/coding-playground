package rad;

public class fibbTime {
    
    public static void run()
    {
        int numPrimes = 0;
        for(int i =2; i<=250001; i++)
        {
            numPrimes+=isPrime(i);
        }
    }//System.out.println(rad.Collatz.run((long) Long.MAX_VALUE));

    public static int isPrime(int n) {
        for(int i=2; i<=n/2;i++)
            if (n % i == 0)
                return 0;
        return 1;
    }
}
