fn test_str() {
    // 新建字符串
    let mut s = String::new();
    // 根据字符串字面量创建
    let s = "initial contents".to_string();
    // 或者
    let s = String::from("initial contents");

    // 使用 push_str 和 push 附加字符串
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    // 没有获取 s2 的所有权，因此后续还能继续使用 s2
    println!("s2 is {s2}");
    // push 方法需要一个单独的字符作为参数
    let mut s = String::from("lo");
    s.push('l');
    println!("s is {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // s1 被移动了，不能继续使用
    let s3 = s1 + &s2;
    println!("s3 is {}", s3);
    // 下面这句会编译报错
    // println!("{}", s1);

    // 使用 format! 宏
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{} - {} - {}", s1, s2, s3);
    println!("{}", s);
    // 下面这句并不会编译报错
    println!("s1 is {}", s1);

    // 遍历字符串
    for c in "hello, world".chars() {
        print!("{} ", c);
    }

}