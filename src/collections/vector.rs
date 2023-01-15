fn test_vec() {
    // 使用 new 函数创建 vector
    let v: Vec<i32> = Vec::new();

    // 使用 vec! 宏创建有初始值的 vector
    let v = vec![1, 2, 3];

    // 创建一个 vector，并更新
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);
    
    // 读取 vector 中的元素
    let third = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element.")
    }

    // 遍历 vector 中的元素
    let v = vec![100, 32, 57];
    for i in &v {
        print!("{} ", i);
    }
    println!();
    // 遍历可变 vector，并修改每一项
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // * 表示解引用运算符
        *i += 50;
        print!("{} ", i);
    }
    println!();

    // 使用枚举存储多种类型的值
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];
    // 获取第二个元素
    match row.get(1) {
       Some(second) => {
            match second {
                SpreadsheetCell::Text(text) => println!("The text is {}", text),
                _ => println!("Not Test")
            }
       },
       None => println!("There is no second element.")
    }
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}