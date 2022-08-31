// loop - infinity (break;) or end programm.
// for - more fast and safety (more useful
// while - just for allot iterations

fn main() {
    // let mut x = 5;
    // println!("Hello, world! {}", x);
    // x = 6;
    // println!("Hello, world! {}", x);

    // inicialization
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let v1 = tuple.0;
    println!("tuple and first argument of it (v1) {:?}, {}", tuple, v1);

    let a = [1, 2 ,3 ,4 ,5]; // [i32; 5] automaticly added
    let first = a[0];
    let last = a[4];
    let y = {
        let last = 3;
        last - 1
    };
    
    println!("first and last elements of array a and y {}, {}, {}", first, last, y);
    let b: [u8; 5] = [1, 2, 3, 4, 5];
    let months = ["Jan", "Feb", "Mar", "Apr"];
    println!("arrays a, b and months: {:?}, {:?}, {:?}", a, b, months);

    let r = five();
    println!("inicialization by function return {}", r);

    // visability 
    println!("MAX_POINTS just in main visability! {}", MAX_POINTS);
    const MAX_POINTS: u32 = 100_000;
    print();

    whiles(3, false);

    //ownership
    let mut s = String::from("hello");
    let s1 = s.clone();
    println!("{}", s);
    println!("{}", s1);

    s.push_str(", world!");
    println!("{}", s);
    println!("{}", s1);

    takes_ownership(s1);
    // println!("{}", s1); // don't work because s1 borowed for fn takes_ownership

    let (s2, len) = calculate_length(&mut give());
    let param = (&s2, &len);
    println!("{}, {}, {:?}, {:?}", s2, len, calculate_length(&mut give()), param);

    // can not use &mut for two or more different param 
    // let mut s11 = String::from("String");
    // let s12 = &mut s11;
    // let s13 = &mut s11;
    // println!("{}", s12);
    // println!("{}", s13);

    // but might to use it in different visabilities
    let mut s11 = String::from("String");
    {
        let s12 = &mut s11;
        println!("{}", s12);
    }
    {
        let s13 = &mut s11;
        println!("{}", s13);
    }
    // в одной области видимости много неизменяемых ссылок и только одна неизменяемая
    // все ссылки должны быть действительны

    let mut string = String::from("hello world");
    let index = {
        last_byte_of_first_word(&string) + 1
    };
    println!("{}, {}", index, &string[..index + 2]);
    string.clear();

    let string = String::from("hello world");
    let hello = &string[0..5];
    let world = &string[6..11];
    let hey = &string[..8];
    let heyy = &string[4..];
    let heyyy = &string[..];
    println!("{}, {}, {}, {}, {}", hello, world, hey, heyy, heyyy);

    let my_string = String::from("Hello world");
    let word = first_word(&my_string);
    println!("{}", word);
    let my_string_literal = "Hello word";
    let word = first_word(&my_string_literal);
    println!("{}", word);
    let word = first_word(my_string_literal);
    println!("{}", word);

    let user1 = User {
        username: String::from("potemkinViktor"),
        email: String::from("vapotemkin@gmail.com"),
        active: true,
        sign_in_count: 1,
    };

    println!(
        "[{}; {}; {}; {}]",
        user1.username,
        user1.email,
        user1.sign_in_count,
        user1.active
    );

    let user2 = add_new_user(String::from("vapotemkin@gmail.com"), String::from("potemkinViktor"));
    println!("{}, {}, {}, {}", user2.email, user2.username, user2.active, user2.sign_in_count);

    let user3 = User {
        username: String::from("potemkinViktor"),
        email: String::from("vapotemkin@gmail.com"),
        ..user2 // adding with last param from user2
    };

    println!("{}, {}, {}, {}", user3.email, user3.username, user3.active, user3.sign_in_count);
    
}

fn print() {
    println!("MAX_POINTS just out of main visability! {}", MAX_POINTS);
}

const MAX_POINTS: u32 = 200_000;

fn five() -> u8 {
    5
}

fn whiles(x: u32, y: bool) -> u32 {
    let counter = x;
    let condition = y;
    let mut number = 0;
    let a = [10, 20, 30 , 40 ,50];

    if counter < 4 {
        println!("true");
    } else {
        println!("false");
    }

    let counter = if condition {
        println!("less than 4");
        3
    } else {
        println!("over 4");
        5
    };

    loop {
        println!("again! {}", number);

        number += 1;
        if number == 4 { break; }
    }

    while number != 0 {
        println!("one more time! {}", number);
        number -= 1;
    }

    for element in a.iter() {
        println!("the value is {}", element);
    }

    for z in (1..8).rev() {
        println!("{}", z);
    }
    println!("    STOPED");

    counter
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn calculate_length(s: &mut String) -> (String, usize) {
    s.push_str(" string");
    let length = s.len();

    (s.to_string(), length)
}

fn give() -> String {
    let word = String::from("Hello world");

    word
}

fn last_byte_of_first_word(s: &String) -> usize{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i - 1;
        }
    }

    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn add_new_user(email: String, username: String) -> User {
    User { 
        email,
        username,
        sign_in_count: 1,
        active: true 
    }
}

