extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1, 101);
    

    loop{
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess)
            .expect("입력 실패");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("입력값은 : {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("너무 작군요!"),
            Ordering::Greater => println!("너무 크군요!"),
            Ordering::Equal   => {
                println!("맞추었다 승리!");
                break;
            }
        }
    }
}
