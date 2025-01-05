use std::{io, thread, time};
use std::io::{stdout, Write};

mod parse;
use parse::get_time;
// use crate::parse::*;

// User enters time as argument, defaults to 25:00 if no argument entered
// User may enter HH:MM:SS or MM:SS
// DONE - Parse HH:MM:SS to (u8, u8, u8)
// DONE - parse MM:SS to (u8, u8)
// Every second, SS is decremented
    // if SS is 00, MM -1, SS = 59, etc.
// once HH, MM, SS are all 00, create flashing "times up!" ascii art

const DEFAULT: &str = "25:00";
const INPUT_MESSAGE: &str = "Press enter for 25 minutes or enter a time in HH:MM:SS format. Press [Q] to exit.";

fn main() {
    
    let mut stdout = stdout();
    let mut counter = 0;
    
    println!("{}", INPUT_MESSAGE);

    let (mut hr, mut min, mut sec) = (0, 0, 0);
    
    loop {
        let mut time = String::new();

        if let Err(e) = io::stdin().read_line(&mut time) {
            println!("Invalid input: {e}\r\n{INPUT_MESSAGE}");
        }

        if &time.trim().to_lowercase() == "q" {
            return;
        }
    
        match get_time(time) {
            Ok((hrx, minx, secx)) => {
                hr = hrx;
                min = minx;
                sec = secx;
                
                break;
            },
            Err(e) => println!("{e}\r\n{INPUT_MESSAGE}")
        }
    }


    // print!("\x1B[2J\x1B[1;1H"); // clears console, takes cursor to first row first column

    loop {
        print!("{hr:0>2}:{min:0>2}:{sec:0>2}\r");
        stdout.flush().expect("fluuuuuush"); // TODO
        thread::sleep(time::Duration::from_secs(1));
        if hr == 0 && min == 0 && sec == 0 {
            // times up
            todo!();
        }

        if sec == 0 {
            if min == 0 {
                hr -= 1;
                min = 59;
                sec = 60;
            } else {
                min -= 1;
                sec = 60;
            }
        }

        sec -= 1;
    }

}

