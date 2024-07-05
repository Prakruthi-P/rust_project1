
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
use::std::io;
extern  crate rand;
use rand::random;

fn get_guess()->u8{
    loop{
        println!(" Input guess");
        let mut guess= String::new();
        io::stdin().read_line(&mut guess).expect("");
    }
}
fn main() {
    println!("Hello, world!");
}
