/*
Not complete.
Have not even tried to submit this file.
Can't run single rust files so uuuuuuuuuuuuuuuuuh
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
        Trie { root: TrieNode::default() }
    }
    pub fn from(words: &[&str]) -> self {
        let mut word_bank: Trie { root:TrieNode::default() };
        words.iter().for_each(word_bank.insert);
    }

    pub fn insert(&mut self, word: &str) {
        word.chars()
            .fold(&mut self.root, |current_node, letter| current_node.children.entry(c).or_default());
    }

    pub fn contains(&self, word: &str) -> Option<bool> {

        match words.chars()
            .fold(&self.root, |current_node, letter| current_node.children.get(&letter)) 
            {
                None => None,
                Some(word) => Some(word.is_end_of_word)
            }
    }       
 
}
-+  
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

    let word:String = scanner.next();

    let mut metals:usize = 0;
    let mut powerful_substrings:usize = 0;
    let mut current_word = String::new();
    let mut word_bank: Trie = Trie::new();
    word_bank
    for letter in word.chars(){
        match 
    }
    println!("{}",powerful_substrings);
}
