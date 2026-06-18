use std::io::{self, Write};
use std::process::Command;

const ERROR_001: &str = "ERROR 001: Error to read line";
const ERROR_002: &str = "ERROR 002: 1st number is invalid";
const ERROR_003: &str = "ERROR 003: Action is invalid";
const ERROR_004: &str = "ERROR 004: 2nd number is invalid";
const ERROR_005: &str = "ERROR 005: Zero division";

// Main scene
fn main() {
    const BTNS: &[&str] = &["Standard mode", "About", "Exit"];
    let mut btn: usize = 0;
    let mut data = String::new();

    loop {
        data.clear();

        clear();
        println!("Calculator++\n");

        for i in 0..BTNS.len() {
            let prefix = if i == btn { ">>" } else { ">" };
            println!("{} {}", prefix, BTNS[i]);
        }

        io::stdin().read_line(&mut data).expect(ERROR_001);
        if data.trim().to_lowercase() == "e" {
            match btn {
                0 => standard_mode(),
                1 => about(),
                2 => {
                    clear();
                    std::process::exit(0);
                }
                _ => {}
            }
        } else {
            btn = (btn + 1) % BTNS.len();
        }
    }
}

// Other scenes
fn standard_mode() {
    const ACTIONS: &[char] = &['+', '-', '*', '/'];
    let mut data = String::new();

    loop {
        data.clear();
        clear();
        println!(">>> ");
        print!("Enter the 1st number > ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut data).expect(ERROR_001);
        let num1: f64 = data.trim().parse().expect(ERROR_002);

        data.clear();
        clear();
        println!(">>> {}", num1);
        print!("Enter the action (+, -, *, /) > ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut data).expect(ERROR_001);
        let action: char = data.trim().parse().expect(ERROR_003);

        if !ACTIONS.contains(&action) {
            panic!("{}", ERROR_003);
        }

        data.clear();
        clear();
        println!(">>> {}{}", num1, action);
        print!("Enter the 2nd number > ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut data).expect(ERROR_001);
        let num2: f64 = data.trim().parse().expect(ERROR_004);

        let result: f64 = match action {
            '+' => num1 + num2,
            '-' => num1 - num2,
            '*' => num1 * num2,
            '/' => {
                if num2 != 0.0 {
                    num1 / num2
                } else {
                    panic!("{}", ERROR_005);
                }
            }
            _ => unreachable!(),
        };

        loop {
            data.clear();
            clear();
            println!(">>> {}{}{}={}", num1, action, num2, result);
            print!("Run again? (yes/no) > ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut data).expect(ERROR_001);

            match data.trim().to_lowercase().as_str() {
                "yes" => break,
                "no" => return,
                _ => continue,
            }
        }
    }
}

fn about() {
    let mut data = String::new();

    clear();
    io::stdin().read_line(&mut data).expect("Error to read line");
}

// Utilities
fn clear() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").args(["/C", "cls"]).status();
    } else {
        let _ = Command::new("clear").status();
    }
}
