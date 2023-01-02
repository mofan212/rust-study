fn tuple() {
    let tup = (500, 6.4, 1);
    // destructuring 解构
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    // 使用 . 加索引访问元组中的信息
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("The values are: {}, {}, {}", five_hundred, six_point_four, one);
}

fn array() {
    // 声明数组中的数据类型和元素数量
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The first value of array is: {}", a[0]);
    println!("The second value of array is: {}", a[1]);
    // 数组 a 中有 5 个元素，每个元素都是 3
    let a = [3; 5];
}