use std::io::{stdin, Read};
use std::vec;

fn reverse(str: String) -> String {
    let mut reversed = String::new();

    let mut i = str.len() - 1;
    while i >= 0 {
        reversed.push(str.chars().nth(i).unwrap());
        i = i - 1;
    }

    return reversed;
}

fn main() {
    let mut buffer = String::new();
    let res = stdin().read_line(&mut buffer);

    let t = buffer.trim().parse::<i32>().unwrap();
    for _ in 0..t {
        let mut line = String::new();
        let r = stdin().read_line(&mut line);

        println!("{}", reverse(line.trim()));
    }
}
