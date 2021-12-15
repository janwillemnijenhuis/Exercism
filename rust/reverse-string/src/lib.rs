use unicode_reverse::reverse_grapheme_clusters_in_place;
extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;
pub fn reverse(input: &str) -> String {
    let word: String = input
        .graphemes(true)
        .rev()
        .collect();

    return word;
}

fn main() {
    println!("{}", reverse("cool"));
}
