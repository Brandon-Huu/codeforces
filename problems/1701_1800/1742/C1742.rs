/*
 * Name: Stripes 
 * URL: https://codeforces.com/contest/1742/problem/C
 * Completed: November 3rd, 2023
 *
 * I believe this could be made fater.
 * However, I do not want to spend time working on it.
 * 
 * I could check if a row is equal to RED_ROW in the first map.
 * Idk if it matters due to how iterators work
 *
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
        .map(|_| (0..SIZE).map(|_| scanner.next::<String>() ).collect::<Vec<String>>())
        .map(|array| array.iter().any(|row| row.as_str() == RED_ROW ) )
        .for_each(|result| match result {
            true => println!("{}", "R"),
            _ => println!("{}", "B")
        });

}
