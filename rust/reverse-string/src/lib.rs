use unicode_reverse::reverse_grapheme_clusters_in_place;
extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;
pub fn reverse(input: &str) -> String {
    // let strlen = input.len();
    // let mut reversed_string: Vec<char> = Vec::new();
    // for c in input.chars().rev() {
    //     reversed_string.push(c);
    // // }
    // return reversed_string.iter().collect();
    let word: String = input
        .graphemes(true)
        .rev()
        .collect();

    return word;
}

fn main() {
    println!("{}", reverse("cool"));
}
