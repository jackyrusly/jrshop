use std::io::{self, Read, Write};

pub fn clear() {
    for _ in 1..25 {
        println!();
    }
}

pub fn pause() {
    print!("\nPress enter to continue...");
    io::stdout().flush().expect("Could not flush stdout");
    let _ = io::stdin().read(&mut [0u8]);
}

pub fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}
