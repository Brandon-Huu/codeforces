/*
Name: Strings of Power
Url: https://codeforces.com/problemset/problem/318/B
Completed: November2, 2023

Initially I wanted a trie implementation I could incrementally advance down so i wont have to waste operations.

However,
I already had a solution that worked.
and I'm on a time limit so i submitted what I knew what worked and not what was best
*/

use std::collections::HashMap;

#[derive(Default, Debug)]
struct TrieNode {
    is_end_of_word: bool,
    children: HashMap<char, TrieNode>,
}

#[derive(Default, Debug)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }
    pub fn from(words: &[&str]) -> Self {
        let mut word_bank = Trie {
            root: TrieNode::default(),
        };
        words.iter().for_each(|word| word_bank.insert(word));
        word_bank
    }

    pub fn insert(&mut self, word: &str) {
        word.chars()
            .fold(&mut self.root, |current_node, letter| {
                current_node.children.entry(letter).or_default()
            })
            .is_end_of_word = true;
    }

    pub fn contains(&self, word: &str) -> Option<bool> {
        match word.chars().try_fold(&self.root, |current_node, letter| {
            current_node.children.get(&letter)
        }) {
            None => None,
            Some(word) => Some(word.is_end_of_word),
        }
    }
}

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

    let word: String = scanner.next();

    let mut metals: usize = 0;
    let mut powerful_substrings: usize = 0;
    let mut word_bank: Trie = Trie::from(&["heavy", "metal"]);

    let mut current_word = String::new();

    for letter in word.chars() {
        current_word.push(letter);

        match (
            word_bank.contains(&current_word.as_str()),
            current_word.as_str(),
        ) {
            (Some(true), "heavy") => {
                metals += 1;
                current_word = String::new();
            }
            (Some(true), "metal") => {
                powerful_substrings += metals;
                current_word = String::new();
            },
            (None, _) => current_word = letter.to_string(),
            _ => (),
        }
    }

    println!("{}", powerful_substrings);
}
