use std::io::{stdin, Read};
use std::vec;


fn main() {
    let mut buffer = String::new();

    // `read_line` returns `Result` of bytes read
    let res = stdin().read_line(&mut buffer);
    let items: Vec<&str> = buffer.split_whitespace().collect();
    let n = items[0].parse::<i32>().unwrap();
    let k = items[1].parse::<i32>().unwrap();

    let mut total_divisible: i32 = 0;
    for _ in 0..n {
        let mut input = String::new();
        let r = stdin().read_line(&mut input);
        let x = input.trim().parse::<i32>().unwrap();

        if x % k == 0 {
            total_divisible = total_divisible + 1;
        }
    }

    println!("{}", total_divisible);
}
