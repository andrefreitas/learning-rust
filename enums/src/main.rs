#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}


#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String
}

#[derive(Debug)]
enum IpAddress {
    V4(String),
    V6(String)
}

impl IpAddress {
    fn call(&self){
        println!("{:?}", self)
    }
}


fn main() {
    let home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    let loopback = IpAddr{
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };

    let ip1 = IpAddress::V4(String::from("127.0.0.1"));
    let ip2 = IpAddress::V6(String::from("::1"));

    let result = match ip1 {
        IpAddress::V4(ref address) => {
            println!("It's a v4 address: {}", address);
            "v4"
        }
        IpAddress::V6(_) => "v6",
    };
    println!("Result: {}", result);

    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);

    println!("home: {:?}", ip1);
    println!("loopback: {:?}", ip2);
    ip2.call();

    let num1: Option<u32> = Some(1);
    assert_eq!(num1.is_some(), true);
    assert_eq!(num1.is_none(), false);
    assert_eq!(num1.expect("Doesn't contain a value"), 1);
    assert_eq!(num1.unwrap(), 1);
    assert_eq!(None.unwrap_or("bike"), "bike");
    assert_eq!(Some("sss").map(|s| s.len()), Some(3));

    match num1 {
        Some(v) => println!("Some: {}", v),
        None => println!("None!")
    }

    match num1 {
        Some(v) => println!("Some: {}", v),
        _ => ()
    }

    if let Some(v) = num1 {
        println!("Value is {}", v);
    }
}
