// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut word_count: HashMap<&str, usize> = HashMap::new();
    let mut iter = note.iter();
    let mut next = iter.next();
    while next != None {
        let mut item = next.unwrap();
        if !word_count.contains_key(item) {
            word_count.insert(item, 1);
        } else {
            *word_count.get_mut(item).unwrap() += 1;
        }
        next = iter.next();
    }
    println!("{:?}", word_count);
    for item in word_count {
        let mut word = item.0;
        let mut amt = item.1;
        for i in 0..amt {
            println!("{}", word)
        }
    }
    return true;
}

fn main() {
    let magazine = "two times three is not four".split_whitespace().collect::<Vec<&str>>();
    let note = "two times two is four".split_whitespace().collect::<Vec<&str>>();
    can_construct_note(&magazine, &note);
}
