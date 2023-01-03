fn test_first_world() {
    let my_string = String::from("hello world");

    let world = first_word(&my_string[0..6]);
    println!("{}", world);
    let world = first_word(&my_string[..]);
    println!("{}", world);
    let world = first_word(&my_string);
    println!("{}", world);

    let my_string_literal = "hello world";
    let world = first_word(&my_string_literal[0..6]);
    println!("{}", world);
    let world = first_word(&my_string_literal[..]);
    println!("{}", world);
    let world = first_word(my_string_literal);
    println!("{}", world);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn slice() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{}, {}", hello, world);

    let slice = &s[0..2];
    println!("{slice}");
    let slice = &s[..2];
    println!("{}", slice);

    let len = s.len();
    let slice = &s[3..len];
    println!("{}", slice);
    let slice = &s[3..];
    println!("{}", slice);

    let slice = &s[0..len];
    println!("{}", slice);
    let slice = &s[..];
    println!("{}", slice);
}

fn array_slice() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}