extern crate chinese_dictionary;
use chinese_dictionary::{query, WordEntry};

use std::io;
use std::fmt;

// newtype wrapper for WordEntry
struct MyWordEntry(WordEntry);

impl fmt::Display for MyWordEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let word_entry = &self.0;
        write!(
            f, "Simplified: {}\n
                Pinyin Marks: {}\n
                English: {:?}\n
                Measure Words: {:?}\n
                HSK: {}",

                // word_entry.traditional,
                word_entry.simplified,
                word_entry.pinyin_marks,
                // word_entry.pinyin_numbers,
                word_entry.english,
                // word_entry.tone_marks,
                // word_entry.hash,
                word_entry.measure_words,
                word_entry.hsk,
                // word_entry.word_id
        )
    }
}

fn main() {
    println!("Welcome to chinese Dictdoy!\nEnter some english word to look up to ...");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    let word = input.trim();
    let results = query(word).unwrap();

    println!("results");

    for result in results {
        println!("{}", MyWordEntry(result.clone()));
    }
}

