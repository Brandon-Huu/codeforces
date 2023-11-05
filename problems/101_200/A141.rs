/*
Problem Name: Amusing Joke
Url: https://codeforces.com/problemset/problem/141/A
Completed: November 5, 2023
*/
#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}

impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed to pase");
            }
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

fn main() {
    let mut scanner = Scanner::default();

    let mut letters: Vec<char> = scanner
        .next::<String>()
        .chars()
        .chain(scanner.next::<String>().chars())
        .collect();

    let mut other: Vec<char> = scanner.next::<String>().chars().collect();

    letters.sort();
    other.sort();

    let result: &str = match letters == other {
        true => "YES",
        _ => "NO",
    };

    println!("{}", result);
}
