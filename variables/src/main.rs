fn main() {
    let x = 1____5__u16;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "     ";
    println!("{}", spaces);
    let spaces = spaces.len();
    println!("{}", spaces);

    let guess: u32 = "42".parse().expect("Not a number!");

    let y = 2.0;
    let y: f32 = 3.0;

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    println!("{} {} {} {} {}", sum, difference, product, quotient, remainder);

    let t = true;
    let f: bool = false;

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("The value of y is: {}", b);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("{} {}", first, second);
}
