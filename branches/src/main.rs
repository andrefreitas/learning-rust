fn main() {
    let number = 2;

    let old = false;
    let age = if old { 70 } else { 30 };
    println!("Age: {}", age);

    if number > 3 {
        println!("Bigger");
    } else if number % 2 == 0{
        println!("Divisible by two!");
    } else {
        println!("bahhh");
    }

    loop {
        println!("hellloooo");
    }
}
