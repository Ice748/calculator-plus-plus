mod utilities;
use std::io;

pub fn menu() {
    const BTNS: &[&str] = &["Standard mode", "About", "Exit"];
    let mut btn: usize = 0;
    let mut data = String::new();

    loop {
        data.clear();

        utilities::clear();
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
                    utilities::clear();
                    std::process::exit(0);
                }

                _ => {}
            }
        } else {
            btn = (btn + 1) % BTNS.len();
        }
    }
}

pub fn standard_mode() {
    let mut data = String::new();

    utilities::clear();
    io::stdin().read_line(&mut data).expect("Error to read line");
}

pub fn about() {
    let mut data = String::new();

    utilities::clear();
    io::stdin().read_line(&mut data).expect("Error to read line");
}
