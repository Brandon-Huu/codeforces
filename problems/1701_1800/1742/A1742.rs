/*
 * Name: Sum
 * URL: https://codeforces.com/problemset/problem/1742/A
 * Completed: November 3rd, 2023
 *
 * Dude...
 */

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>
}

impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop{
            if let Some(value) = self.buffer.pop(){
                return value.parse().ok().unwrap();
            }
            let mut input: String = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}
fn main(){
    let mut scanner = Scanner::default();
    
    let test_cases:usize = scanner.next();
    
     (0..test_cases)
        .map(|_| [scanner.next::<usize>(), scanner.next::<usize>(), scanner.next::<usize>()] )
        .map(|mut test| {
            test.sort();
            match test[2] == test[1] + test[0] {
                true => "YES",
                _ => "NO"
            }
        })
        .for_each(|result| println!("{}", result) );

}
