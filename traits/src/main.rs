use std::collections::hash_map::HashMap;

trait MessageConsumer {
    fn topic(&self) -> &str;
    fn consume(&self);
}

struct FooEventConsumer {
    text: String,
}

struct BarEventConsumer {
    name: String,
    age: u8,
}

impl MessageConsumer for FooEventConsumer {
    fn topic(&self) -> &str {
        "foo"
    }

    fn consume(&self) {
        println!("My text is {}", self.text);
    }
}

impl MessageConsumer for BarEventConsumer {
    fn topic(&self) -> &str {
        "bar"
    }

    fn consume(&self) {
        println!("My name is {} and age is {}", self.name, self.age);
    }
}

struct Consumers {
    consumers: HashMap<String, Box<dyn MessageConsumer>>,
}

impl Consumers {
    fn new() -> Consumers {
        Consumers {
            consumers: HashMap::new(),
        }
    }

    fn insert(&mut self, consumer: Box<dyn MessageConsumer>) {
        self.consumers.insert(consumer.topic().to_string(), consumer);
    }
}

fn main() {
    let mut consumers = Consumers::new();
    consumers.insert(Box::new(
        FooEventConsumer {
            text: String::from("bar"),
        }
    ));

    consumers.insert(Box::new(
        BarEventConsumer {
            name: String::from("John"),
            age: 12,
        }
    ));

    for (k, v) in consumers.consumers  {
        println!("key: {}", k);
        v.consume();
    }

    // let mut h: HashMap<String, Box<dyn MessageConsumer>> = HashMap::new();

    // let consumer1 = FooEventConsumer {
    //     text: String::from("bar"),
    // };
    // h.insert(String::from("c1"), Box::new(consumer1));

    // let consumer2 = BarEventConsumer {
    //     name: String::from("John"),
    //     age: 12,
    // };
    // h.insert(String::from("c2"), Box::new(consumer2));

    // for (k, v) in &h {
    //     println!("key: {}", k);
    //     v.consume();
    // }
}
