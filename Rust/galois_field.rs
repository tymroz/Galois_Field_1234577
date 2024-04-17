use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy)]
struct GaloisField {
    value: i32,
}

impl GaloisField {
    const CHARACTERISTIC: i32 = 1234577;

    fn from_value(val: i32) -> Self {
        Self {
            value: (val % Self::CHARACTERISTIC + Self::CHARACTERISTIC) % Self::CHARACTERISTIC,
        }
    }

    fn mod_value(&self, a: i32, b: i32) -> i32 {
        (a % b + b) % b
    }

    fn extended_euclidean(&self, a: i32, b: i32) -> i32 {
        let mut old_r = a;
        let mut r = b;
        let mut old_s = 1;
        let mut s = 0;
        let mut old_t = 0;
        let mut t = 1;

        while r != 0 {
            let quotient = old_r / r;

            let temp = r;
            r = old_r - quotient * r;
            old_r = temp;

            let temp = s;
            s = old_s - quotient * s;
            old_s = temp;

            let temp = t;
            t = old_t - quotient * t;
            old_t = temp;
        }

        self.mod_value(old_s, Self::CHARACTERISTIC)
    }
}

impl std::ops::Add for GaloisField {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::from_value(self.value + other.value)
    }
}

impl std::ops::Sub for GaloisField {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::from_value(self.value - other.value)
    }
}

impl std::ops::Mul for GaloisField {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::from_value(self.value * other.value)
    }
}

impl std::ops::Div for GaloisField {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        if other.value == 0 {
            panic!("Division by zero");
        }
        let inverse = self.extended_euclidean(other.value, Self::CHARACTERISTIC);
        Self::from_value(self.value * inverse)
    }
}

impl std::ops::AddAssign for GaloisField {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl std::ops::SubAssign for GaloisField {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl std::ops::MulAssign for GaloisField {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl std::ops::DivAssign for GaloisField {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

impl std::cmp::PartialEq for GaloisField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl std::cmp::PartialOrd for GaloisField {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.value.cmp(&other.value))
    }
}

impl fmt::Display for GaloisField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Debug for GaloisField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl FromStr for GaloisField {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.trim()
            .parse::<i32>()
            .map(|val| GaloisField
            ::from_value(val))
    }
}

fn main() {
    let mut a = GaloisField::from_value(5);
    let b = GaloisField::from_value(1234576);
    let c = a + b;
    println!("a: {}", a);
    println!("b: {}", b);
    println!("a + b: {}", c);
    println!("a == b: {}", a == b);
    println!("a != b: {}", a != b);
    a += b;
    println!("a += b, a: {}", a);
    println!();

    let mut d = GaloisField::from_value(10);
    let e = GaloisField::from_value(20);
    let f = d - e;
    println!("d: {}", d);
    println!("e: {}", e);
    println!("d - e: {}", f);
    println!("d <= e: {}", d <= e);
    println!("d >= e: {}", d >= e);
    d -= e;
    println!("d -= e, d: {}", d);
    println!();

    let mut g = GaloisField::from_value(308646);
    let h = GaloisField::from_value(4);
    let i = g * h;
    println!("g: {}", g);
    println!("h: {}", h);
    println!("g * h: {}", i);
    println!("g < h: {}", g < h);
    println!("g > h: {}", g > h);
    g *= h;
    println!("g *= h, g: {}", g);
    println!();

    let mut j = GaloisField::from_value(5);
    let k = GaloisField::from_value(1234576);
    let l = j / k;
    println!("j: {}", j);
    println!("k: {}", k);
    println!("j / k: {}", l);
    j /= k;
    println!("j /= k, j: {}", j);
    println!();

    println!("Enter a number: ");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let m: GaloisField = input.trim().parse().expect("Invalid number");
    println!("You entered: {}", m);
}