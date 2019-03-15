fn main() {
    let shadowed = 5;
    let shadowed = shadowed + 1;
    let shadowed = shadowed * 2;

    println!("The value of shadowed is: {}", shadowed);

    let spaces = "   ";
    let spaces = spaces.len();

    println!("The value of spaces is: {}", spaces);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is: {}", y);
}
