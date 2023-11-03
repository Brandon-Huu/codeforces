/*
 * Name: Increasing 
 * URL: https://codeforces.com/contest/1742/problem/B 
 * Completed: November 3rd, 2023
 *
 * Stop saying that word...
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
        .map(|_| {
            let array_size:usize = scanner.next();

            let unique_values: std::collections::HashSet<usize> = (0..array_size)
                .map(|_| scanner.next::<usize>() )
                .collect();

            match array_size == unique_values.len() {
                true => "YES",
                _ => "NO"
            }
        })
        .for_each(|result| println!("{}", result) );

}
