use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    // Standard Rust way to read line-by-line from a parent process pipe
    for line in stdin.lock().lines() {
        let input = line.expect("Failed to read from pipe");

        // Echo back a confirmation message
        writeln!(stdout, "Timer received: {}", input).expect("Failed to write to pipe");

        // CRITICAL: Flush so the main process gets the data immediately
        stdout.flush().expect("Failed to flush pipe");
    }
}
/*fn main() {
    let stdin = io::stdin();



    for _ in 0..100000 {
        println!("big guy");
    }
}*/
