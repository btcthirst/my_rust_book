fn main() {
    
    println!("hello common programming concepts!");

    variables_use();
    operations();
    bool_type();
    char_type();
    compound_types();
    functions();
    control_flow();
}

fn variables_use() {
    let x: i32 = 5;
    println!("The value of immutable x is: {x}");
    {
        let x: i32 = x + 51;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of the same immutable x is: {x}");

    let x: i32 = 51;
    println!("The value of else immutable x is: {x} \n");
    
    let mut y = 6;
    println!("The value of mut y is: {y}");

    y = y+3;
    println!("The value of mut y is: {y}");

    const THREE_HOURS_IN_SECONDS: u64 = 60*60*3;
    println!("The value of const THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");
    
    let text = "456";
    let length = text.len();
    let number: u64 = text.parse().expect("not a number");

    println!("\n &str - text: {text}, length of text: {length}, parsed number: {number}")
}

fn operations() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
    println!("\n like all results {sum}, {difference}, {product}, {quotient}, {truncated}, {remainder}");
}

fn bool_type() {
    let t = true;
    let f = false;
    print!("\n t: {t}, f: {f} \n");
}

fn char_type() {
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';
    println!("\n chars c: {c}, z: {z}, heart_eyed_cat: {heart_eyed_cat}")
}

fn compound_types() {
    // Кортеж? tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!(" \n {}, {}, {}",tup.0, tup.1, tup.2);
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup; // деструктуризація

    println!("The value of x, y & z is: {y}, {x}, {z} or tup.1: {}",tup.1);

    // Масив? array
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("{}, {:?}",a[0],months);
}

fn functions() {
    println!("sum: {}", sum(5, 6));
    println!("like new funk res : {:?}", new_fn());
}

fn sum(x: u64,y: u64) -> u64 {
    x+y
}

fn new_fn() -> i32 {
    let text = {
        5
    };
    text
}

fn control_flow() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was three");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    let mut counter = 0;
    loop {
        println!("in loop again!");
        if counter > 5 {
            break;
        } else {
            counter += 1;
        }
    }

    counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}