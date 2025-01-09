use std::io::{self};



mod classic;
//mod traditional;
mod plot;
mod traditional;
mod traditional_old;
fn main() {

    let phase = 0;
    if phase == 0 {
        println!("WELCOME TO MMMRNG!\n Enter either \"classic\", \"batch\", or \"traditional\" to begin.");
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("ERROR");
            let input = input.trim().to_lowercase();

            if input == "classic" {
                classic::handle::file_write();
                break;
            } else if input == "traditional" {
                traditional::tradhandle::file_write();
                break;
            } else if input == "batch" {
                traditional::tradhandle::file_write();
                break;
            } else if input == "plot" {
                plot::print();
                break;
            } else {
                println!("Please enter either \"classic\", \"batch\" or \"traditional\".");
            }
        }
    }
}
