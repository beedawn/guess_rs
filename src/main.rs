use std::io;

use rand::Rng;

fn main() {
    println!("Hello, world!");
    let mut rng = rand::thread_rng();
    let mut x: i32 = rng.gen_range(0..=10);
 
    loop {
        println!("Enter a number");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to readline");


        let my_num: i32 = input.trim().parse()
            .expect("please give me correct string number!");
        if my_num == x {
            println!("You guessed right!");
            x = rng.gen_range(0..=10);
        } else {
            println!("You lose");
        }
        println!("Play again? (Y/N)");
        let mut play_again = String::new();
        io::stdin().read_line(&mut play_again).expect("failed to read play again");
        let parse_play_again = play_again.trim();
        if parse_play_again == "N" || parse_play_again == "n" {
            break;
        }
    }

    println!("The number was {}", x);
}
