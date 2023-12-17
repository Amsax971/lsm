use std::process;

pub fn comp(int: i32, e: i32) {

        if int == 21 {
            println!("gg");
            process::exit(0x0100);
        } else if int < 21 {
            println!("more");
        } else if int > 21 {
            println!("less");
        }
}