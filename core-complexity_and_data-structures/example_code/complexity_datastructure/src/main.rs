use std::ops::Add;

struct Point<T> {
    x: T,
    y: T,
}

struct User {
    username: String,
    email: String,
    signin_count: i64,
    active: bool,
    address: Address,
}

trait Greet {
    fn x(&self) -> f64;

    fn greet(&self) {
        println!("Hello from the default {}", self.x());
    }
}

impl User {
    fn greet(&self) {
        println!("{} ({})", self.username, self.email);
    }

    fn getCity(&self) -> &String {
        &self.address.city
    }

    fn biggest<'a>(&'a self, u: &'a User) -> &'a User {
        if u.username.len() > self.username.len() {
            return u;
        }
        self
    }
}

struct Address {
    number: i32,
    street: String,
    zip: String,
    city: String,
    country: String,
}

enum UserIdentifiant {
    ID(i32),
    Email(String),
}

fn main() {
    println!("Hello, world!");

    //let mut test;
    {
        let x = String::from("hello");
        let y = String::from("world");
        //test = biggest(&x, &y);
        let result: Option<String> = uncertainfunction(4);
        //let getIt = result.unwrap();
        /*
        match result {
            Some(name) => getIt = name,
            None => println!("Erreur"),
        }*/
        let d = divide(26.0, 2.0);
        match d {
            Ok(value) => println!("{}", value), //do something with valeu;
            Err(error) => println!("Error: {}", error),
        }
    }

    let logResult = login(UserIdentifiant::ID(123));

    match logResult {
        UserIdentifiant::Email(value) => print!("{}", value),
        UserIdentifiant::ID(value) => print!("{}", value),
    }
    //println!("{}", test);

    /*Struct */

    let mut user1 = User {
        active: true,
        email: String::from("cziane@dsti.institute"),
        username: String::from("cziane"),
        address: Address {
            number: 3,
            street: String::from("A random street"),
            zip: String::from("91000"),
            city: String::from("A random city"),
            country: String::from("France"),
        },
        signin_count: 0,
    };

    let mut user2 = User {
        active: true,
        email: String::from("johnny@dsti.institute"),
        username: String::from("joe"),
        signin_count: 0,
        //..user1
        address: user1.address,
    };

    //Can modify it cause mutable
    user1.username = String::from("cziane9");
    //Not possible due to owner ship is on user2.address and make user1 unusable
    //user1.address.city = String::from("corbeil");
    //println!("{}", user1.address.city);

    println!("{}", user2.address.city);

    user2.greet();

    {
        let mut user3 = User {
            active: true,
            email: String::from("johnny@dsti.institute"),
            username: String::from("joe"),
            signin_count: 0,
            //..user1
            address: Address {
                number: 3,
                street: String::from("A random street"),
                zip: String::from("91000"),
                city: String::from("A random city"),
                country: String::from("France"),
            },
        };

        user3.biggest(user1)
    }
}

fn biggest<'a>(x: &'a String, y: &'a String) -> &'a String {
    let r = "test";
    y
}

fn uncertainfunction(x: u8) -> Option<String> {
    if x > 5 {
        return Some(String::from("Hello"));
    }
    None
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn login(u: UserIdentifiant) -> UserIdentifiant {
    match u {
        UserIdentifiant::Email(v) => {
            return UserIdentifiant::Email(String::from("clement.ziane@dsti.institute"));
        }
        UserIdentifiant::ID(v) => return UserIdentifiant::ID(1),
    }
}

fn lifetimeOtherExample() {
    let a = String::from("test");
    let c: &String;
    {
        let b = String::from("another");
        c = anyfunction(&a, &b);
    }
    println!("{}", c);
}

fn anyfunction<'a>(a: &'a String, b: &'a String) -> &'a String {
    if a.len() > b.len() {
        return a;
    }
    b
}

fn trait_example() {
    let my_arr = [1, 2, 3, 4];
    let my_vec = vec![1, 2, 3, 4];

    for v in my_arr {
        println!("{}", v);
    }

    let filtered_iterator: Vec<&i32> = my_vec.iter().filter(|v| **v % 2 == 0).collect();

    let vec_example = vec![1, 2, 3, 4];

    let modified: Vec<i32> = vec_example.into_iter().map(|x| x * 2).collect();

    println!("{:?}", modified);
}

fn generic_example() {
    let p = Point { x: 1u8, y: 3u8 };
    let pf = Point { x: 1.6, y: 2.5 };
}
