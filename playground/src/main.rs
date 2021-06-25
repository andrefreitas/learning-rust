fn main() {
    // Floating point numbers
    let num1: f32 = 0.223;
    let num2: f64 = 0.678;
    println!("num1={num1} num2={num2}", num1=num1, num2=num2);

    // Numeric operations
    let sum = 5 + 10;
    let subtraction = 20 - 3;
    let mult = 2 * 30;
    let quotient = 7 / 3;
    let remainder = 4 % 3;
    println!("{} {} {} {} {}", sum, subtraction, mult, quotient, remainder);

    // Boolean - 1 byte size
    let t = true;
    let f1: bool = false;

    // Chars - 4 bytes in size
    let c: char = 'x';
    let emoji = '🚀';
    println!("{} {}", c, emoji);

    // Tuples - fixed length
    let tup: (u32, f32, u8) = (35, 23.0, 1);
    let (a,b,c) = tup;

    println!("a: {} == {}", a, tup.0);

    // Arrays - fixed length
    // Array index panic instead of accessing invalid memory
    let some_a = [1,2,3,4,5];
    let some_b: [i32; 2] = [1,2];
    let repeated_numbers = [0; 5];
    println!("arr: {}, {}", repeated_numbers[0], repeated_numbers[2]);
}
