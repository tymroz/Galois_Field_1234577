import java.util.Scanner;

class GaloisField {
    private static final int characteristic = 1234577;

    private int value;

    private int mod(int a, int b) {
        return (a % b + b) % b;
    }

    private int extendedEuclidean(int a, int b) {
        int old_r = a, r = b;
        int old_s = 1, s = 0;
        int old_t = 0, t = 1;

        while (r != 0) {
            int quotient = old_r / r;

            int temp = r;
            r = old_r - quotient * r;
            old_r = temp;

            temp = s;
            s = old_s - quotient * s;
            old_s = temp;

            temp = t;
            t = old_t - quotient * t;
            old_t = temp;
        }

        return mod(old_s, characteristic);
    }

    public GaloisField() {
        this.value = 0;
    }

    public GaloisField(int val) {
        this.value = mod(val, characteristic);
    }

    public int getValue() {
        return value;
    }

    public GaloisField add(GaloisField other) {
        return new GaloisField(mod(value + other.value, characteristic));
    }

    public GaloisField subtract(GaloisField other) {
        return new GaloisField(mod(value - other.value, characteristic));
    }

    public GaloisField multiply(GaloisField other) {
        return new GaloisField(mod(value * other.value, characteristic));
    }

    public GaloisField divide(GaloisField other) {
        if (other.value == 0) {
            throw new IllegalArgumentException("Division by zero");
        }
        int inverse = extendedEuclidean(other.value, characteristic);
        return new GaloisField(mod(value * inverse, characteristic));
    }

    public boolean equals(GaloisField other) {
        return value == other.value;
    }

    public boolean notEquals(GaloisField other) {
        return !equals(other);
    }

    public boolean lessThan(GaloisField other) {
        return value < other.value;
    }

    public boolean lessThanOrEqual(GaloisField other) {
        return value <= other.value;
    }

    public boolean greaterThan(GaloisField other) {
        return value > other.value;
    }

    public boolean greaterThanOrEqual(GaloisField other) {
        return value >= other.value;
    }

    public GaloisField addAssign(GaloisField other) {
        this.value = mod(this.value + other.value, characteristic);
        return this;
    }

    public GaloisField subtractAssign(GaloisField other) {
        this.value = mod(this.value - other.value, characteristic);
        return this;
    }

    public GaloisField multiplyAssign(GaloisField other) {
        this.value = mod(this.value * other.value, characteristic);
        return this;
    }

    public GaloisField divideAssign(GaloisField other) {
        if (other.value == 0) {
            throw new IllegalArgumentException("Division by zero");
        }
        int inverse = extendedEuclidean(other.value, characteristic);
        this.value = mod(this.value * inverse, characteristic);
        return this;
    }

    @Override
    public String toString() {
        return String.valueOf(value);
    }
}