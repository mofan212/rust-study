fn test_reference() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);
    println!("s is {}", s);

    let mut ss = String::from("world");

    {
        // 使用大括号创建一个新的作用域，以允许拥有多个可变引用，但不能同时拥有
        let r1 = &mut ss;
    }
    let r2 = &mut ss;
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
