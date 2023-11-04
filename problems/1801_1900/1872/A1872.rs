/*
 * Name: Two vessels 
 * URL: https://codeforces.com/contest/1872/problem/A 
 * Completed: November 4, 2023
 *
 *
 * I hate doing this
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

const SIZE: usize = 8;
const RED_ROW: &str = "RRRRRRRR";

fn main(){
    let mut scanner = Scanner::default();
    
    let test_cases: usize = scanner.next();

    (0..test_cases)
        .map(|_| (scanner.next::<usize>(), scanner.next::<usize>(), scanner.next::<usize>() ))
        .map(|(a,b,c)| (a.max(b),a.min(b),c) )
        .map(|(a,b,c)| (a-b,2 * c) )
        .map(|(a,c)| a / c + (a % c > 0) as usize) 
        .for_each(|result| println!("{}", result));

}
