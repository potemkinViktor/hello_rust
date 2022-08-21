// loop - infinity (break;) or end programm
// for - more fast and safety (more useful)
// while - 

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
}

fn print() {
    println!("MAX_POINTS just out of main visability! {}", MAX_POINTS);
}

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
    println!("    STOPED");

    counter
}

const MAX_POINTS: u32 = 200_000;