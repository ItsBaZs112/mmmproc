use std::io::{self};

mod classic;
mod traditional;
fn main() {
    let phase = 0;
    if phase == 0 {
        println!("WELCOME TO MMMRNG!\n Enter either \"Classic\", \"batch\", or \"Traditional\" to begin.");
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("ERROR");
            let input = input.trim().to_lowercase();

            if input == "classic" {
                classic::handle::file_write();
                break;
            } else if input == "traditional" {
                traditional::tradhandle::file_write(false);
                break;
            } else if input == "batch" {
                traditional::tradhandle::file_write(true);
                break;
            } else {
                println!("Invalid input. Please enter either \"Classic\" or \"Traditional\".");
            }
        }
    }
}
