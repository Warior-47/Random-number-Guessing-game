use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("the secret number is : {secret_number}");
    let mut count = 0;
    let mut average = 0;
    let mut average_win = 0;
    loop {
        //println!("Please input your guess number.");
        let mut guess = String::new();
        // io::stdin()
        //     .read_line(&mut guess)
        //     .expect("Failed to read line");
        // // let guess: u32 = match guess.trim().parse() {
        //     Ok(num) => num,
        //     Err(_) => continue,
        // };

        let guess: u32 = rand::thread_rng().gen_range(1..=100);
        count+=1;
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is too small!"),
            Ordering::Greater => println!("Your guess is too big!"),
            Ordering::Equal =>{ //println!("Great you guessed it right! : {average_win}");
            average_win=average_win+1;
            average=average+count;
            count= 0;
                if average_win == 10000 { // Change this and below number to increase the average win
                    average = average/10000;// Change this and above number to increase the average win
                    println!("The Average attempts it took to win was : {average}");
                    break;
                }
            }
        }
    }
}
