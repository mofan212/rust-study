
fn test_hashmap() {
    // 创建 HashMap，并插入一些值
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 获取 HashMap 中的值
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("the score is {}", score);

    // 使用循环遍历 HashMap 中的每一项值
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // HashMap 的所有权
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 下面代码会编译报错
    // println!("field_name is {}", field_name);

    // 覆盖一个值
    scores.insert(String::from("Blue"), 20);
    scores.insert(String::from("Blue"), 40);
    println!("{:?}", scores); // {"Blue": 40, "Yellow": 50}

    // 只在键没有对应的值时才插入
    let blue = scores.entry(String::from("Blue")).or_insert(50);
    println!("blue is {}", blue); // 40
    scores.entry(String::from("Red")).or_insert(100);
    println!("{:?}", scores); // {"Blue": 40, "Yellow": 50, "Red": 100}

    // 根据旧值更新值
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map); // {"hello": 1, "world": 2, "wonderful": 1}
}