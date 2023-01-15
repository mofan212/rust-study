fn main() {
    test_str();
}

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


}