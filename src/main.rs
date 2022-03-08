use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("+++++++++++++++++++++++++++++++++++++++++++++++++");
    println!("[~Welcome to my rust guessing game!!~]");
    println!("[~Guess the correct number to win some cookies!~]");
    println!("+++++++++++++++++++++++++++++++++++++++++++++++++");
    println!("");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("");
        println!("[You guessed: {}!]", guess);
        println!("");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!(":( --------------* TOO SMALL *------------------ :(");
                println!(":( Sorry, the number you guessed was too SMALL!! :(");
                println!(":( --------------* TOO SMALL *------------------ :(");
                println!("[!!] Try again [!!]");
                println!("")
            }
            Ordering::Greater => {
                println!(":( --------------* TOO BIG *-------------------- :(");
                println!(":( Sorry, the number you guessed was too BIG!!-- :(");
                println!(":( --------------* TOO BIG *-------------------- :(");
                println!("[!!] Please try again [!!]");
                println!("")
            }
            Ordering::Equal => {
                println!(":) --------------* Gives you cookies *-----------------  :) ");
                println!(":) Ayyeee way to go!! You win, enjoy your cookies!       :) ");
                println!(":) --------------* Eats cookies slowly *---------------  :) ");
                break;
            }
        }
    }
}
