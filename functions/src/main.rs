fn main() {
    println!("Hello, world!");

    another_function(32, 12);

    // Expressions and statements
    // This is a statement
    let foo = 1 + 10 /* This is the expression */;

    // This is a block that returns an expression
    let b = {
        let x = 3;
        x + 10 // Expressions don't end with ;
    };

    println!("b={}", b);
    println!("five: {}", five());
    plus_one_expression(10);
    plus_one_return(10);
}

fn another_function(x: i32, y: i32) {
    println!("Value of x is {}", x);
    println!("Value of y is {}", y);
}

fn five() -> u8 {
    5 // it's an expression so no need to ; and no need to return
}


fn plus_one_expression(x: i32) -> i32 {
    x + 1
}

fn plus_one_return(x: i32) -> i32 {
    return x + 1;
}


