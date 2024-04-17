mod galois_field;

use galois_field::GaloisField;
use std::io::{self};

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
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let m: GaloisField = input.trim().parse().expect("Invalid number");
    println!("You entered: {}", m);
}
