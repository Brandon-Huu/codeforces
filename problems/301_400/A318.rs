/*
Problem Name: A. Even Odds
Url: https://codeforces.com/problemset/problem/318/A
Completed: November 2, 2023
*/

#[derive(Default)]
struct Scanner {


    buffer: Vec<String>
}


impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T{
        loop{

            if let Some(token) = self.buffer.pop(){
                return token.parse().ok().expect("Failed to pase");
            }
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }

}


fn main() {
    let mut scanner = Scanner::default();

    let numbers:usize = scanner.next();
    let selection:usize = scanner.next();
    let odd_count:usize = numbers / 2 + (numbers % 2);

    let result:usize = (selection <= odd_count) as usize * (selection * 2 - 1) 
        + (selection > odd_count) as usize * (selection-odd_count) * 2;

    println!("{}", result);
}
