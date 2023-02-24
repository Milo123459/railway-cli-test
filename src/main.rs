use std::fs;
use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let test_to_run = &args[1];
    if fs::metadata(format!("./tests/{test_to_run}.test")).is_err() {
        panic!("test doesn't exist")
    }
    let test = fs::read_to_string(format!("./tests/{test_to_run}.test")).unwrap();
    let args = test.split(' ').collect::<Vec<&str>>();
    let v2cmd = Command::new("railway").args(args.clone()).output().unwrap();
    let v3cmd = Command::new("rlwy").args(args.clone()).output().unwrap();
    let v2 = String::from_utf8_lossy(&v2cmd.stdout);
    let v3 = String::from_utf8_lossy(&v3cmd.stdout);
    let v2error = String::from_utf8_lossy(&v2cmd.stderr);
    let v3error = String::from_utf8_lossy(&v3cmd.stderr);
    let v2status = v2cmd.status.code().unwrap();
    let v3status = v3cmd.status.code().unwrap();

    println!("==================================");
    println!("V2");
    println!("EXIT CODE: {v2status}");
    println!("STDOUT");
    println!("{v2}");
    println!("STDERR");
    println!("{v2error}");
    println!("==================================");
    println!("V3");
    println!("EXIT CODE: {v3status}");
    println!("STDOUT");
    println!("{v3}");
    println!("STDERR");
    println!("{v3error}");
}
