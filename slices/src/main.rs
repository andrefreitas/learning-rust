fn print_hello(name: &str){
    println!("Hello {}", name);
}

fn main() {
    let phrase = "Hello, world!";
    let hello = &phrase[0..5];
    let world = &phrase[5..];
    let slice = &phrase[..];
    println!("Slice: {}", slice);
    println!("{}, {}", hello, world);

    let name = String::from("Andre");
    print_hello(name.as_str());
    print_hello(&name[..]);

    let nums = [1,2,3,10,5];
    let slice = &nums[1..4];
    assert_eq!(slice, &[2,3,10]);
}
