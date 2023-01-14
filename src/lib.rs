mod module;

// 重导出
pub use crate::module::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::module::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    module::front_of_house::hosting::add_to_waitlist();

    // 在夏天订购一个黑麦土司作为早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 改变主意更换想要面包的类型
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 如果取消下一行的注释代码不能编译；
    // 不允许查看或修改早餐附带的季节水果
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // 使用 use
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // 需要提供一个公共的关联函数来构造 Breakfast
        // 因为 seasonal_fruit 字段是私有的，不能直接构造 Breakfast 实例
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // 如果将枚举设置成共有，那么它所有成员都是共有的
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

use std::collections::HashMap;

fn use_hashMap() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// 使用 as 关键字提供新的名称
use std::fmt::Result;
use std::io::Result as IoResult;
