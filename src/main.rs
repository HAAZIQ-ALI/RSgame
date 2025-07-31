use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let mut how_many = String::new();
    println!("How many numbers you want to guess ? :");
    io::stdin().read_line(&mut how_many).expect("are you stohpid");
    let num_guess :u8 = how_many.trim().parse().unwrap();

    let mut correct = Vec::new();

    for _i in  0 .. num_guess {
        correct.push(rand::thread_rng().gen_range(1, 101));

    }
     //println!("The secret number is {} !",secret_num) ;

    println!("Guess The Number! (between 1 to 100 )");
    let mut guess_no :i8 = 0 ;
    let mut guesses_made: u8 = 0;

   
  

    while  guesses_made < num_guess{
        println!("Please type your guess :");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
           .expect("failed to read input Stohpid");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        

        match guess.cmp(&correct [guesses_made as usize]){
            Ordering::Less => {
                println!("Too small like u");
                guess_no += 1;
            },
            Ordering::Greater => {
                println!("Too big like ur mouth ");
                guess_no += 1;
            },
            Ordering::Equal => {
                println!("You Win!! You took {} guesses", guess_no + 1);
                guesses_made += 1;
                if guesses_made >= num_guess {
                    println!("Congratulations! You guessed all {} numbers!", num_guess);
                    break;
                }
                guess_no = 0; // Reset guess counter for next number
            }
        }
    }

}
