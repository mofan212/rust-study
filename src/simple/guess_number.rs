use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn guess_number() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 假设用户输入了 5，需要先 trim() 下，因为用户输入的内容可能像 5\n 或者 5\r\n
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // 匹配所有 Err 值
            Err(_) => continue,
        };

        // println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // 数字猜对就退出
                break;
            }
        }
    }
}
