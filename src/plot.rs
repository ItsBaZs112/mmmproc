use std::io::{self, Write};

pub fn print() {
     let mut f = "0";
        for i in 0..16 {

            for j in 0..14 {
                if i == 14 && j == 10 {
                    f = "1";
                }
                else {
                    f = "0";
                }
                print!("{}\n",f.repeat(i));
            }
            io::stdout().flush();
            print!("\x1B[14A");
            
        }
}   