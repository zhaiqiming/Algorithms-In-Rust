
// use algorithms_in_rust::tree::*;
use std::io;
fn main() {
    println!("IN");
    loop {
        let mut line = String::new();
        if 0 == io::stdin().read_line(&mut line).unwrap() {
            break;
        }
        let v: Vec<&str> = line.split("(").collect();
        if v.len() > 1 {
            let v: Vec<&str> = v[1].split(" ").collect();
            println!("{}", v[0]);
        }
    }
}
