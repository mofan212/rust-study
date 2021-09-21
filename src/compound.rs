fn main() {
    println!("Hello, world!");
    another_function(5, 6.4); // argument

    // expression
    let x = 5;
    println!("The value of x is {}", x);
    let y = {
        let x = 3;
        x + 3
    };
    println!("The value of y is {}", y);

    println!("The return of function is {}", plus_five(6));

    if_number_gt5(6);

    use_loop();

    use_while();

    use_for();
}

fn another_function(x: i32, y: f64) {
    println!("Another Function!");
    println!("The value of x is {}, y is {}", x, y); // parameter
}

fn plus_five(x: i32) -> i32 {
    5 + x
}

fn if_number_gt5(x: i32) {
    if x > 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is {}", number);
}

fn use_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2
        }
    };

    println!("The result is {}", result);
}

fn use_while() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number = number - 1
    }

    println!("LIFTOFF!")
}

fn use_for() {
    let array = [10, 20, 30, 40, 50];

    for element in array.iter() {
        println!("The value is {}", element)
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");
}