fn string_test () {
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s);

    // 克隆
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}


fn function() {
    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn return_ownership() {
    let s1 = gives_ownership();
    println!("{s1}");

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);
    println!("{s3}");

    let s4 = String::from("hello");
    let (s5, len) = calculate_length(s4);
    println!("The length of '{}' is {}", s5, len);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}