use std::{time, thread};
use std::io::{stdout, Write};

// User enters time as argument, defaults to 25:00 if no argument entered
// User may enter HH:MM:SS or MM:SS
// - Parse HH:MM:SS to (u8, u8, u8)
// - parse MM:SS to (u8, u8)
// Every second, SS is decremented
    // if SS is 00, MM -1, SS = 59, etc.
// once HH, MM, SS are all 00, create flashing "times up!" ascii art

fn main() {
    let mut stdout = stdout();
    let mut counter = 0;
    
    loop {
        thread::sleep(time::Duration::from_secs(1));
        counter += 1;
        print!("{}\r", counter);
        
        stdout.flush().unwrap();
    }
}

// print!("\x1B[2J\x1B[1;1H"); // clears console, takes cursor to first row first column