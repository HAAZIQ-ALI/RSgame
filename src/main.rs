use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess The Number!");

    let secret_num = rand::thread_rng().gen_range(1,101);
    //println!("The secret number is {} !",secret_num) ;

    loop { println!("Please type your guess :");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
       .expect("failed to read input Stohpid");

    let guess : u32 = guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    }
        .expect("Please type numbers!");
    println!("You guessed {}",guess);

    match guess.cmp (&secret_num){
        Ordering::Less => println!("Too small like u"),
        Ordering::Greater => println!("Too big like ur mouth "),
        Ordering::Equal => {
            
            println!("You Win!!");
            break;
        }
    }
}
}
