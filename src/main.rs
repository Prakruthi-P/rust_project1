
/*
cargo build -Compiles the current project.

cargo check -Analyzes the current project and report errors, but don't build object files.

cargo run -Builds and executes src/main.rs.
	
cargo clean -Removes the target directory.

cargo update -Updates dependencies listed in Cargo.lock.
	
cargo new -Creates a new cargo project.

Create a binary crate
cargo new project_name --bin

Create a library crate
cargo new project_name --lib
*/
use std::io;
extern crate rand; 
//importing external crate
use rand::random;
fn get_guess() -> u8 {
   loop {
      println!("Input guess") ;
      let mut guess = String::new();
      io::stdin().read_line(&mut guess)
         .expect("could not read from stdin");
      match guess.trim().parse::<u8>(){ //remember to trim input to avoid enter spaces
         Ok(v) => return v,
         Err(e) => println!("could not understand input {}",e)
      }
   }
}
fn handle_guess(guess:u8,correct:u8)-> bool {
   if guess < correct {
      println!("Too low");
      false

   } else if guess> correct {
      println!("Too high");
      false
   } else {
      println!("You go it ..");
      true
   }
}
fn main() {
   println!("Welcome to no guessing game");

   let correct:u8 = random();
   //println!("correct value is {}",correct);
   loop {
      let guess = get_guess();
      if handle_guess(guess,correct){
         break;
      }
   }
}