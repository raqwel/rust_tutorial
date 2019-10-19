extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");          // 数を当ててごらん

    let secret_num = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is {}", secret_num);

    loop {

        let mut guess = String::new();

        println!("Please input your guess.");   // ほら、予想を入力してね

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");     // 行の読み込みに失敗しました

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);     // 次のように予想しました: {}

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("bingo!");
                break;
            },
            Ordering::Greater => println!("Too big")
        }
    }
}
