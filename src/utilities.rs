use std::process::Command;

pub fn clear() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").args(["/C", "cls"]).status();
    } else {
        let _ = Command::new("clear").status();
    }
}
