const MAX_POINT: u32 = 100_000;

fn variables() {

    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    println!("The Const value of MAX_POINT is {}", MAX_POINT);

    let space = "    ";
    let space = space.len();
    println!("The length of space is {}", space);

    let guess: u32 = "42".parse().expect("Not a Number");
    println!("The guess is {}", guess);

    let var_f64 = 2.0;
    let var_f32: f32 = 3.0;
    println!("The value of var_f64 is {}", var_f64);
    println!("The value of var_f32 is {}", var_f32);

    let boolean = true;
    println!("The value of boolean is {}", boolean);
    let boolean: bool = false;
    println!("The value of boolean is {}", boolean);

    let var_char = 'z';
    println!("The value of var_char is {}", var_char);
    let var_char: char = 'z';
    println!("The value of var_char is {}", var_char);
    let var_char = '🧐';
    println!("The value of var_char is {}", var_char);

    // tuple 
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{}, {}, {}", tup.0, tup.1, tup.2);
    let (x, y, z) = tup;
    println!("{}, {}, {}", x, y, z);

    // array
    let array = ["Tom", "Jerry"];
    println!("Tom cat is named {}", array[0]);
    let array = [3; 5]; // 相当于 [3, 3, 3, 3, 3]
    println!("{}", array[2]);
}
