use std::io;
use std::process::Command;

// Main
fn main() {
    menu();
}

// Scenes
fn menu() {
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

        io::stdin().read_line(&mut data).expect("Error to read line");
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

fn standard_mode() {
    let mut data = String::new();

    clear();
    io::stdin().read_line(&mut data).expect("Error to read line");
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
