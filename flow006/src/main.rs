use std::io::{stdin, Read};
use std::vec;

fn main() {
    let mut buffer = String::new();
    let res = stdin().read_line(&mut buffer);

    let t = buffer.trim().parse::<i32>().unwrap();
    for _ in 0..t {
        let mut line = String::new();
        let r = stdin().read_line(&mut line);
        let mut n = line.trim().parse::<i32>().unwrap();

        let mut digit_sum: i32 = 0;
        while n > 0 {
            digit_sum += n % 10;
            n /= 10;
        }

        println!("{}", digit_sum);
    }
}
