use std::io::{stdin, Read};
use std::vec;

fn main() {
    let mut buffer = String::new();
    let res = stdin().read_line(&mut buffer);

    let t = buffer.trim().parse::<i32>().unwrap();
    for _ in 0..t {
        let mut line = String::new();
        let r = stdin().read_line(&mut line);
        let items: Vec<&str> = line.split_whitespace().collect();

        let a = items[0].parse::<i32>().unwrap();
        let b = items[1].parse::<i32>().unwrap();

        println!("{}", a % b);
    }
}
