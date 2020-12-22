use std::str;
use std::io::{Read, Write, stdin, stdout};
use termios::{Termios, tcsetattr};
use termios::{BRKINT, CS8, ECHO, ICANON, ICRNL, IEXTEN, INPCK, ISIG, ISTRIP, IXON, OPOST, TCSAFLUSH};
use libc::{STDIN_FILENO, VMIN, VTIME};

fn disable_raw_mode(termios_main: Termios) {
    tcsetattr(STDIN_FILENO, TCSAFLUSH, &termios_main);
}

fn enable_raw_mode(termios_main: Termios) {
    let mut termios = termios_main.clone();
    
    termios.c_lflag &= !(BRKINT | ICRNL | INPCK | ISTRIP | IXON);
    termios.c_lflag &= !(OPOST);
    termios.c_lflag |= CS8;
    termios.c_lflag &= !(ECHO | ICANON | IEXTEN | ISIG);
    termios.c_cc[VMIN] = 0;
    termios.c_cc[VTIME] = 1;

    tcsetattr(STDIN_FILENO, TCSAFLUSH, &termios);
}

fn read_input() {
    let mut buffer = [0; 1];

    println!("Starting to read");
    loop {
        if stdin().read_exact(&mut buffer).is_ok() {
            let input = match str::from_utf8(& buffer) {
                Ok(string) => string,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e)
            };

            print!("{}", input);
            stdout().lock().flush().is_ok();
            
            if input == "q" {
                break;
            }
        }
    }
    println!("\nEnding to read");
}

fn main() {
    let termios_main: Termios = Termios::from_fd(STDIN_FILENO).unwrap();

    enable_raw_mode(termios_main);
    read_input();

    disable_raw_mode(termios_main);
    
    println!("Hello, world!");
}