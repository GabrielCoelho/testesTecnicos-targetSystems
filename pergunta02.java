public class pergunta02 {

    public static void main(String[] args) {
        int informedNumber = 13;

        if (isFibonacci(informedNumber)) {
            System.out.println(informedNumber + " is present in Fibonacci");
        } else {
            System.out.println(informedNumber + " isn't present in Fibonacci");
        }
    }

    public static boolean isFibonacci(int w) {
        long x1 = 5L * w * w + 4;
        long x2 = 5L * w * w - 4;

        long x1Sqrt = (long) Math.sqrt(x1);
        long x2Sqrt = (long) Math.sqrt(x2);

        return (x1Sqrt * x1Sqrt == x1) || (x2Sqrt * x2Sqrt == x2);
    }
}
