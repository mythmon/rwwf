extern crate rwwf;
extern crate docopt;
extern crate rustc_serialize;
extern crate permutohedron;

use std::collections::HashSet;
use docopt::Docopt;
use rwwf::{Trie, powerset};

const USAGE: &'static str = "
Usage: anagrams <letters>
";

const WORD_LIST: &'static str = include_str!("../../word_list");

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_letters: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                     .and_then(|d| d.parse())
                     .and_then(|d| d.decode())
                     .unwrap_or_else(|e| e.exit());

    let mut lexicon = Trie::new();

    for line in WORD_LIST.split("\n") {
        if line != "" {
            lexicon.insert(line);
        }
    }

    let letters: Vec<char> = args.arg_letters.chars().collect();
    let mut words_found = HashSet::new();

    for mut subset in powerset(&letters[..]) {
        let heap = permutohedron::Heap::new(&mut subset);

        for permutation in heap {
            let word: String = permutation.into_iter().collect();
            if lexicon.contains(&word[..]) {
                words_found.insert(word);
            }
        }
    }

    if words_found.len() > 0 {
        for (idx, word) in words_found.iter().enumerate() {
            println!("{:>2}. {}", idx + 1, word);

        }
    } else {
        println!("No words found");
    }
}
