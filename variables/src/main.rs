use std::io;
fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is {}", x);

    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;

    let floored = 2 / 3;

    let remainder = 43 % 5;
    println!("{}", sum);
    println!("{}", difference);
    println!("{}", product);
    println!("{}", quotient);
    println!("{}", floored);
    println!("{}", remainder);

    let t = true;

    let f: bool = false;
    println!("{}", t);
    println!("{}", f);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    let something: (i32, f64, u8) = (500, 6.4, 1);
    println!(
        "something.0 is  {}, something.1 is  {}, something.2 is {}",
        something.0, something.1, something.2
    );

    let array = [1, 2, 3, 4, 5]; //配列は固定長

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = array[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );

    another_function(5, 'h');
    let five_plus = plus_one(5);

    println!("{}", five_plus);

    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("contdition was false");
    }

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

    let condition_number = if condition { 5 } else { 6 };
    println!("The value of number is:{}", condition_number);

    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
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
    println!("End count = {}", count);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

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
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn another_function(value: i32, unit_label: char) {
    println!("The measurement is : {}{}", value, unit_label);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
