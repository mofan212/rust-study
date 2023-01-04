
fn create_structure() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("some_user_name_123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{}", user1.username);
    // 修改可变的结构体字段值
    user1.email = String::from("anotheremail@example.com");
    println!("{}", user1.email);

    let user2 = build_user(String::from("xxx@example.com"), String::from("test"));
    println!("{}", user2.active);

    // 结构体更新语法
    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("email of user3 is {}", user3.email);
    println!("username of user3 is {}", user3.username);

    // 元组结构体
    let black = Color(0, 0, 0);
    println!("{}, {}, {}", black.0, black.1, black.2);
    let origin = Point(0, 0, 0);
    println!("{}, {}, {}", origin.0, origin.1, origin.2);

    // 类单元结构体
    let subject = AlwaysEquals;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEquals;