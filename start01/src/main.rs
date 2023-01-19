use std::io::stdin;

fn main() {
    let mut buffer = String::new();

    // `read_line` returns `Result` of bytes read
    let res = stdin().read_line(&mut buffer);
    println!("{}", buffer);
}
