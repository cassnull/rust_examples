use std::process::Command;

fn main() {
    let mut child = if cfg!(target_os = "windows") {
        Command::new("timeout").arg("5").spawn().unwrap()
    } else {
        Command::new("sleep").arg("5").spawn().unwrap()
    };
    let _result = child.wait().unwrap();

    println!("reached end of main");
}
