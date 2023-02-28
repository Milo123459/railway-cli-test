use std::{fs, process::Stdio};
use std::process::Command;
use std::io::Write;
use keycodes::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let test_to_run = &args[1];
    if fs::metadata(format!("./tests/{test_to_run}.test")).is_err() {
        panic!("test doesn't exist")
    }
    let test = fs::read_to_string(format!("./tests/{test_to_run}.test")).unwrap();
    let mut press = test.split('[').map(|thing| thing.to_string()).collect::<Vec<String>>();
    let not_trimmed = press[0].clone();
    let command_to_run = not_trimmed.trim();
    let sequence = press[1..].iter_mut().map(|input| { let mut input = input.trim().to_string(); input.pop(); input.to_owned() }).collect::<Vec<String>>();
    let args = test.split(' ').filter(|p| !(p.contains('[') || p.contains(']'))).collect::<Vec<&str>>();
    let mut v2cmd_child = Command::new("/usr/local/bin/railway").arg(command_to_run).args(args.clone()).stdin(Stdio::piped()).stdout(Stdio::from(fs::File::create("v2out.txt").unwrap())).stderr(Stdio::from(fs::File::create("v2err.txt").unwrap())).spawn().unwrap();
    let mut v3cmd_child = Command::new("rlwy").arg(command_to_run).args(args).stdin(Stdio::piped()).stdout(Stdio::from(fs::File::create("v3out.txt").unwrap())).stderr(Stdio::from(fs::File::create("v3err.txt").unwrap())).spawn().unwrap();
    let mut v2cmd_stdin = v2cmd_child.stdin.take().unwrap();
    let mut v3cmd_stdin = v3cmd_child.stdin.take().unwrap();
    for input in sequence {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        let c_i = input.clone();
        let what_to_do = c_i.split(' ').collect::<Vec<&str>>()[0];
        let input_sa = input.split(' ').collect::<Vec<&str>>()[1..].join(" ");
        dbg!(input_sa.clone());
        dbg!(what_to_do);
        if what_to_do == "press" {
            dbg!(input_sa.clone());
            v2cmd_stdin.write_all(input_sa.clone().as_bytes()).unwrap();
            v3cmd_stdin.write_all(input_sa.as_bytes()).unwrap();
        } else if what_to_do == "write" {
            // oh no
            let to_write = input_sa.split(' ');
            for thing in to_write {
                v2cmd_stdin.write_all(thing.as_bytes()).unwrap();
                v3cmd_stdin.write_all(thing.as_bytes()).unwrap(); 
            }
        }
    }
    let v2status = v2cmd_child.wait().unwrap().code().unwrap();
    let v3status = v3cmd_child.wait().unwrap().code().unwrap();
    let v2 = fs::read_to_string("v2out.txt").unwrap();
    let v3 = fs::read_to_string("v3out.txt").unwrap();
    let v2error = fs::read_to_string("v2err.txt").unwrap();
    let v3error = fs::read_to_string("v3err.txt").unwrap();

    std::fs::remove_file("v2out.txt").unwrap();
    std::fs::remove_file("v3out.txt").unwrap();
    std::fs::remove_file("v2err.txt").unwrap();
    std::fs::remove_file("v3err.txt").unwrap();

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
