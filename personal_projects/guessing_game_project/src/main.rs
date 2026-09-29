use std::io;
fn main() {
   let secret:u16 = 212;
   let mut attempts = 0;

   println!("The secret number has been chosen, please enter your guess.");
   println!("Hint: The secret number is unsigned.....if yk yk");

  loop { // allows you to keep guessing
     let mut guess = String::new();
   io::stdin().read_line(&mut guess).expect("Not a valid string");
   let guess:u16 = guess.trim().parse().expect("Not a valid input");

   attempts += 1; // adds to number of attempts after each guess

   if guess == secret {
    println!("Congratulations, you won. The secret number was {}!, you got it in {} tries", guess, attempts);

    break; // stops the loop when guess is right
   } else if guess > secret {
    println!("Oops! your guess is too high, try again"); // tells you if you are too high or low
    println!("You have guessed {} times", attempts ); // shows no of attempts
   } else {
    println!("Oops! your guess is too low, try again");
    println!("You have guessed {} times", attempts );
   }
}

  }