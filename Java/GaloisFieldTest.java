import java.util.Scanner;

public class GaloisFieldTest {
   public static void main(String[] args) {
        try {
            GaloisField a = new GaloisField(5);
            GaloisField b = new GaloisField(1234576);
            GaloisField c = a.add(b);
            System.out.println("a: " + a);
            System.out.println("b: " + b);
            System.out.println("a + b: " + c);
            System.out.println("a == b: " + a.equals(b));
            System.out.println("a != b: " + a.notEquals(b));
            a.addAssign(b);
            System.out.println("a += b, a: " + a);
            System.out.println();

            GaloisField d = new GaloisField(10);
            GaloisField e = new GaloisField(20);
            GaloisField f = d.subtract(e);
            System.out.println("d: " + d);
            System.out.println("e: " + e);
            System.out.println("d - e: " + f);
            System.out.println("d >= e: " + (d.greaterThanOrEqual(e)));
            System.out.println("d <= e: " + (d.lessThanOrEqual(e)));
            d.subtractAssign(e);
            System.out.println("d -= e, d: " + d);
            System.out.println();

            GaloisField g = new GaloisField(308646);
            GaloisField h = new GaloisField(4);
            GaloisField i = g.multiply(h);
            System.out.println("g: " + g);
            System.out.println("h: " + h);
            System.out.println("g * h: " + i);
            System.out.println("g > h: " + (g.greaterThan(h)));
            System.out.println("g < h: " + (g.lessThan(h)));
            g.multiplyAssign(h);
            System.out.println("g *= h, g: " + g);
            System.out.println();

            GaloisField j = new GaloisField(5); 
            GaloisField k = new GaloisField(1234576);
            GaloisField l = j.divide(k);
            System.out.println("j: " + j);
            System.out.println("k: " + k);
            System.out.println("j / k: " + l);
            j.divideAssign(k);
            System.out.println("j /= k, j: " + j);
            System.out.println();

            System.out.print("Enter a number: ");
            Scanner scanner = new Scanner(System.in);
            GaloisField input = new GaloisField(scanner.nextInt());
            System.out.println("You entered: " + input);

        } catch (Exception e) {
            System.err.println("Exception: " + e.getMessage());
        }
    }
}
